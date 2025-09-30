const std = @import("std");
const q = @import("q_lang");

pub const Lexer = struct {
    pub const Token = struct {
        pub const Span = struct { lo: usize = 0, hi: usize = 0 };

        word: []const u8 = "null",
        span: Span = .{},
        kind: enum {
            sof, // start of file
            eof,
            err,
        } = .err,

        pub fn display(self: Token) void {
            _ = self;
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

    pub fn next(_: *Lexer, _: std.mem.Allocator) !void {}
};
