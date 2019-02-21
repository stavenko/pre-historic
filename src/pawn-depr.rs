use amethyst::{
  ecs::{Component, DenseVecStorage}
};

#[derive(Clone)]
pub struct RacialPawnProperties {
  skin_set: String,
}

#[derive(Clone)]
pub struct Pawn {
  racial: RacialPawnProperties,
}

impl Pawn {
  pub fn new() -> Pawn {
    Pawn {
      racial: RacialPawnProperties {
        skin_set: "default".to_string()
      }
    }
  }
}

impl Component for Pawn {
  type Storage = DenseVecStorage<Self>;
}
