use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn setup_debug_lines(mut lines: ResMut<DebugLines>) {
    const AXIS_EXTENT: f32 = 10.0;
    const TICK_EXTENT: f32 = 0.05;
    const DURATION: f32 = 1800.0;

    lines.line_colored(
        Vec3::X * AXIS_EXTENT,
        Vec3::X * -AXIS_EXTENT,
        DURATION,
        Color::RED,
    );
    lines.line_colored(
        Vec3::Y * AXIS_EXTENT,
        Vec3::Y * -AXIS_EXTENT,
        DURATION,
        Color::GREEN,
    );
    lines.line_colored(
        Vec3::Z * AXIS_EXTENT,
        Vec3::Z * -AXIS_EXTENT,
        DURATION,
        Color::BLUE,
    );

    let tick_count = AXIS_EXTENT.floor() as i32;

    for distance in 1..tick_count {
        for side in [-1, 1] {
            let distance = (side * distance) as f32;

            let from = Vec3::new(distance, TICK_EXTENT, 0.0);
            let to = Vec3::new(distance, -TICK_EXTENT, 0.0);
            lines.line_colored(from, to, DURATION, Color::RED);

            let from = Vec3::new(0.0, distance, TICK_EXTENT);
            let to = Vec3::new(0.0, distance, -TICK_EXTENT);
            lines.line_colored(from, to, DURATION, Color::GREEN);

            let from = Vec3::new(TICK_EXTENT, 0.0, distance);
            let to = Vec3::new(-TICK_EXTENT, 0.0, distance);
            lines.line_colored(from, to, DURATION, Color::BLUE);
        }
    }
}
