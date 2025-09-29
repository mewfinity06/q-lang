const std = @import("std");
const q = @import("q_lang");

const USAGE =
    \\ Usage: q-lang <file>
    \\
;

pub fn main() !u8 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const alloc = arena.allocator();
    defer arena.deinit();

    // Get args
    const args = try std.process.argsAlloc(alloc);
    defer alloc.free(args);

    if (args.len <= 1) {
        std.debug.print("{s}", .{USAGE});
        return 1;
    }

    // Open and read file
    const path_absolute = try std.fs.cwd().realpathAlloc(alloc, args[1]);
    defer alloc.free(path_absolute);

    const file = try std.fs.openFileAbsolute(path_absolute, .{ .mode = .read_only });
    const file_stat = try file.stat();
    const file_size: usize = @intCast(file_stat.size);

    const buffer = try alloc.alloc(u8, file_size);
    defer alloc.free(buffer);

    // FIXME: We should check if readAll equals file_size but it's not super important now
    _ = try file.readAll(buffer);

    return 0;
}
