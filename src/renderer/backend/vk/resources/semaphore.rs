use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::DeviceDestroy;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkSemaphore {
    pub semaphore: vk::Semaphore,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkSemaphore {
    pub(in crate::renderer::backend::vk::resources) fn new(device: &Device) -> Self {
        let create_info = vk::SemaphoreCreateInfo::default();

        let semaphore = unsafe {
            device
                .create_semaphore(&create_info, None)
                .expect("VkSemaphore::new - Failed to create semaphore!")
        };

        Self { semaphore }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> vk::Semaphore {
        self.semaphore.clone()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkSemaphore {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_semaphore(self.semaphore, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
