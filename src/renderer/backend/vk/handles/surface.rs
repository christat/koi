use ash::{extensions::khr::Surface, vk};
use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::{handles::InstanceHandle, platform};
use crate::utils::traits::Cleanup;
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

impl Cleanup for SurfaceHandle {
    fn cleanup(&mut self) {
        unsafe {
            self.surface.destroy_surface(self.surface_khr, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
