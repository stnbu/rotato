use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::TAU;

pub fn generate_tree<'a>(
    depth: i32,
    max_depth: i32,
    max_children: usize,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let max_children = 1;
    let max_depth = 1;
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

    let transform = if depth == 0 {
        Transform::default()
    } else {
        let x: f32 = rng.gen::<f32>() * 0.2;
        let y: f32 = 2.0 + rng.gen::<f32>() * 0.1;
        Transform::from_translation(Vec3::Y * y + Vec3::X * x)
    };
    let entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.25,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::WHITE.into()),
            transform,
            ..Default::default()
        })
        .push_children(children.as_slice())
        .id();
    entity
}
