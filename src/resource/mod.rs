mod resource;

use amethyst::{
  ecs:: {
    World,
    Builder
  },
};

use crate::rendering::tile_map::Tile;
pub use self::resource::{
  Resource,
  Stacking,
  ResourceInfo,
};


pub fn place_resource(world: &mut World, res: Resource, tile: Tile) {
  
}
