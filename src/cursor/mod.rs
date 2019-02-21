use amethyst::{
  ecs::{ Component, DenseVecStorage },
  prelude::{ Builder, World},
  core::{
    nalgebra::{ Matrix3 }
  }
};
use crate::asset_loader::load_png_texture;
use crate::rendering::screen_space::screen_rect::{ 
  ScreenRect,
  Transform2D
};
pub mod mouse_pointer_system;

pub struct Cursor;
impl Component for Cursor{ 
  type Storage = DenseVecStorage<Self>;
}
  
pub fn create_cursor(world: &mut World) {
  world.register::<ScreenRect>();
  world.register::<Transform2D>();
  world.register::<Cursor>();
  let handle = load_png_texture(world, "./resources/sprites/ui/cursor.png".to_string());
  let mut rect: ScreenRect = Default::default();
  rect.size.x = 48.0;
  rect.size.y = 80.0;
  rect.size /= 4.0;
  rect.position.y = -rect.size.y;
  let transform = Transform2D {
    model: Matrix3::<f32>::identity() 
  };
  world.create_entity()
    .with(rect)
    .with(handle)
    .with(transform)
    .with(Cursor)
    .build();
}
