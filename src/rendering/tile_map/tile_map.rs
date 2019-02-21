use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::core::nalgebra::{ Vector2, Vector3, Matrix2 };
use amethyst::renderer::{ TextureHandle };
use super::tile::Tile;

type Basis = (
  Vector3<f32>, 
  Vector3<f32>, 
  Vector3<f32>
);

pub struct TextureInfo {
  pub texture: TextureHandle,
  pub size: Vector2<u16>,
}

pub struct TileMap {
  pub basis: Basis,
  pub scale: Vector2<f32>
}

impl TileMap {
  pub fn get_basis(&self) -> Matrix2<f32> {
    let (x, y, ..) = self.basis;
    Matrix2::<f32>::new(
      x.x, 
      y.x, 
      x.y,
      y.y 
    )
  }

  pub fn calculate_tile(&self, v: &Vector2<f32>) -> Option<Tile> {
    self.get_basis()
      .try_inverse()
      .map(|m| {
        let v = m * v;
        let v = Vector2::<i32>::new(
          v.x.round() as i32,
          v.y.round() as i32,
        );

        Tile {
          position: Vector3::<i32>::new(v.x, v.y, -v.x - v.y)
        }
      })
  }
}

impl Component for TileMap {
  type Storage = DenseVecStorage<Self>;
}

impl Component for TextureInfo {
  type Storage = DenseVecStorage<Self>;
}

pub fn hex_basis(scale: Vector2<f32>) -> Basis {
  use std::f32::consts::{ PI };
  let angle_axis = PI / 6.0; 
  let angle_hex = PI / 3.0; 
  let k = 2.0 * angle_hex.sin();
  (
    Vector3::<f32>::new(
      angle_axis.cos() * k * scale.x, 
      angle_axis.sin() * k * scale.y, 0.0) * 1.0, 
    Vector3::<f32>::new(0.0, 1.0, 0.0) * k * scale.y,
    Vector3::<f32>::new(0.0, 0.0, 1.0)
  )
}
