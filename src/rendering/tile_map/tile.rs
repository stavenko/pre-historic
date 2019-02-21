use std::hash::{ Hash, Hasher };
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::core::nalgebra::{ Vector2, Vector3, Matrix2 };

pub struct Tile {
  pub position: Vector3<i32>
}

pub struct TileSprite {
  pub offset: Vector2<f32>,
  pub size: Vector2<f32>
}
  

impl Component for Tile {
  type Storage = DenseVecStorage<Self>;
}

impl Component for TileSprite {
  type Storage = DenseVecStorage<Self>;
}

impl Hash for TileSprite {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.offset.x.to_bits().hash(state);
    self.offset.y.to_bits().hash(state);
    self.size.x.to_bits().hash(state);
    self.size.y.to_bits().hash(state);
  }
}

impl Hash for Tile {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.position.x.hash(state);
    self.position.y.hash(state);
  }
}

impl Tile {
  pub fn apply_basis(&self, basis: &Matrix2<f32>) -> Vector2<f32> {
    let p = Vector2::<f32>::new(
      self.position.x as f32,
      self.position.y as f32
    );
    basis * p
  }
}
