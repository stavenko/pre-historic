extern crate amethyst;
mod pawn;
mod game_state;
mod game_messages;
mod rendering;
mod asset_loader;
mod resource;
mod cursor;
mod debug_placement_system;

use amethyst::{
  prelude::*,
  core::{
    TransformBundle,
    nalgebra::{ Vector2 },
  },
  input::{ InputBundle },
  renderer::{
    DisplayConfig,
    DrawFlat2D,
    // Event,
    Pipeline,
    RenderBundle,
    Stage,
    // VirtualKeyCode
  },
  utils::application_root_dir
};
use gfx::{
  state::ColorMask,
  preset::blend
};
// use crate::tile_map::tile_map_pass;
use crate::rendering::tile_map::TileMapPass;
use crate::rendering::screen_space::screen_space_pass::ScreenSpacePass;
use crate::cursor::mouse_pointer_system::MousePointerSystem;
use crate::debug_placement_system::DebugPlacementSystem;
// use crate::game_state::TileMapTilesKind;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());
  let app_dir = application_root_dir();
  let path = format!("{}/resources/display_config.ron", app_dir);

  let display_config = DisplayConfig::load(path);
  let (w, h) = display_config.dimensions.unwrap();
  let ss_pass: ScreenSpacePass = Default::default(); 
  let pipe = Pipeline::build()
    .with_stage(
      Stage::with_backbuffer()
        .clear_target([0.0, 0.0, 0.2, 1.0], 0.0)
        .with_pass(TileMapPass::new())
        .with_pass(
          DrawFlat2D::new()
          .with_transparency(ColorMask::all(), blend::ALPHA, None)
         )
        .with_pass(ss_pass),
    );
  let input_bundle = InputBundle::<String, String>::new(); 
  let screen_dimensions = Vector2::<f32>::new(w as f32, h as f32);

  let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(input_bundle)?
    .with(MousePointerSystem::new(screen_dimensions/2.0), "mouse_pointer_system", &["input_system"])
    .with(DebugPlacementSystem::new(), "debug_placement_system", &["mouse_pointer_system"])
    .with_bundle(
      RenderBundle::new(pipe, Some(display_config))
        .with_sprite_sheet_processor()
    )?;
  

  let mut game = Application::new(
    app_dir, 
    game_state::State::new(w as u16, h as u16), 
    game_data)?;
  game.run();

  Ok(())
}
