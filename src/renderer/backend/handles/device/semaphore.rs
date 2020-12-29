use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::handles::device::DeviceCleanup;
use crate::renderer::backend::BackendConfig;
//----------------------------------------------------------------------------------------------------------------------

pub struct SemaphoreHandle {
    acquire_image_semaphores: Vec<vk::Semaphore>,
    render_complete_semaphores: Vec<vk::Semaphore>,
}
//----------------------------------------------------------------------------------------------------------------------

impl SemaphoreHandle {
    pub fn init(device: &Device, config: &BackendConfig) -> Self {
        let mut acquire_image_semaphores = vec![];
        let mut render_complete_semaphores = vec![];

        let create_info = vk::SemaphoreCreateInfo::default();

        for _ in (0..config.buffering).into_iter() {
            unsafe {
                acquire_image_semaphores.push(
                    device.create_semaphore(&create_info, None).expect(
                        "SemaphoreHandle::init - Failed to create acquire_image semaphore!",
                    ),
                );

                render_complete_semaphores.push(
                    device.create_semaphore(&create_info, None).expect(
                        "SemaphoreHandle::init - Failed to create render_complete semaphore!",
                    ),
                );
            }
        }

        Self {
            acquire_image_semaphores,
            render_complete_semaphores,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceCleanup for SemaphoreHandle {
    fn cleanup(&mut self, device: &Device) {
        unsafe {
            self.acquire_image_semaphores
                .iter()
                .for_each(|semaphore| device.destroy_semaphore(*semaphore, None));

            self.render_complete_semaphores
                .iter()
                .for_each(|semaphore| device.destroy_semaphore(*semaphore, None));
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
