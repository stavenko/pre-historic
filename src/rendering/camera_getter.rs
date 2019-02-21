use amethyst::{
  renderer::{ ActiveCamera, Camera },
  core::transform::{ GlobalTransform },
  ecs::{ Join, ReadStorage, Read },

};


pub fn get_camera<'a>(
  active: Option<Read<'a, ActiveCamera>>,
  camera: &'a ReadStorage<'a, Camera>,
  global: &'a ReadStorage<'a, GlobalTransform>,
) -> Option<(&'a Camera, &'a GlobalTransform)> {
  active
    .and_then(|a| {
      let cam = camera.get(a.entity);
      let transform = global.get(a.entity);
      cam.into_iter().zip(transform.into_iter()).next()
    })
    .or_else(|| (camera, global).join().next())
}

