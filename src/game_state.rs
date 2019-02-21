extern crate amethyst;
extern crate nalgebra_glm;

use amethyst::{
  assets::{ AssetStorage, Loader },
  core::{
    nalgebra::{ Vector2, Vector3, Vector4 },
    transform::Transform,
  },
  renderer:: { 
    Camera,
    SpriteRender,
    SpriteSheet,
    PngFormat,
    SpriteSheetHandle,
    SpriteSheetFormat,
    WindowMessages,
  },
  winit::{ VirtualKeyCode },
  input::{ is_key_down, is_close_requested }

};


use crate::pawn::sprites::SpriteCollection;
use crate::pawn::place_debug_pawn;
use crate::cursor::create_cursor;
// use crate::texture_loader::load_png_texture;
use crate::rendering::tile_map::create_debug_tile_map;
use crate::game_messages::{ GameMessage, GameMessageResource };

use amethyst::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileMapTilesKind {
  Grass,
  Water,
  Dirt
}

unsafe impl Send for TileMapTilesKind {}
unsafe impl Sync for TileMapTilesKind {}
impl Default for TileMapTilesKind {
  fn default() -> Self {
    TileMapTilesKind::Grass
  }
}

pub struct State {
  window_resolution: Vector2<u16>,
  // pawns: Vec<Pawn>
}


impl State {
  pub fn new(width: u16, height: u16) -> State {
    // let pawns = vec!(Pawn::new());
    State {
      // pawns,
      window_resolution: Vector2::<u16>::new(width, height)
    }
  }

  fn init_map(&mut self, world: &mut World) {
    create_debug_tile_map(world, 4, "./resources/sprites/terrain/tiles.png".to_string());
  }

  fn initialize_pawns(&mut self, world: &mut World) {
    SpriteCollection::create_resource(world);
    // world.register::<Pawn>();

    /*
    for pawn in &self.pawns {
      
      let pawn_sprite = SpriteRender {
        sprite_sheet: ssh.clone(),
        sprite_number: 0
      };

      let mut tr = Transform::default();
      tr.set_xyz(250.5, 250.5, 0.0);
      tr.set_scale(0.2, 0.2, 0.4);
      world.create_entity()
        .with(pawn_sprite)
        .with(pawn.clone())
        .with(tr)
        .build();
    }
    */
  }

}

impl SimpleState for State{
  fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let mut msgs = Vec::<GameMessage>::new();
    {
      let mut messages = data.world.write_resource::<GameMessageResource>();
      msgs.append(&mut messages.messages);
    }

      
    for msg in msgs {
      match msg {
        GameMessage::PlacePawn(spece, tile, initial) => place_debug_pawn(data.world, spece, initial, tile),
        _ => continue
      }
    }
    Trans::None
  }
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    self.initialize_pawns(world);


    // let sprite_handle = load_sprite_sheet(world);

    // self.initialize_pawns(world, sprite_handle);
    self.init_map(world);
    create_cursor(world);

    initialize_camera(world, &self.window_resolution);
    use amethyst::renderer::mouse;
    let mut msg = world.res.fetch_mut::<WindowMessages>();

    mouse::grab_cursor(&mut msg);
    mouse::hide_cursor(&mut msg);
  }

  fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
    match &event {
      StateEvent::Window(event) => {
        if is_close_requested(&event) {
          println!("close requested, but we shall not quit right now");
          Trans::None
        } else if is_key_down(&event, VirtualKeyCode::Escape) {
          use amethyst::renderer::mouse;
          let mut msg = data.world.res.fetch_mut::<WindowMessages>();
          mouse::release_cursor(&mut msg);
          Trans::Quit
        } else { Trans::None }
      }
      _ => Trans::None
    }
  }
}

/*
fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
  let texture_handle = load_png_texture(
    world, 
    "./resources/sprites/pawns/default/front.png".to_string(),
  );
  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "./resources/sprites/pawns/default/sprite_sheet.ron",
    SpriteSheetFormat,
    texture_handle,
    (),
    &sprite_sheet_store
  )
}
*/

fn initialize_camera(world: &mut World, resolution: &Vector2<u16>) {
  let mut transform = Transform::default();
  transform.set_xyz(0.0, 0.1, 0.4);
  let w = resolution.x as f32;
  let h = resolution.y as f32;
  let ortho = nalgebra_glm::ortho( -w / 2., w / 2., -h / 2., h / 2., -10., 10.);
  world.create_entity()
    .with(Camera{
      proj: ortho
    })
    .with(transform)
    .build();
}

