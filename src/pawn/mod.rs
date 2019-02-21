mod view_properties;
pub mod sprites;
use amethyst::{
  core::transform::{ Transform },
  core::nalgebra::{ Vector2 },
  ecs::{ 
    Builder, 
    World, 
  },
  renderer::{ 
    SpriteRender
  },
};
use self::sprites::SpriteCollection;

pub use self::view_properties::{ Sex, Race, Complex, Spece };
use crate::rendering::tile_map::{ Tile };

pub fn place_debug_pawn(world: &mut World, spece: Spece, mut wher: Vector2<f32>, tile: Tile) {
  let mut transform: Transform = Default::default();
  let mut shift = Vector2::<f32>::new(0.0, 128.);
  let scale = 0.15;
  shift *= scale;
  wher -= shift;

  transform.set_scale(scale, scale, 1.0);
  transform.set_xyz(wher.x, wher.y, 0.9);

  world.register::<Spece>();
  let sprite: Option<SpriteRender> = {
    println!("get collection");
    let sprite_collection = world.res.fetch_mut::<SpriteCollection>();
    let spr = sprite_collection.sprites.get(&spece);
    match spr {
      Some(s) => Some(s.clone()),
      _ => None,
    }
  };
  
  match sprite {
    Some(s) => {
      println!("do placement");
      world.create_entity()
        .with(transform)
        .with(spece)
        .with(tile)
        .with(s)
        .build();
    }
    _ => return
  };
}

pub fn register(world: &mut World) {
  world.register::<Spece>()
}

pub fn get_random_sprite(world: &mut World) -> Option<SpriteRender> {
  // let res = world.res.fetch::<SpriteCollection>();
  None 
}

