use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

use crate::random_unit_vector;

pub fn generate_tree<'a>(
    depth: i32,
    max_depth: i32,
    max_children: usize,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let max_children = 3;
    let max_depth = 3;
    //
    let mut rng = rand::thread_rng();
    let num_children: usize = rng.gen::<usize>() % max_children + 1;
    let mut children: Vec<Entity> = vec![];
    if depth < max_depth {
        for _ in 0..num_children {
            children.push(generate_tree(
                depth + 1,
                max_depth,
                max_children,
                commands,
                meshes,
                materials,
            ));
        }
    }

    let translation = random_unit_vector() * (1.0 + rng.gen::<f32>());
    let transform = if depth == 0 {
        Transform::default()
    } else {
        Transform::from_translation(translation)
    };
    if depth > 0 {
        commands.spawn(PbrBundle {
            mesh: meshes.add(
                shape::Cylinder {
                    height: translation.length() - 1.0,
                    radius: 0.03,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(translation / 2.0)
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, translation.normalize())),
            ..Default::default()
        });
    }

    let entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.25,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::BLUE.into()),
            transform,
            ..Default::default()
        })
        .push_children(children.as_slice())
        .id();
    entity
}
