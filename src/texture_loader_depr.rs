use amethyst::{
  prelude::{ World },
  renderer::{ PngFormat, Texture, TextureMetadata, TextureHandle }, 
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
