const std = @import("std");

pub const log = std.log.scoped(.q);

pub fn readFile(alloc: std.mem.Allocator, file_name: []const u8) ![]u8 {
    const path_absolute = try std.fs.cwd().realpathAlloc(alloc, file_name);
    defer alloc.free(path_absolute);

    const file = try std.fs.openFileAbsolute(path_absolute, .{ .mode = .read_only });
    const file_stat = try file.stat();
    const file_size: usize = @intCast(file_stat.size);

    const buffer = try alloc.alloc(u8, file_size);
    _ = try file.readAll(buffer);

    return buffer;
}
