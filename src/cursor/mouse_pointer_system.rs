use amethyst::{
  renderer::{ 
    ScreenDimensions,
  },
  core::{
    shrev::{ EventChannel, ReaderId },
    nalgebra::{ Vector2 },
    specs::{
      prelude::{
        Join, WriteStorage, ReadStorage, Read, System, Resources
      }
    }
  }
};
use nalgebra_glm::{ translation2d, clamp_vec, vec2 };
use winit::{
  ButtonId, ElementState, WindowEvent, DeviceEvent, Event, KeyboardInput
};
use super::Cursor;
use crate::rendering::screen_space::screen_rect::Transform2D;

pub struct MousePointerSystem {
  event_reader: Option<ReaderId<Event>>,
  mouse_coords: Vector2<f32>
}

impl MousePointerSystem {
  pub fn new(cursor: Vector2<f32>) -> Self {
    MousePointerSystem {
      event_reader: None,
      mouse_coords: cursor
    }
  }
}

impl<'a> System<'a> for MousePointerSystem {
  type SystemData = (
    Read<'a, EventChannel<Event>>,
    WriteStorage<'a, Transform2D>,
    ReadStorage<'a, Cursor>,
    Option<Read<'a, ScreenDimensions>>
  );

  fn run(&mut self, (event_channel, mut transforms, cur, screen_dim): Self::SystemData) {
    match self.event_reader.as_mut() {
      None => {
        println!("setup of InputSystem isn`t called");
        return;
      }
      Some(mut reader) => {
        let dimensions: [f32; 2] = match screen_dim{
          None => [0.0, 0.0],
          Some(ref sd) => [sd.width(), sd.height()]
        };
        for event in event_channel.read(&mut reader) {
          if let Event::DeviceEvent { ref event, .. } = *event {
            if let DeviceEvent::MouseMotion { delta } = *event {
              let (x, y) = delta;
              for (mut tr, _is_cursor) in (&mut transforms, &cur).join() {
                let move_by = Vector2::<f32>::new(x as f32, -y as f32);
                let new_coords = self.mouse_coords + move_by;
                let new_coords = clamp_vec(&new_coords, &vec2(0.0, 0.0), &vec2(dimensions[0], dimensions[1]));
                self.mouse_coords = new_coords;
                tr.model = translation2d(&self.mouse_coords);
              }
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
