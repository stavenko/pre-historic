
use amethyst::{
  prelude::{ World },
  renderer::{
    PngFormat,
    Texture,
    TextureMetadata,
    SpriteSheetHandle,
    SpriteSheetFormat,
    SpriteSheet,
    TextureHandle
  }, 
  assets::{ SimpleFormat, Loader, AssetStorage }
};

pub fn load_png_texture(world: &mut World, path: String) -> TextureHandle 
{
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      path,
      PngFormat,
      TextureMetadata::srgb_scale(),
      (),
      &texture_storage
    )
  };
  texture_handle
}
pub fn load_ss_asset(world: &mut World, path: String, th: TextureHandle) -> SpriteSheetHandle {
  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    path,
    SpriteSheetFormat,
    th,
    (),
    &sprite_sheet_store
  )
}
