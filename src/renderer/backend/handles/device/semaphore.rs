use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::{handles::device::DeviceCleanup, BackendConfig};
//----------------------------------------------------------------------------------------------------------------------

pub struct SemaphoreHandle {
    pub present_semaphore: vk::Semaphore,
    pub render_semaphore: vk::Semaphore,
}
//----------------------------------------------------------------------------------------------------------------------

impl SemaphoreHandle {
    pub fn init(device: &Device, config: &BackendConfig) -> Self {
        let create_info = vk::SemaphoreCreateInfo::default();

        unsafe {
            let present_semaphore = device
                .create_semaphore(&create_info, None)
                .expect("SemaphoreHandle::init - Failed to create present semaphore!");

            let render_semaphore = device
                .create_semaphore(&create_info, None)
                .expect("SemaphoreHandle::init - Failed to create render semaphore!");

            Self {
                present_semaphore,
                render_semaphore,
            }
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceCleanup for SemaphoreHandle {
    fn cleanup(&mut self, device: &Device) {
        unsafe {
            device.destroy_semaphore(self.present_semaphore, None);
            device.destroy_semaphore(self.render_semaphore, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
