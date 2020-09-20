use ash::{extensions::khr::Surface, vk};
use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::{handles::InstanceHandle, platform};
//----------------------------------------------------------------------------------------------------------------------

pub struct SurfaceHandle {
    pub surface: Surface,
    pub surface_khr: vk::SurfaceKHR,
}
//----------------------------------------------------------------------------------------------------------------------

impl SurfaceHandle {
    pub fn init(instance_handle: &InstanceHandle, window: &WinitWindow) -> Self {
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

impl Drop for SurfaceHandle {
    fn drop(&mut self) {
        unsafe {
            self.surface.destroy_surface(self.surface_khr, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
