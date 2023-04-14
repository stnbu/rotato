use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

use crate::random_unit_vector;

#[derive(Component)]
pub struct Rps(pub f32);

pub fn generate_tree<'a>(
    depth: i32,
    max_depth: i32,
    max_children: usize,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) -> (Entity, Entity) {
    let mut rng = rand::thread_rng();
    let num_children: usize = rng.gen::<usize>() % max_children + 1;
    let mut children: Vec<Entity> = vec![];
    if depth < max_depth {
        for _ in 0..num_children {
            let (sphere, stick) = generate_tree(
                depth + 1,
                max_depth,
                max_children,
                commands,
                meshes,
                materials,
            );
            children.push(sphere);
            children.push(stick);
        }
    }

    let translation = random_unit_vector(4.0) * (1.0 + rng.gen::<f32>());
    let transform = if depth == 0 {
        Transform::default()
    } else {
        Transform::from_translation(translation)
    };
    let radius = 0.002475;
    let height = translation.length() - 0.2;
    let stick = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::Cylinder {
                    height,
                    radius,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(translation / 2.0)
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, translation.normalize())),
            ..Default::default()
        })
        .id();

    let radius = 0.05;
    let sphere = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::BLUE.into()),
            transform,
            ..Default::default()
        })
        .insert(Rps((rng.gen::<f32>() - 0.5) * TAU / 360.0
            * 20.0
            * (depth as f32).powi(2)))
        .push_children(children.as_slice())
        .id();
    (sphere, stick)
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
