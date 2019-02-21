use amethyst::renderer::{
  Attribute,
  Attributes,
  AttributeFormat,
  VertexFormat,
  With

};
use gfx::{
  format::{ ChannelType, Format, SurfaceType },
  pso::buffer::{ Element },
  traits::{ Pod },
};

struct PositionWithId;

impl Attribute for PositionWithId {
  const NAME: &'static str = "position_id";
  const FORMAT: Format  = Format(SurfaceType::R32_G32_B32, ChannelType::Int);
  const SIZE: u32 = 12;
  type Repr = [i32; 3];
}

pub struct TileMapAttributes {
  _point: [i32; 3]
}
unsafe impl Pod for TileMapAttributes {}

impl VertexFormat for TileMapAttributes {
  const ATTRIBUTES: Attributes<'static> = &[
    (PositionWithId::NAME, <Self as With<PositionWithId>>::FORMAT),
  ];
}

impl With<PositionWithId> for TileMapAttributes {
  const FORMAT: AttributeFormat = Element {
    offset: 0,
    format: <PositionWithId as Attribute>::FORMAT
  };
}

