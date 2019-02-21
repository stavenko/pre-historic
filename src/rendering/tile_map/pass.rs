use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use gfx::{
  Primitive,
  Slice,
  pso::buffer::ElemStride,
  preset::blend,
  state::ColorMask,
  handle::{ Buffer, Sampler, RawTexture, RawShaderResourceView }
};
use amethyst::{
  assets::{ AssetStorage },
  ecs:: { ReadStorage, Read, Join },
  core::transform::GlobalTransform,
  core::nalgebra::{ Vector2, Vector3, Matrix2 },
  renderer::{ 
    VertexFormat,
    ActiveCamera, 
    Resources,
    Camera,
    Texture,
    Encoder,
    Factory,
    Effect,
    NewEffect,
    error::{ Result },
  },

  renderer::pipe::pass::{ Pass, PassData } 
};
use glsl_layout::*;
use nalgebra_glm as g;
extern crate shred;
extern crate shred_derive;
// #[macro_use]
use shred_derive::*;
const VERT_SHADER: &[u8] = include_bytes!("../../../resources/shaders/tile_map/vertex.glsl");
const GEOM_SHADER: &[u8] = include_bytes!("../../../resources/shaders/tile_map/geom.glsl");
const FRAG_SHADER: &[u8] = include_bytes!("../../../resources/shaders/tile_map/frag.glsl");

struct LocalTexture {
  sampler: Sampler<Resources>,
  texture: RawTexture<Resources>,
  view: RawShaderResourceView<Resources>
}
pub struct TileMapPass {
  last_render: LastRender,
  vertex_lookup_texture: Option<(f32, LocalTexture)>,
  render_buffer: Option<Buffer<Resources, i32>>
}

impl TileMapPass {
  pub fn new() -> Self {
    TileMapPass {
      last_render: LastRender::new(),
      render_buffer: None,
      vertex_lookup_texture: None
    }
  }
}

impl LastRender {
  fn new() -> Self {
    LastRender {
      amount: 0,
      hash: 0
    }
  }
}



use super::tile_map::{ TileMap, TextureInfo };
use super::tile::{ Tile, TileSprite };
use super::attrs::{ TileMapAttributes };
use super::camera::{ CameraProperties, set_camera_uniforms };
use crate::rendering::camera_getter::get_camera;

#[derive(Eq, PartialEq)]
pub struct LastRender {
  amount: usize,
  hash: u64
}


#[derive(SystemData)]
pub struct TileMapPassData<'a> {
  active_camera: Option<Read<'a, ActiveCamera>>,
  camera: ReadStorage<'a, Camera>,
  camera_transform: ReadStorage<'a, GlobalTransform>,
  tile_map: ReadStorage<'a, TileMap>,
  texture_info: ReadStorage<'a, TextureInfo>,
  tiles: ReadStorage<'a, Tile>,
  tile_sprites: ReadStorage<'a, TileSprite>,
  tex_assets: Read<'a, AssetStorage<Texture>>
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
struct TileMapArguments {
  tile_size: vec2,
  texture_size: vec2,
  basis: mat2,
}

impl<'a> PassData<'a> for TileMapPass {
  type Data = TileMapPassData<'a>;
}

fn to_v2<T: 'static +std::fmt::Debug+Copy+PartialEq>(f: &Vector2<T>) -> [T; 2] {
  [f.x, f.y]
}

impl Pass for TileMapPass {
  fn compile(&mut self, effect: NewEffect<'_>) -> Result<Effect> {
    use std::mem;

    effect 
      .geom(VERT_SHADER, GEOM_SHADER, FRAG_SHADER)
      .with_primitive_type(Primitive::PointList)
      .with_texture("tile_sheet")
      .with_texture("tile_props")
      .with_raw_global("tile_props_width")
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
        0)
      .with_blended_output("color", ColorMask::all(), blend::ALPHA, None)
      .build()
  }

  fn apply<'a:, 'b: 'a> (
    &'a mut self,
    encoder: &mut Encoder,
    effect: &mut Effect,
    mut factory: Factory,
    data: TileMapPassData<'a>
  ) {
    effect.clear();
    let camera = get_camera(
      data.active_camera,
      &data.camera, 
      &data.camera_transform
    );

    set_camera_uniforms(effect, encoder, camera);

    for (tile_map, texture) in (&data.tile_map, &data.texture_info).join() {
      let basis = tile_map.get_basis();
      let basis: [[f32; 2]; 2] = basis.into();
      let tm_args = TileMapArguments {
        tile_size: to_v2(&tile_map.scale).into(),
        texture_size: to_v2(&g::vec2(texture.size.x as f32, texture.size.y as f32)).into(),
        basis: basis.into()
      };
      let texture = match data.tex_assets.get(&texture.texture) {
        None => continue,
        Some(t) => t
      };
      effect.data.textures.push(texture.view().clone());
      effect.data.samplers.push(texture.sampler().clone());
      effect.update_constant_buffer("TileMapArguments", &tm_args.std140(), encoder);
      let mut render = LastRender::new();
      let mut hashier = DefaultHasher::new();
      let mut texture = Vec::<f32>::new();
      let mut buffer = Vec::<i32>::new();
      for (tile, sprite_info) in (&data.tiles, &data.tile_sprites).join() {
        tile.hash(&mut hashier);
        sprite_info.hash(&mut hashier);
        buffer.push(tile.position.x);
        buffer.push(tile.position.y);
        buffer.push(render.amount as i32);
        
        texture.push(sprite_info.offset.x);
        texture.push(sprite_info.offset.y);
        texture.push(sprite_info.size.x);
        texture.push(sprite_info.size.y);

        render.amount += 1;
      }
      render.hash = hashier.finish();
      if self.last_render != render {
        use gfx::{
          buffer,
          memory::{ Bind },
          Factory
        };

        self.render_buffer = match factory.create_buffer_immutable(
          &buffer,
          buffer::Role::Vertex,
          Bind::empty()
        ) {
          Ok(b) => Some(b),
          _ => None
        };
        self.vertex_lookup_texture = prepare_texture(texture, &mut factory);
        self.last_render = render;
      }

      let (tex_width, texture) = match &self.vertex_lookup_texture {
        None => continue,
        Some(t) => t
      };
      let buffer = match &self.render_buffer {
        None => continue,
        Some(b) => b
      };
      // let tex_width: [f32; 2] = [tex_width, tex_width];
      effect.update_global("tile_props_width", *tex_width);
      effect.data.textures.push(texture.view.clone());
      effect.data.samplers.push(texture.sampler.clone());

      use gfx::memory::Typed;
      effect.data.vertex_bufs.push(buffer.raw().clone());
      effect.draw(&Slice {
        start: 0,
        end: buffer.len() as u32,
        base_vertex: 0,
        instances: Some((self.last_render.amount as u32, 0)),
        buffer: Default::default()
      },
      encoder
    );
    }
  }
}

fn prepare_texture(data: Vec<f32>, factory: &mut Factory) -> Option<(f32, LocalTexture)> {
  opt(prepare_texture_res(data, factory))
}
fn prepare_texture_res(data: Vec<f32>, factory: &mut Factory) -> Result<(f32, LocalTexture) > {
  use gfx::{
    Factory,
    format::{ SurfaceType, ChannelType, Swizzle },
    memory::{ Bind, Usage, cast_slice },
    texture::{ SamplerInfo, FilterMethod, WrapMode, Kind, Mipmap, Info, ResourceDesc }
  };
  let w = (data.len() / 4) as u16;

  let tex = factory.create_texture_raw(
    Info {
      kind: Kind::D1(w),
      levels: 1,
      format: SurfaceType::R32_G32_B32_A32,
      bind: Bind::SHADER_RESOURCE,
      usage: Usage::Data,
    },
    Some(ChannelType::Float),
    Some((&[cast_slice(&data)], Mipmap::Provided))
  )?;

	let desc = ResourceDesc {
			channel: ChannelType::Float,
			layer: None,
			min: 1,
			max: 1,
			swizzle: Swizzle::new(),
	};

	let view = factory.view_texture_as_shader_resource_raw(&tex, desc)?;
  let sampler = SamplerInfo::new(FilterMethod::Scale, WrapMode::Clamp);
	let sampler = factory.create_sampler(sampler);

  Ok((w as f32, LocalTexture {
    sampler,
    texture: tex,
    view
  }))
}

fn opt<T>(r: Result<T>)-> Option<T> {
  match r {
    Ok(t) => Some(t),
    _ => None
  }
}

