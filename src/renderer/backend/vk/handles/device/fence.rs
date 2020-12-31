use ash::{
    version::DeviceV1_0,
    vk::{Fence, FenceCreateFlags, FenceCreateInfo},
    Device,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::handles::device::DeviceCleanup;
//----------------------------------------------------------------------------------------------------------------------

pub struct FenceHandle {
    pub render_fence: Fence,
}
//----------------------------------------------------------------------------------------------------------------------

impl FenceHandle {
    pub fn init(device: &Device) -> Self {
        let fence_create_info = FenceCreateInfo::builder().flags(FenceCreateFlags::SIGNALED);

        let render_fence = unsafe {
            device
                .create_fence(&fence_create_info, None)
                .expect("FenceHandle::init - Failed to create fence!")
        };

        Self { render_fence }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceCleanup for FenceHandle {
    fn cleanup(&mut self, device: &Device) {
        unsafe {
            device.destroy_fence(self.render_fence, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
