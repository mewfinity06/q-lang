const std = @import("std");
const q = @import("q_lang");

const log = std.log.scoped(.q);

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

    const path_absolute = try std.fs.cwd().realpathAlloc(alloc, args[1]);
    defer alloc.free(path_absolute);

    const file = try std.fs.openFileAbsolute(path_absolute, .{ .mode = .read_only });
    const file_stat = try file.stat();
    const file_size: usize = @intCast(file_stat.size);

    const buffer = try alloc.alloc(u8, file_size);
    defer alloc.free(buffer);

    // FIXME: We should check if readAll equals file_size but it's not super important now
    _ = try file.readAll(buffer);

    var lexer = Lexer{ .source = buffer };
    var token: ?Token = Token{ .kind = .sof };

    while (token != null) : (try lexer.next(&token)) {
        token.?.display(log);
    }

    return 0;
}
