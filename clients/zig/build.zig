const std = @import("std");

pub fn build(b: *std.Build) void {
    const module = b.addModule("evgl_client", .{
        .root_source_file = b.path("src/root.zig"),
        .target = b.standardTargetOptions(.{}),
        .optimize = b.standardOptimizeOption(.{}),
    });
    _ = module;
}
