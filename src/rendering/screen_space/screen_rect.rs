use amethyst::{
  ecs::{ Component, DenseVecStorage },
  core::nalgebra::{ Vector2, Matrix3 },
  renderer::{ TextureHandle }
};

pub struct ScreenRect{
  pub position: Vector2<f32>,
  pub size: Vector2<f32>,
  pub flip_x: bool,
  pub flip_y: bool,

}

impl Default for ScreenRect {
  fn default() -> Self {
    ScreenRect {
      position: Vector2::<f32>::new(0.0, 0.0),
      size: Vector2::<f32>::new(24.0, 24.0),
      flip_x: false,
      flip_y: false,
    }
  }
}

pub struct Transform2D {
  pub model: Matrix3<f32>
}

impl Default for Transform2D {
  fn default() -> Self {
    Transform2D {
      model: Matrix3::<f32>::identity()
    }
  }
}


impl Component for Transform2D {
  type Storage = DenseVecStorage<Self>;
}

impl Component for ScreenRect {
  type Storage = DenseVecStorage<Self>;
}

/*
impl Component for TextureForScreenItem {
  type Storage = DenseVecStorage<Self>;
}
*/
