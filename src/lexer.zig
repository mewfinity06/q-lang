const std = @import("std");
const q = @import("q_lang");

pub const Lexer = struct {
    pub const Token = struct {
        pub const Span = struct { lo: usize = 0, hi: usize = 0 };

        word: []const u8 = "null",
        span: Span = .{},
        kind: union(enum) {
            sof, // start of file
            eof,
            err: []const u8, // message
        },

        pub fn display(self: Token, log: type) void {
            if (self.kind == .sof)
                log.info("START OF FILE", .{})
            else if (self.kind == .eof)
                log.info("END OF FILE", .{})
            else
                log.info("Found `{s}` @ {}..{}", .{
                    self.word,
                    self.span.lo,
                    self.span.hi,
                });
        }
    };

    source: []u8,
    cur: usize = 0,
    token: Token = .{ .kind = .sof },

    pub fn init(source: []u8) Lexer {
        return .{
            .source = source,
        };
    }

    pub fn next(_: *Lexer, token: *?Token) !void {
        token.* = null;
    }
};
