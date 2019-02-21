extern crate gfx;

use std::collections::HashMap;
use std::marker::PhantomData;
use amethyst::{
  assets::{ AssetStorage },
  core::{
    specs::{ 
      prelude::{
        Join, Read, ReadStorage
      }
    },
    transform::{ Transform, GlobalTransform },
    nalgebra::Matrix4
  },
  renderer:: { 
    ActiveCamera,
    Camera,
    Projection,
    SpriteRender,
    SpriteSheet,
    Texture,
    SpriteSheetHandle,
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

use crate::tile_map::tile_map_component::TileMap;
use crate::camera_getter::get_camera;
use glsl_layout::*;


const VERT_SHADER: &[u8] = include_bytes!("../../resources/shaders/tile_map/vertex.glsl");
const GEOM_SHADER: &[u8] = include_bytes!("../../resources/shaders/tile_map/geom.glsl");
const FRAG_SHADER: &[u8] = include_bytes!("../../resources/shaders/tile_map/frag.glsl");
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


struct TileMapPositionAttribute;
struct TileMapOffset;
struct TileMapSliceSize;

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
struct CameraProperties {
  projection_matrix: mat4,
  view_matrix: mat4,
}

impl Default for CameraProperties {
  fn default() -> Self {
    let identity: [[f32; 4]; 4] = Matrix4::identity().into();
    CameraProperties {
      projection_matrix: identity.clone().into(),
      view_matrix: identity.into()
    }
  }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
struct TileMapArguments {
  tile_size: vec2,
  texture_size: vec2,
}
impl CameraProperties {
  /*
  fn new (c: (&Camera, &GlobalTransform)) -> Self {
    let (cam, trans) = c;
    let proj: [[f32; 4]; 4] = cam.proj.into();
    let view: [[f32; 4]; 4] = trans
      .0
      .try_inverse()
      .expect("view matrix cannot be inverted")
      .into();
    CameraProperties {
      projection_matrix: proj.into(),
      view_matrix: view.into()
    }
  }
  */
}

impl Attribute for TileMapOffset {
  const NAME: &'static str = "tile_map_offset";
  const FORMAT: Format  = Format(SurfaceType::R32_G32, ChannelType::Float);
  const SIZE: u32 = 8;
  type Repr = [f32; 2];
}
impl Attribute for TileMapSliceSize {
  const NAME: &'static str = "tile_map_slice_size";
  const FORMAT: Format  = Format(SurfaceType::R32_G32, ChannelType::Float);
  const SIZE: u32 = 8;
  type Repr = [f32; 2];
}

impl Attribute for TileMapPositionAttribute {
  const NAME: &'static str = "tile_map_position";
  const FORMAT: Format  = Format(SurfaceType::R32_G32_B32, ChannelType::Float);
  const SIZE: u32 = 12;
  type Repr = [f32; 3];
}

#[repr(C)]
struct TileMapAttributes {
  pub tile_map_position: [f32; 3],
  pub tile_map_offset: [f32; 2],
  pub tile_map_slice_size: [f32; 2],
}

unsafe impl Pod for TileMapPositionAttribute {}
unsafe impl Pod for TileMapOffset {}
unsafe impl Pod for TileMapSliceSize {}
unsafe impl Pod for TileMapAttributes {}

impl VertexFormat for TileMapAttributes {
  const ATTRIBUTES: Attributes<'static> = &[
    (TileMapPositionAttribute::NAME, <Self as With<TileMapPositionAttribute>>::FORMAT),
    (TileMapOffset::NAME, <Self as With<TileMapOffset>>::FORMAT),
    (TileMapSliceSize::NAME, <Self as With<TileMapSliceSize>>::FORMAT)
  ];
}

impl With<TileMapSliceSize> for TileMapAttributes {
  const FORMAT: AttributeFormat = Element {
    offset: TileMapPositionAttribute::SIZE + TileMapOffset::SIZE,
    format: <TileMapSliceSize as Attribute>::FORMAT
  };
}

impl With<TileMapOffset> for TileMapAttributes {
  const FORMAT: AttributeFormat = Element {
    offset: TileMapPositionAttribute::SIZE,
    format: <TileMapOffset as Attribute>::FORMAT
  };
}

impl With<TileMapPositionAttribute> for TileMapAttributes {
  const FORMAT: AttributeFormat = Element {
    offset: 0,
    format: <TileMapPositionAttribute as Attribute>::FORMAT
  };
}

type TileMapBuffer = (usize, Buffer<Resources, f32>);

pub struct TileMapPass<T> {
  buffers: HashMap<u64, TileMapBuffer>,
  _ph: PhantomData<T>
}

impl<T: 'static + Default + Sync + Eq + PartialEq> TileMapPass<T> {
  pub fn new() -> Self {
    TileMapPass {
      buffers: HashMap::new(),
      _ph: PhantomData
    }
  }

  fn init_buffer(
    &mut self, 
    tile_map: &TileMap<T>, 
    factory: &mut Factory
  ) -> TileMapBuffer {
    use gfx::{
      buffer,
      memory::{ Bind },
      Factory
    };
    
    let (instances, array) = tile_map.get_arrays_float_interleaved();

    let vertices_buf = factory
      .create_buffer_immutable(
        &array,
        buffer::Role::Vertex, 
        Bind::empty()
      )
      .expect("Buffer is not created");
    (instances, vertices_buf)
  }

  fn actual_draw<'a>(
    &self, 
    tile_map: &TileMap<T>,
    camera: Option<(&'a Camera, &'a GlobalTransform)>,
    vbs: &TileMapBuffer, 
    effect: &mut Effect, 
    encoder: &mut Encoder
  ) {

    use gfx::{
      memory::{ Typed }
    };
    let (instances, vb) = vbs;
    let sz = vb.len();
    let scale = tile_map.get_scale();
    let uniforms = TileMapArguments {
      tile_size: scale.clone().into(),
      texture_size: tile_map.get_texture_size().into()
    };

    effect.update_constant_buffer("TileMapArguments", &uniforms.std140(), encoder);
    set_camera_uniforms(effect, encoder, camera);

    for _ in TileMapAttributes::ATTRIBUTES {
      effect.data.vertex_bufs.push(vb.raw().clone());
    }

    effect.draw(
      &Slice {
        start: 0,
        end: sz as u32,
        base_vertex: 0,
        instances: Some((*instances as u32, 0)),
        buffer: Default::default()
      },
      encoder
    );
  }
}


impl<'a, T: 'static + Default + Send + Sync + Eq> PassData<'a> for TileMapPass<T> 
{
  type Data = (
    Option<Read<'a, ActiveCamera>>,
    ReadStorage<'a, Camera>,
    ReadStorage<'a, TileMap<T>>,
    Read<'a, AssetStorage<Texture>>,
    Read<'a, AssetStorage<SpriteSheet>>,
    ReadStorage<'a, SpriteRender>,
    ReadStorage<'a, GlobalTransform>
  );
}

impl<T: 'static + Default + Send + Sync + Eq> Pass for TileMapPass<T> {
  fn compile(&mut self, effect: NewEffect<'_>) -> Result<Effect> {
    use std::mem;

    effect 
      .geom(VERT_SHADER, GEOM_SHADER, FRAG_SHADER)
      .with_primitive_type(Primitive::PointList)
      .with_texture("tileSheet")
      .with_raw_constant_buffer(
        "CameraProperties",
        mem::size_of::<<CameraProperties as Uniform>::Std140>(),
        1
        )
      .with_raw_constant_buffer(
        "TileMapArguments",
        mem::size_of::<<TileMapArguments as Uniform>::Std140>(),
        1
        )
      .with_raw_vertex_buffer(
        TileMapAttributes::ATTRIBUTES, 
        TileMapAttributes::size() as ElemStride,
        1)
      .with_blended_output("color", ColorMask::all(), blend::ALPHA, None)
      .build()
  }


  fn apply<'a: , 'b: 'a>(
    &'a mut self, 
    encoder: &mut Encoder,
    effect: &mut Effect,
    mut factory: Factory,
  (
    active_camera,
    camera_storage,
    tile_map_storage,
    texture_storage,
     _spritesheet_storage,
     _sprite_render,
    global_transform_storage
  ): <Self as PassData<'a>>::Data
  ) {
    let camera = get_camera(
      active_camera,
      &camera_storage,
      &global_transform_storage
    );
    // effect.clear();
    for tile_map in (tile_map_storage).join() {  
      let texture = tile_map.get_texture(&texture_storage);
      match texture {
        None => {
          continue;
        }
        Some(texture) => {
          effect.data.textures.push(texture.view().clone());
          effect.data.samplers.push(texture.sampler().clone());
        }
      }

      let id = tile_map.calculate_hash();
      let svb = self.buffers.get(&id);
      match svb {
        None => {
          let vb = self.init_buffer(tile_map, &mut factory);
          self.actual_draw(tile_map, camera, &vb, effect, encoder); 
          self.buffers.insert(id, vb);
        }
        Some(vb) => {
          self.actual_draw(tile_map, camera, &vb, effect, encoder); 
        }
      }
    }
  }
}


fn set_camera_uniforms(
    effect: &mut Effect,
    encoder: &mut Encoder,
    camera: Option<(&Camera, &GlobalTransform)>,
) {
  let camera_properties = camera
    .as_ref()
    .map(|&(ref cam, ref transform)| {
      // let identity: [[f32; 4]; 4] = Matrix4::identity().into();
      // println!("projection_matrix: {}", cam.proj);
      // println!("view_matrix: {}", transform.0.try_inverse().unwrap());
      let proj: [[f32; 4]; 4] = cam.proj.into();
      let view: [[f32; 4]; 4] = transform
        .0
        .try_inverse()
        .expect("Unable to get inverse of camera transform")
        .into();
      CameraProperties {
        projection_matrix: proj.into(),
        view_matrix: view.into(),
      }
    })
    .unwrap_or_else(|| {
      let identity: [[f32; 4]; 4] = Matrix4::identity().into();
      CameraProperties {
        projection_matrix: identity.clone().into(),
        view_matrix: identity.into(),
      }
    });
  effect.update_constant_buffer("CameraProperties", &camera_properties.std140(), encoder);
}

