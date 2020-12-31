use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::{
    handles::{DeviceHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle},
    DebugUtilsManager, VkBackendConfig,
};
use crate::renderer::frontend::hal::RendererBackend;
use crate::utils::traits::Cleanup;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkBackend {
    config: VkBackendConfig,
    instance_handle: InstanceHandle,

    #[cfg(debug_assertions)]
    debug_utils_manager: DebugUtilsManager,

    surface_handle: SurfaceHandle,
    physical_device_handle: PhysicalDeviceHandle,
    device_handle: DeviceHandle,

    frame_index: u32,
    pipelines_initialized: bool,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkBackend {
    pub fn init(app_name: &str, window: &WinitWindow) -> Self {
        info!("----- VkBackend::init -----");

        let (instance_handle, mut config) = InstanceHandle::init(app_name);

        #[cfg(debug_assertions)]
        let debug_utils_manager = DebugUtilsManager::init(&instance_handle);

        let surface_handle = SurfaceHandle::init(&instance_handle, window);

        let physical_device_handle =
            PhysicalDeviceHandle::init(&instance_handle, &surface_handle, &mut config);

        let device_handle = DeviceHandle::init(
            &instance_handle,
            &surface_handle,
            &physical_device_handle,
            &config,
            &window,
        );

        Self {
            config,
            instance_handle,

            #[cfg(debug_assertions)]
            debug_utils_manager,

            surface_handle,
            physical_device_handle,
            device_handle,

            frame_index: 0,
            pipelines_initialized: false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Drop for VkBackend {
    fn drop(&mut self) {
        self.device_handle.cleanup();
        self.surface_handle.cleanup();
        self.debug_utils_manager.cleanup();
        self.instance_handle.cleanup();
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl RendererBackend for VkBackend {
    fn draw(&mut self) {
        if !self.pipelines_initialized {
            self.device_handle.init_pipelines();
            self.pipelines_initialized = true;
        }

        self.device_handle.draw(self.frame_index);
        self.frame_index += 1;
    }

    fn await_device_idle(&mut self) {
        self.device_handle.await_idle();
    }

    fn swap_pipelines(&mut self) {
        self.device_handle.swap_pipelines();
    }

    fn load_mesh(&mut self) {
        unimplemented!()
    }
}
