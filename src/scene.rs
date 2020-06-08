use crate::hitable::Hitable;
use crate::camera::Camera;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub world: Box<dyn Hitable>,
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