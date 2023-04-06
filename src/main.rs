use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

mod tree;
use tree::*;

const EPSILON: f32 = 0.00001;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_system(control)
        .run();
}

#[derive(Component)]
struct CameraGimbal;

#[derive(Component)]
struct CameraBoom;
use bevy::prelude::SpatialBundle;
fn spawn_camera(mut commands: Commands) {
    commands.spawn(SpatialBundle {
        visibility: Visibility::Visible,
        ..Default::default()
    });
    commands
        .spawn((SpatialBundle::default(), CameraBoom))
        .with_children(|children| {
            children.spawn((
                Camera3dBundle {
                    transform: Transform::from_translation(Vec3::splat(2.5))
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
                CameraGimbal,
            ));
            children.spawn(DirectionalLightBundle {
                transform: Transform::default().looking_at(Vec3::new(1.0, 1.0, 1.0), Vec3::Y),
                visibility: Visibility::Visible,
                directional_light: DirectionalLight {
                    shadows_enabled: false,
                    ..Default::default()
                },
                ..Default::default()
            });
            children.spawn(DirectionalLightBundle {
                transform: Transform::default().looking_at(Vec3::new(-1.0, -1.0, 1.0), Vec3::Y),
                visibility: Visibility::Visible,
                directional_light: DirectionalLight {
                    shadows_enabled: false,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn control(
    mut boom_query: Query<&mut Transform, With<CameraBoom>>,
    mut gimbal_query: Query<&mut Transform, (With<CameraGimbal>, Without<CameraBoom>)>,
    mut scroll_evr: EventReader<MouseWheel>,
    keys: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut boom = boom_query.get_single_mut().unwrap();
    let mut gimbal = gimbal_query.get_single_mut().unwrap();
    // FIXME: Handle "line" vs "pixel" at some point.
    let boom_scale_delta = scroll_evr.iter().fold(0.0_f32, |b, delta| b + delta.y);
    let new_scale = boom.scale - Vec3::ONE * boom_scale_delta / 100.0;
    if new_scale.x > EPSILON && new_scale.y > EPSILON && new_scale.z > EPSILON {
        boom.scale = new_scale;
    }
    let mouse_delta = motion_evr
        .iter()
        .fold(Vec2::ZERO, |b, ev| b + Vec2::new(ev.delta.x, ev.delta.y))
        * 0.01;
    if mouse_delta.length() > EPSILON {
        if keys.pressed(KeyCode::B) {
            let local_x = boom.local_x();
            let local_y = boom.local_y();
            boom.rotate(Quat::from_axis_angle(local_x, -mouse_delta.y));
            boom.rotate(Quat::from_axis_angle(local_y, -mouse_delta.x));
        }
        if keys.pressed(KeyCode::G) {
            let local_x = gimbal.local_x();
            let local_y = gimbal.local_y();
            gimbal.rotate(Quat::from_axis_angle(local_x, -mouse_delta.y));
            gimbal.rotate(Quat::from_axis_angle(local_y, -mouse_delta.x));
        }
    }
}

#[derive(Component)]
struct RotRate(u32);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    generate_tree(0, 4, 3, &mut commands, &mut meshes, &mut materials);
}
