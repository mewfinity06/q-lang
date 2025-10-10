const std = @import("std");
const q = @import("q_lang");

const log = q.log;

const lexer_lib = @import("lexer.zig");
const Lexer = lexer_lib.Lexer;
const Token = Lexer.Token;
const Span = Token.Span;

const USAGE =
    \\Usage: q-lang <file>
    \\
;

pub fn main() !u8 {
    // NOTE: DebugAllocator.safety is off due to weird bug when freeing `args`
    //       Once this is resolved, turn safety back on
    var arena: std.heap.DebugAllocator(.{ .safety = false }) = .init;
    const alloc = arena.allocator();
    defer _ = arena.deinit();

    // NOTE: See DebugAllocator
    const args = try std.process.argsAlloc(alloc);

    if (args.len <= 1) {
        log.err("{s}", .{USAGE});
        return 1;
    }

    const file_name = args[args.len - 1];

    const file_contents = try q.readFile(alloc, file_name);
    defer alloc.free(file_contents);

    var lexer = Lexer{ .source = file_contents };
    var token: ?Token = Token{ .kind = .sof };

    while (token != null) : (try lexer.next(&token)) {
        token.?.display();
    }

    return 0;
}
