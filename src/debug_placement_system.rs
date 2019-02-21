use amethyst::{
  core::{
    shrev::{ EventChannel, ReaderId },
    nalgebra::{ Vector2, Vector3 },
    specs::{
      prelude::{
        SystemData, Join, Entities, ReadStorage, Read, Write, System, Resources, 
      }
    },
    transform::{ GlobalTransform }
  },
  renderer::{
    ActiveCamera, Camera, ScreenDimensions
  }

};
use winit::{
  WindowEvent, Event 
};
extern crate shred;
extern crate shred_derive;
// #[macro_use]
use shred_derive::*;

use crate::cursor::{ Cursor };
use crate::rendering::screen_space::screen_rect::{ Transform2D };
use crate::rendering::camera_getter::get_camera;
use crate::rendering::tile_map::{ Tile, TileMap };
use crate::pawn::{ Spece, Sex, Complex, Race };
use crate::game_messages::{ GameMessageResource, GameMessage::{ PlacePawn }};
use nalgebra_glm::*;

pub struct DebugPlacementSystem {
  event_reader: Option<ReaderId<Event>>,
}

impl DebugPlacementSystem {
  pub fn new() -> Self {
    DebugPlacementSystem {
      event_reader: None,
    }
  }
}

#[derive(SystemData)]
pub struct DebugPlacementData<'a > {
  event_channel: Read<'a, EventChannel<Event>>,
  transform: ReadStorage<'a, Transform2D>,
  cur: ReadStorage<'a, Cursor>,
  active_camera: Option<Read<'a, ActiveCamera>>,
  camera: ReadStorage<'a, Camera>,
  camera_transform: ReadStorage<'a, GlobalTransform>,
  screen_dimensions: Option<Read<'a, ScreenDimensions>>,
  tile_map: ReadStorage<'a, TileMap>,
  game_messages: Write<'a, GameMessageResource>
}

impl<'a> System<'a> for DebugPlacementSystem {
  type SystemData = DebugPlacementData<'a>;

  fn run(&mut self, mut system: Self::SystemData) {
    match self.event_reader.as_mut() {
      None => {
        println!("setup of Debug Placement system isn`t called");
        return;
      }
      Some(mut reader) => {
        let ac = system.active_camera;
        let maybe_camera = get_camera(
          ac,
          &system.camera,
          &system.camera_transform
        );
        let dimensions = match system.screen_dimensions {
          None => return,
          Some(dim) => vec2(dim.width() as f32, dim.height() as f32)
        };

        let screen_matrix = match maybe_camera {
          None => return,
          Some((cam, glob)) => {

            let view = glob
              .0
              .try_inverse()
              .expect("it must be possible to inverse so simple matrix");
            (view * cam.proj).try_inverse().expect("once again")
          }

        };
        for event in system.event_channel.read(&mut reader) {
          // if let Event::DeviceEvent { ref event, .. } = *event {
            // println!("device");
          // }
          if let Event::WindowEvent{ ref event, .. } = *event {
            match event {
              WindowEvent::MouseInput { .. } => {
                println!("mouse button");
                for (tr, _c) in (&system.transform, &system.cur).join() {
                  let mut mouse_pointer_position = (tr.model * vec3(0., 0., 1.0)).xy();
                  // let mouse_pointer_position = mouse_pointer_position.xy(); / dimensions;
                  mouse_pointer_position.x /= dimensions.x;
                  mouse_pointer_position.y /= dimensions.y;
                  mouse_pointer_position *= 2.0;
                  mouse_pointer_position -= vec2(1.0, 1.0);
                  let mp = screen_matrix * vec4(mouse_pointer_position.x, mouse_pointer_position.y, 1.0, 1.0);
                  println!("place to: {}", mp.xy());
                  for tm in (&system.tile_map).join() {
                    let tile = tm.calculate_tile(&mp.xy());
                    match tile {
                      None => continue,
                      Some(tile) => {
                        println!("tile is found {}", &tile.position);
                        /*
                        let collection = match &system.pawn_sprites_collection {
                          Some(t) => t,
                          _ => continue
                        };
                        */
                        let sp = Spece::Human(Sex::Male, Race::Euro, Complex::Athletic);
                        println!("fetch me");
                        let basis = tm.get_basis();
                        let pos = tile.apply_basis(&basis); 
                        system.game_messages.messages.push( PlacePawn(sp, tile, pos));
                        // post placement message here
                      }
                    }
                  }
                }
              }
              _ => {}
            }
          }
        }
      }
    }
  }

  fn setup(&mut self, res: &mut Resources) {
    use amethyst_core::specs::prelude::SystemData;
    Self::SystemData::setup(res);
    self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
  }
}
