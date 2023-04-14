use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

use crate::random_unit_vector;

#[derive(Component)]
pub struct Rps(pub f32);

pub fn generate_tree<'a>(
    depth: i32,
    parameters: &crate::Parameters,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) -> (Entity, Entity) {
    let mut rng = rand::thread_rng();
    let num_children: usize = rng.gen::<usize>() % parameters.max_children + 1;
    let mut children: Vec<Entity> = vec![];
    if depth < parameters.max_depth {
        for _ in 0..num_children {
            let (ball, stick) = generate_tree(depth + 1, parameters, commands, meshes, materials);
            children.push(ball);
            children.push(stick);
        }
    }

    let transform = if depth == 0 {
        Transform::default()
    } else {
        Transform::from_translation(
            (random_unit_vector(4.0) / (depth as f32 + rng.gen::<f32>())) * 2.0,
        )
    };
    let relative_depth = depth as f32 / parameters.max_depth as f32;
    let color = parameters.get_intermediate_color(relative_depth);

    // stick
    let stick = if depth == 0 {
        commands.spawn(SpatialBundle::default()).id()
    } else {
        let radius = 0.002475;
        let height = transform.translation.length();
        let height = if height > 0.0 { height } else { 1.0 };

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        height,
                        radius,
                        ..Default::default()
                    }
                    .into(),
                ),
                material: materials.add(color.with_a(0.8).into()),
                transform: Transform::from_translation(transform.translation / 2.0).with_rotation(
                    Quat::from_rotation_arc(Vec3::Y, transform.translation.normalize()),
                ),
                ..Default::default()
            })
            .id()
    };

    let radius = 0.05;
    let ball = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(color.into()),
            transform,
            ..Default::default()
        })
        .insert(Rps((rng.gen::<f32>() - 0.5) * TAU / 360.0
            * 20.0
            * (depth as f32).powi(2)))
        .push_children(children.as_slice())
        .id();
    (ball, stick)
}

#[derive(Resource, Default)]
pub struct Rotating(pub bool);

pub fn toggle_rotation(keys: Res<Input<KeyCode>>, mut rotating: ResMut<Rotating>) {
    if keys.just_pressed(KeyCode::Space) {
        rotating.0 = !rotating.0;
    }
}

pub fn rotate(
    rotating: Res<Rotating>,
    mut rotations: Query<(&mut Transform, &Rps)>,
    time: Res<Time>,
) {
    if rotating.0 {
        for (mut transform, &Rps(rps)) in rotations.iter_mut() {
            let delta_s = time.delta_seconds();
            transform.rotate_y(rps * delta_s);
        }
    }
}
