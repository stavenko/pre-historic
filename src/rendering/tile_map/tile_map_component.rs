use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use amethyst::{
  prelude::{ World },
  ecs::{Component, DenseVecStorage},
  assets::{ AssetStorage },
  core::{
    nalgebra::{ Matrix2, Vector2, Vector3, Vector4 },
  },
  renderer:: { 
    PngFormat, 
    Texture, 
    TextureHandle, 
  }
};
use crate::texture_loader::load_png_texture;
// use glsl_layout::*;

use std::f32::consts::{ PI };

type Basis = (
  Vector3<f32>,
  Vector3<f32>,
  Vector3<f32>
);
  
pub struct Tile{
  pub position:  Vector3<i32>,
  pub texture: Vector4<u32>,
}

impl Tile{
  fn get_hex_matrix(basis: &Basis) -> Matrix2<f32> {
    let (x, y, ..) = basis;
    Matrix2::<f32>::new(
      x.x, 
      y.x, 
      x.y,
      y.y 
    )
  }

  fn hex_to_world(&self, basis: &Basis) -> Vector2<f32> {

    let hex = Vector2::<f32>::new(
      self.position.x as f32, 
      self.position.y as f32
    );
    Tile::get_hex_matrix(basis) * hex
  }
}

pub struct TextureSprite<T: Eq + PartialEq> {
  pub offset: Vector2<u32>,
  pub size: Vector2<u32>,
  pub kind: T,
}

pub struct TileMapTextureDescriptor<T: Eq + PartialEq>{
  pub size: Vector2<u16>,
  pub tiles: Vec<TextureSprite<T>>,
}

impl<T: Eq + PartialEq + Default> Default for TileMapTextureDescriptor<T> {
  fn default() -> Self {
    TileMapTextureDescriptor {
      size: Vector2::<u16>::new(0, 0),
      tiles: Vec::new(),
    }
  }
}

pub struct TileMap<T: Sync + Eq + PartialEq> {
  tiles: Vec<Tile>,
  texture_handle: TextureHandle,
  texture_descriptor: TileMapTextureDescriptor<T>,
  origin: Vector3<f32>,
  scale: Vector2<f32>,
  basis: Basis,
  // hash: Option<u64>
}

impl<T: Sync + Eq + PartialEq> Hash for TileMap<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    for tile in &self.tiles {
      tile.position.x.hash(state);
      tile.position.y.hash(state);
      tile.position.z.hash(state);

      tile.texture.x.hash(state);
      tile.texture.y.hash(state);
    }
  }
}

impl<T: 'static + Default + Sync + Eq> TileMap<T> {
  pub fn build() -> TileMapBuilder<T> {
    Default::default()
  }

  pub fn calculate_hash(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }

  pub fn get_texture_size(&self) -> [f32; 2] {
    let [x, y]: [u16; 2] = self.texture_descriptor.size.into();
    [x as f32, y as f32]
  }

  pub fn get_scale(&self) -> [f32; 2] {
    self.scale.into()
  }
  
  pub fn get_texture<'a>(&self, texture_storage: &'a AssetStorage<Texture> ) -> Option<&'a Texture> {
    texture_storage.get(&self.texture_handle)
  }

  pub fn get_arrays_float_interleaved(&self) -> (usize, Vec<f32>) { 
    let (vertex, offset, size) = self.get_arrays();
    let mut result = Vec::<f32>::new();
    let amount = self.tiles.len();
    for i in 0..amount {
      for c in 0..3 {
        result.push(vertex[i*3 + c]);
      }
      for c in 0..2 {
        result.push(offset[i*2 + c] as f32);
      }
      for c in 0..2 {
        result.push(size[i*2 + c] as f32);
      }
    }
    (amount, result)
  }

  pub fn world_to_hex_tile(&self, position_in_world: &Vector2<f32>) -> Option<&Tile> {
    let m = Tile::get_hex_matrix(&self.basis);
    let m = m.try_inverse().expect("Could not create inverse matrix from 2d basis");
    let s = m * position_in_world;
    let tile = self.tiles.iter().find(|t| {
      t.position.x == s.x.round() as i32 &&
      t.position.y == s.y.round() as i32
    });
    tile
  }

  pub fn get_arrays(&self) -> (Vec<f32>, Vec<u32>, Vec<u32>) {
    let mut tile_coords_array = Vec::<f32>::new(); 
    let mut texture_offset = Vec::<u32>::new();
    let mut texture_slice_size = Vec::<u32>::new();
    for tile in &self.tiles {
      let world_coords = tile.hex_to_world(&self.basis);
      tile_coords_array.push(world_coords.x);
      tile_coords_array.push(world_coords.y);
      tile_coords_array.push(0.0);


      texture_offset.push(tile.texture.x);
      texture_offset.push(tile.texture.y);
      texture_slice_size.push(tile.texture.z);
      texture_slice_size.push(tile.texture.w);

    }
    (tile_coords_array, texture_offset, texture_slice_size)
  }
}

impl<'a, T: 'static + Sync + Eq + Send > Component for TileMap<T> {
  type Storage = DenseVecStorage<Self>;
}


pub struct TileMapBuilder<T: Eq + PartialEq> {
  is_hex: bool,
  size: usize,
  tile_filler: Box<Fn(&TileMapTextureDescriptor<T>, &Vector3<i32>) -> Vector4<u32>>,
  texture_handle: Option<TextureHandle>,
  texture_descriptor: TileMapTextureDescriptor<T>,
  tile_scale: Vector2<f32>,
}

fn default_filler<T: Eq>(_t: &TileMapTextureDescriptor<T>, v: &Vector3<i32>) -> Vector4<u32> {
  println!("------------- default filler ---------------");
  Vector4::<u32>::new(
    (v.x + 3).abs() as u32 % 2, 
    (v.y + 3).abs() as u32 % 2, 
    (v.y + 3).abs() as u32 % 2, 
    (v.y + 3).abs() as u32 % 2, 
  )
}

impl<T: 'static + Eq + Default> Default for TileMapBuilder<T> {
  fn default() -> Self {
    let size = 3;

    TileMapBuilder {
      is_hex: true,
      size,
      texture_handle: None,
      texture_descriptor: Default::default(),
      tile_scale: Vector2::<f32>::new(1.0, 1.0),
      tile_filler: Box::new(&default_filler)
    }
  }
}


impl<T: Eq + Sync> TileMapBuilder<T> {
  pub fn with_filler(mut self, f: &'static Fn(&TileMapTextureDescriptor<T>, &Vector3<i32>) -> Vector4<u32>) -> Self {
    self.tile_filler = Box::new(f);
    self
  }

  pub fn with_tile_scale(mut self, x: f32, y: f32) -> Self {
    self.tile_scale = Vector2::<f32>::new(x, y);
    self
  }

  pub fn with_texture(
    mut self,
    path: String, 
    desctriptor: TileMapTextureDescriptor<T>,
    world: &mut World
  ) -> Self {
    let texture_handle = load_png_texture(world, path);
    self.texture_handle = Some(texture_handle);
    self.texture_descriptor = desctriptor;
    self
  }

  pub fn with_quad(mut self) -> Self {
    self.is_hex = false;
    self
  }

  pub fn with_hex(mut self) -> Self {
    self.is_hex = true;
    self
  }

  pub fn with_size(mut self, s: usize) -> Self {
    self.size = s;
    self
  }

  pub fn build(self) -> TileMap<T> {
    let (tiles, basis) = if self.is_hex {
      let angle_axis = PI / 6.0; 
      let angle_hex = PI / 3.0; 
      let k = 2.0 * angle_hex.sin();
      let scale = self.tile_scale;
      let basis:Basis = (
        Vector3::<f32>::new(
          angle_axis.cos() * k * scale.x, 
          angle_axis.sin() * k * scale.y, 0.0) * 1.0, 
        Vector3::<f32>::new(0.0, 1.0, 0.0) * k * scale.y,
        Vector3::<f32>::new(0.0, 0.0, 1.0)
      );
      let tiles = self.get_hex_map_tiles(self.size);
      (tiles, basis)
    } else {
      let basis = (
        Vector3::<f32>::new(1.0, 0.0, 0.0), 
        Vector3::<f32>::new(0.0, 1.0, 0.0),
        Vector3::<f32>::new(0.0, 0.0, 1.0)
      );
      let tiles = self.get_quad_map_tiles(self.size);
      (tiles, basis)
    };

    let origin = Vector3::<f32>::new(0.0, 0.0, 0.0);
    TileMap {
      texture_descriptor: self.texture_descriptor,
      tiles,
      basis,
      texture_handle: self.texture_handle.unwrap(),
      origin,
      scale: self.tile_scale
    }
	}  

  fn get_quad_map_tiles(&self, uside: usize) -> Vec<Tile> {
    // let (basis_x, basis_y, _bz) = basis;
    // let angle = PI / 6.0;
    let side = uside as i32;
    let mut faces:Vec<Tile> = Vec::new();
    let x_start: i32 = -(side - 1);
    let x_end: i32 = side;
    for x in x_start..x_end {
      let y_start = -(side-1);
      let y_end = side;
      for y in y_start..y_end {
        let b = Vector3::<i32>::new(x, y, 0);
        // let wo16rld_point = basis_x * x as f32 + basis_y * y as f32;
        let t = (self.tile_filler)(&self.texture_descriptor, &b);
        faces.push(Tile {
          position: b,
          texture: t
        });
      }
    }
    faces
  }

  fn get_hex_map_tiles(&self, uside: usize) -> Vec<Tile> {
    let side = uside as i32;
    let mut tiles:Vec<Tile> = Vec::new();
    let start: i32 = -(side - 1);
    let end: i32 = side;
    
    for x in start..end {
      for y in start ..end {
        for z in start..end {
          if x + y + z == 0 {
            let b = Vector3::<i32>::new(x, y, z);
            let t = (self.tile_filler)(&self.texture_descriptor, &b);
            tiles.push(Tile {
              position: b,
              texture: t
            });
          }
        }
      }
    }
    tiles
  }
}


