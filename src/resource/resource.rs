use std::collections::hash_map::HashMap;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Resource {
  PieceOfMeat,
  Apple,
}

#[derive(Clone, Copy)]
pub struct Stacking {
  unordered: u32,
  ordered: u32,
}

#[derive(Clone, Copy)]
pub struct ResourceInfo {
  stacking: Stacking,
}

pub fn create_resource_dictionary() -> HashMap<Resource, ResourceInfo> {
  let mut map = HashMap::<Resource, ResourceInfo>::new();
  map.insert(Resource::Apple, ResourceInfo { 
    stacking: Stacking {
      unordered: 20,
      ordered: 30
    }
  });
  map.insert(Resource::PieceOfMeat, ResourceInfo { 
    stacking: Stacking {
      unordered: 2,
      ordered: 10
    }
  });
  map
}
