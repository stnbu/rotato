use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use std::f32::consts::TAU;

mod tree;
use tree::*;

mod gui;
use gui::*;

mod random;

const EPSILON: f32 = 0.00001;

#[derive(Resource, Default, Copy, Clone)]
pub struct Parameters {
    pub max_children: usize,
    pub max_depth: i32,
    pub color_endpoints: (Color, Color),
    pub y_bias: f32,
}

impl Parameters {
    pub fn get_intermediate_color(&self, position: f32) -> Color {
        assert!(position >= 0.0 && position <= 1.0);
        let (start, end) = self.color_endpoints;
        let coefficient = 1.0 - position;
        let start = [
            coefficient * start.r(),
            coefficient * start.g(),
            coefficient * start.b(),
            coefficient * start.a(),
        ];
        let coefficient = position;
        let end = [
            coefficient * end.r(),
            coefficient * end.g(),
            coefficient * end.b(),
            coefficient * end.a(),
        ];
        let mut result = [0.0_f32; 4];
        for i in 0..4 {
            result[i] = start[i] + end[i];
        }
        result.into()
    }
}

#[derive(Component)]
struct CameraGimbal;

#[derive(Component)]
struct CameraBoom;

fn random_unit_vector(y_bias: f32) -> Vec3 {
    let theta = random::gen::<f32>() * TAU;
    let phi = random::gen::<f32>() * TAU / 2.0 / y_bias;
    let x = phi.sin() * theta.cos();
    let y = phi.cos();
    let z = phi.sin() * theta.sin();
    Vec3::new(x, y, z)
}

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
                    transform: Transform::from_translation(Vec3::Z * 10.0)
                        .looking_at(Vec3::Y * 3.0, Vec3::Y),
                    ..Default::default()
                },
                CameraGimbal,
            ));
            for _ in 0..5 {
                children.spawn(DirectionalLightBundle {
                    transform: Transform::default().looking_at(random_unit_vector(1.0), Vec3::Y),
                    visibility: Visibility::Visible,
                    directional_light: DirectionalLight {
                        shadows_enabled: false,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
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

fn redraw_tree(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    parameters: Res<Parameters>,
    tree_entities: Query<Entity, With<Tree>>,
) {
    if parameters.is_added() || parameters.is_changed() {
        tree_entities.for_each(|e| commands.entity(e).despawn());
        let (_, stick) = generate_tree(
            0,
            parameters.as_ref(),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        commands.entity(stick).despawn();
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Parameters {
        max_children: 4,
        max_depth: 5,
        color_endpoints: (Color::rgb_u8(139, 69, 19), Color::GREEN),
        y_bias: 5.0,
    })
    .insert_resource(Rotating::default())
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugin(EguiPlugin)
    .add_startup_system(spawn_camera)
    .add_system(redraw_tree)
    .add_system(menu)
    .add_system(rotate)
    .add_system(toggle_rotation)
    .add_system(control);
    app.run();
}
