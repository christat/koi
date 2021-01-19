use ash::{extensions::khr::Surface, vk};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::window::WindowHandle,
    renderer::backend::vk::{handles::InstanceHandle, platform},
    utils::traits::Destroy,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct SurfaceHandle {
    pub surface: Surface,
    pub surface_khr: vk::SurfaceKHR,
}
//----------------------------------------------------------------------------------------------------------------------

impl SurfaceHandle {
    pub fn init(instance_handle: &InstanceHandle, window: &WindowHandle) -> Self {
        let InstanceHandle {
            entry, instance, ..
        } = instance_handle;

        Self {
            surface: Surface::new(entry, instance),
            surface_khr: platform::create_surface(&entry, &instance, &window),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl Destroy for SurfaceHandle {
    fn destroy(&mut self) {
        unsafe {
            self.surface.destroy_surface(self.surface_khr, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
