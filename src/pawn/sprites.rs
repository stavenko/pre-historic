use std::collections::{ HashMap };
use amethyst::ecs::World;
use amethyst::renderer::{ SpriteRender };
use super::view_properties::{ Spece, Race, Sex, Complex };
use crate::asset_loader::{ load_png_texture, load_ss_asset };

pub struct SpriteCollection {
  pub sprites: HashMap<Spece, SpriteRender>,
}

impl SpriteCollection {
  pub fn create_resource(world: &mut World) {
    let mut collection = SpriteCollection::new();
    collection.load_textures(world);
    world.res.insert(collection);
  }
   
  fn new () -> Self {
    SpriteCollection {
      sprites: HashMap::<Spece, SpriteRender>::new()
    }
  }

  fn load_textures(&mut self,world: &mut World) {
    let handle = load_png_texture(world, "resources/sprites/pawns/default/front.png".to_string());
    let sprite_sheet_handle = load_ss_asset(
      world, 
      "resources/sprites/pawns/default/sprite_sheet.ron".to_string(),
      handle
    );

    let spece = Spece::Human(Sex::Male, Race::Euro, Complex::Athletic);
    self.sprites.insert(spece, SpriteRender {
      sprite_sheet: sprite_sheet_handle,
      sprite_number: 0
    });
  }
}
