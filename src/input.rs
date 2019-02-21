extern crate amethyst_core;

use winit::{
  ButtonId, ElementState, WindowEvent, DeviceEvent, Event, KeyboardInput
};

use amethyst::{
  core::{
    shrev::{ EventChannel, ReaderId },
    nalgebra::{ Vector2 },
    specs::{
      prelude::{
        Read, System, Resources
      }
    }
  }
};

fn el2str(st: ElementState) -> &'static str {
  match st {
    Released => "Released",
    _ => "Pressed"
  }
}

pub struct InputSystem {
  mouse: Vector2<i32>,
  event_reader: Option<ReaderId<Event>>,
}
impl InputSystem {
  pub fn new() -> Self {
    InputSystem {
      mouse: Vector2::<i32>::new(0, 0),
      event_reader: None,
    }
  }
  
  fn process_mouse_move(&mut self, delta: &(f64, f64)) {
    let (x, y) = delta;
    println!("mouse moved {}, {}", x, y);
  }

  fn process_keyboard(&mut self, inp: &KeyboardInput) {
    println!("button touched {}, {}", inp.scancode, el2str(inp.state));
  }

  fn process_button(&mut self, button: ButtonId, st: ElementState) {
  }

}

impl<'a> System<'a> for InputSystem {
  type SystemData = (
    Read<'a, EventChannel<Event>>
  );

  fn run(&mut self, event_channel: Self::SystemData) {
    match self.event_reader.as_mut() {
      None => {
        println!("setup of InputSystem isn`t called");
        return;
      }
      Some(mut reader) => {
        for event in event_channel.read(&mut reader) {
          if let Event::DeviceEvent { ref event, .. } = *event {
            println!("device event");
            /*
            if let DeviceEvent::Key(keybord_input)  = *event {
              self.process_keyboard(&keybord_input);
            }
            */
            if let DeviceEvent::MouseMotion { delta } = *event {
              self.process_mouse_move(&delta);
            }
            /*
            if let DeviceEvent::Button { button, state } = *event {
              self.process_button(button, state);
            }
            */
          }
          if let Event::WindowEvent{ ref event, .. } = *event {
            println!("window event");
            if let WindowEvent::KeyboardInput { input, .. } = *event {
              self.process_keyboard(&input);
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
