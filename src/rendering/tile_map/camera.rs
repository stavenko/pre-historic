use amethyst::core::transform::GlobalTransform;
use amethyst::core::nalgebra::Matrix4;
use amethyst::renderer::{
  Encoder,
  Effect,
  Camera,
};
use glsl_layout::{ Uniform, mat4 };

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub struct CameraProperties {
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
pub fn set_camera_uniforms(
    effect: &mut Effect,
    encoder: &mut Encoder,
    camera: Option<(&Camera, &GlobalTransform)>,
) {
  let camera_properties = camera
    .as_ref()
    .map(|&(ref cam, ref transform)| {
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

