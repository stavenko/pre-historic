use crate::rendering::tile_map::Tile;
use crate::pawn::Spece;
use amethyst::core::nalgebra::{ Vector2 };
pub enum GameMessage {
  PlacePawn(Spece, Tile, Vector2<f32>),
}

pub struct GameMessageResource {
  pub messages: Vec<GameMessage>,
}

impl Default for GameMessageResource {
  fn default() -> Self {
    GameMessageResource {
      messages: Vec::new()
    }
  }
}
