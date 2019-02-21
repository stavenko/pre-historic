use amethyst::core::nalgebra::{ Vector2, Vector3 };
use nalgebra_glm::{ vec2, vec3 };
use amethyst::prelude::{ World, Builder };

use super::tile_map::{ TextureInfo, TileMap, hex_basis };
use super::tile::{ Tile, TileSprite };
use crate::asset_loader::load_png_texture;

pub fn create_debug_tile_map(world: &mut World, size: u16, texture_path: String) {
  let center: Vector3<i32> = Vector3::new(0, 0, 0); 
  let t = load_png_texture(world, texture_path);
  let t = TextureInfo {
    texture: t,
    size: Vector2::new(1024, 1024)
  };

  let scale = Vector2::<f32>::new(20., 10.);
  let tm = TileMap {
    scale,
    basis: hex_basis(scale)
  };

  create_debug_tiles(world, &tm, size, center);
  world.register::<TileMap>();
  world.register::<TextureInfo>();
  world.create_entity()
    .with(t)
    .with(tm)
    .build();
}

pub fn create_debug_tiles(
  world: &mut World,
  _tile_map: &TileMap,
  size: u16,
  mut center_tile: Vector3<i32>
) {
  let side = size as i32;
  let start: i32 = -(side - 1);
  let end: i32 = side;
  if (center_tile.x + center_tile.y + center_tile.z) != 0 {
    center_tile.z = -center_tile.x - center_tile.y;
  }
  let tile_size = 1.0 / 4.0;

  world.register::<Tile>();
  world.register::<TileSprite>();

  for x in start..end {
    for y in start..end {
      for z in start..end {
        let t = vec3(x, y, z) + center_tile;
        if (t.x + t.y + t.z) == 0 {
          
          world.create_entity()
            .with(Tile {
              position: t
            })
            .with(TileSprite {
              offset: vec2(0.0, 0.0),
              size: vec2(tile_size, tile_size)
            })
            .build();
        }
      }
    }
  }
}
