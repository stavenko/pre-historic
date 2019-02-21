use amethyst::{
  assets::{ AssetStorage },
  core::{
    nalgebra::{ Vector2, Matrix3 },
    specs::{ 
      prelude::{
        Join, Read, ReadStorage
      }
    }
  },
  renderer:: { 
    TextureHandle,
    Texture,
    ScreenDimensions,
    error::{ Result },
    Resources,
    AttributeFormat,
    Encoder,
    VertexFormat,
    Attributes,
    Attribute,
    With,
    Factory,
    pipe::{
      pass::{ Pass, PassData },
      Effect, NewEffect
    }
  }
};

use gfx::{
  Slice,
  Primitive,
  // IndexBuffer,
  handle::Buffer,
  preset::blend,
  state::ColorMask,
  traits::{ Pod },
  format::{ ChannelType, Format, SurfaceType },
  pso::buffer::{ Element, ElemStride }
};
use nalgebra_glm::{ translation2d, scaling2d };

use super::screen_rect::{ 
  Transform2D, ScreenRect //, TextureForScreenItem
};

const VERT_SHADER: &[u8] = include_bytes!("../../../resources/shaders/screen_space/vert.glsl");
const FRAG_SHADER: &[u8] = include_bytes!("../../../resources/shaders/screen_space/frag.glsl");

pub struct ScreenSpacePass {
  buffer: Option<Buffer<Resources, i32>>
}

impl Default for ScreenSpacePass {
  fn default() -> Self {
    ScreenSpacePass {
      buffer: None
    }
  } 
}


struct Position;

impl Attribute for Position{
  const NAME: &'static str = "positionId";
  const FORMAT: Format = Format(SurfaceType::R32, ChannelType::Int);
  const SIZE: u32 = 4;
  type Repr = [i32; 1];
}

#[repr(C)]
struct SSPassAttributes {
  positionId: [i32; 1],
}

unsafe impl Pod for SSPassAttributes {}

impl VertexFormat for SSPassAttributes {
  const ATTRIBUTES: Attributes<'static> = &[
    (Position::NAME, <Self as With<Position>>::FORMAT)
  ];
}

impl With<Position> for SSPassAttributes {
  const FORMAT: AttributeFormat = Element{
    offset: 0,
    format: <Position as Attribute>::FORMAT
  };
}

impl<'a> PassData<'a> for ScreenSpacePass {
  type Data = (
    ReadStorage<'a, ScreenRect>,
    Read<'a, AssetStorage<Texture>>,
    ReadStorage<'a, TextureHandle>,
    ReadStorage<'a, Transform2D>,
    Option<Read<'a, ScreenDimensions>>
  );
}

impl Pass for ScreenSpacePass {
  fn compile(&mut self, effect: NewEffect<'_>) -> Result<Effect> {

    effect
      .simple(VERT_SHADER, FRAG_SHADER)
      .with_primitive_type(Primitive::TriangleList)
      .with_texture("sprite")
      .with_raw_global("transform2D")
      .with_raw_global("projection")
      .with_raw_global("uv_flip")
      .with_raw_global("tile_coords")
      .with_raw_global("screen_size")
      .with_raw_vertex_buffer(
        SSPassAttributes::ATTRIBUTES,
        SSPassAttributes::size() as ElemStride,
        0
      )
      .with_blended_output("color", ColorMask::all(), blend::ALPHA, None)
      .build()
  }

  fn apply <'a:, 'b: 'a> (
    &'a mut self,
    encoder: &mut Encoder,
    effect: &mut Effect,
    mut factory: Factory,
    (
      screen_rect_storage,
      texture_storage,
      texture_handle_storage,
      transform2D_storage,
      screen_dimensions
    ): <Self as PassData<'a>>::Data
  ) {
    use gfx::{
      memory::{ Typed }
    };
    
    let buffer = self.buffer.get_or_insert_with(|| init_buffer(&mut factory));

    let dimensions: [f32; 2] = match screen_dimensions {
      None => [0.0, 0.0],
      Some(ref sd) => [sd.width(), sd.height()]
    };
    let projection_matrix: [[f32; 3]; 3] = prepare2d_projection(dimensions[0], dimensions[1]).into();

    for (screen_rect, texture_handle, transform) in  (&screen_rect_storage, &texture_handle_storage, &transform2D_storage).join() {
      let p: [f32; 4] = [
        screen_rect.position.x,
        screen_rect.position.y,
        screen_rect.size.x,
        screen_rect.size.y,
      ];
      let mut flip: [i32; 2] = [0, 0];
      if screen_rect.flip_x {
        flip[0] = 1;
      };
      if screen_rect.flip_y {
        flip[1] = 1;
      };
      let tr: [[f32;3]; 3] = transform.model.into();
      effect.update_global("uv_flip", flip);
      effect.update_global("transform2D", tr);
      effect.update_global("projection", projection_matrix);
      effect.update_global("tile_coords", p);
      effect.update_global("screen_size", dimensions);
      let len = buffer.len();
      effect.data.vertex_bufs.push(buffer.raw().clone());
      match texture_storage.get(texture_handle) {
        None => continue,
        Some(tex) => {
          effect.data.textures.push(tex.view().clone());
          effect.data.samplers.push(tex.sampler().clone());
        }
      }

      effect.draw(
        &Slice {
          start: 0,
          end: len as u32,
          base_vertex: 0,
          instances: Some((2, 0)),
          buffer: Default::default()
        },
        encoder
      );
    }
  }
}
 
fn init_buffer(factory: &mut Factory) -> Buffer<Resources, i32> {
  use gfx::{
    buffer,
    memory::{ Bind },
    Factory
  };
  let arr = vec!(
    0, 1, 2, 0, 2, 3,
    // 0., 1., 2., 0., 2., 3.,
    /*

    -0.9, -0.9, 0.0, //1
    0.9, -0.9, 1.0,  //2
    0.9, 0.9, 2.0,  //3

    -0.9, -0.9, 0.0, //1
     0.9, 0.9, 2.0, //1
    -0.9, 0.9, 3.0,  //2
    */
    //0.9, -0.9, 10.0,  //2
  );
  let buffer = factory.create_buffer_immutable(
    &arr,
    buffer::Role::Vertex,
    Bind::empty()
    )
    .expect("Buffer must be created");
  buffer
} 

fn prepare2d_projection(width: f32, height: f32) -> Matrix3<f32> {
  let m = scaling2d(&Vector2::<f32>::new(2.0 / width, 2.0 / height));
  let m = translation2d(&Vector2::<f32>::new(-1.0, -1.0)) * m;
  m
}


