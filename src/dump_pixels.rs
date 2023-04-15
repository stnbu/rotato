/*
Weeeeell, this is not easy.

https://gist.github.com/DGriffin91/7478cfcc654176d202230758f7606c2d
https://github.com/bevyengine/bevy/issues/22
https://github.com/bevyengine/bevy/issues/1207
https://github.com/bevyengine/bevy/pull/7163
https://github.com/bevyengine/bevy/pull/5550 (closes 4572)

https://github.com/rmsc/bevy/tree/render_to_file
https://github.com/mrk-its/bevy/tree/render_to_texture
https://github.com/TheRawMeatball/bevy/tree/read-texture-node
 */

// // Failures, I've had a few.
//
// use bevy::prelude::*;
// use bevy::render::render_resource::Extent3d;
// use bevy::render::render_resource::Texture;
// use image::{ImageBuffer, Rgba};
// use std::fs::create_dir_all;
// pub fn write_pixels(
//     mut commands: Commands,
//     texture_query: Query<&Handle<Texture>>,
//     mut frame_number: Local<u64>,
//     windows: Query<&Window>,
// ) {
//     let window = windows.get_single().unwrap();
//     let texture_handle = texture_query.single().unwrap();
//     let texture = texture_handle
//         .as_ref()
//         .unwrap()
//         .as_cpu_accessible()
//         .unwrap();
//     let extent = Extent3d::new(texture.width(), texture.height(), 1);
//     let pixels = texture.read_pixels(&mut commands, extent).unwrap();
//     let image = ImageBuffer::from_fn(texture.width(), texture.height(), |x, y| {
//         let index = ((y * texture.width() + x) * 4) as usize;
//         Rgba([
//             pixels[index],
//             pixels[index + 1],
//             pixels[index + 2],
//             pixels[index + 3],
//         ])
//     });
//     let file_path = format!("frames/{:020}.png", frame_number);
//     image.save(file_path).unwrap();
//     *frame_number += 1;
// }

// use bevy::{prelude::*, render::camera::VisibleEntities};
// use image::{ImageBuffer, Rgba};
// use std::fs::File;
// pub fn save_camera_view(
//     visible_entities: Res<VisibleEntities>,
//     mut textures: ResMut<Assets<Texture>>,
//     mut commands: Commands,
// ) {
//     for visible_entity in visible_entities.iter() {
//         if let Some(texture_handle) = visible_entity.components().get::<TextureHandle>() {
//             let texture = textures.get_mut(texture_handle).unwrap();
//             let pixels = texture.view().raw_pixels();
//             let width = texture.width();
//             let height = texture.height();
//             let mut image = ImageBuffer::new(width as u32, height as u32);
//             for (x, y, pixel) in image.enumerate_pixels_mut() {
//                 let pixel_index = (y as usize * width as usize + x as usize) * 4;
//                 *pixel = Rgba([
//                     pixels[pixel_index],
//                     pixels[pixel_index + 1],
//                     pixels[pixel_index + 2],
//                     pixels[pixel_index + 3],
//                 ]);
//             }
//             let file = File::create("output.png").unwrap();
//             image.save(file, image::PNG).unwrap();
//         }
//     }
// }
