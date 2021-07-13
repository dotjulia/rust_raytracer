use crate::hitable::Hitable;
use crate::camera::Camera;
use dyn_clone::private::sync::Arc;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub world: Arc<dyn Hitable>,
    pub camera: Camera,
}

impl Scene {
    pub fn duplicate(&self) -> Scene {
        return Scene {
            width: self.width,
            height: self.height,
            world: self.world.duplicate(),
            camera: self.camera.duplicate(),
        }
    }
}