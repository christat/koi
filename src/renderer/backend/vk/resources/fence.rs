use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::DeviceDestroy;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkFence {
    pub fence: vk::Fence,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkFence {
    pub(in crate::renderer::backend::vk::resources) fn new(
        device: &Device,
        flags: vk::FenceCreateFlags,
    ) -> Self {
        let create_info = vk::FenceCreateInfo::builder().flags(flags);

        let fence = unsafe {
            device
                .create_fence(&create_info, None)
                .expect("VkFence::new - Failed to create fence!")
        };

        Self { fence }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> &vk::Fence {
        &self.fence
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkFence {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_fence(self.fence, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
