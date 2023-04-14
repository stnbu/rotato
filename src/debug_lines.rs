use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn debug_lines(mut lines: ResMut<DebugLines>) {
    lines.line_colored(Vec3::X * 20.0, Vec3::X * -20.0, 0.0, Color::RED);
    lines.line_colored(Vec3::Y * 20.0, Vec3::Y * -20.0, 0.0, Color::GREEN);
    lines.line_colored(Vec3::Z * 20.0, Vec3::Z * -20.0, 0.0, Color::BLUE);
}
