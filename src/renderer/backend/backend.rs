use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::{
    handles::{DeviceHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle},
    BackendConfig, DebugUtilsManager,
};
use crate::utils::traits::Cleanup;
//----------------------------------------------------------------------------------------------------------------------

pub struct RendererBackend {
    config: BackendConfig,
    instance_handle: InstanceHandle,

    #[cfg(debug_assertions)]
    debug_utils_manager: DebugUtilsManager,

    surface_handle: SurfaceHandle,
    physical_device_handle: PhysicalDeviceHandle,
    device_handle: DeviceHandle,

    frame_index: u32,
}
//----------------------------------------------------------------------------------------------------------------------

impl RendererBackend {
    pub fn init(app_name: &str, window: &WinitWindow) -> Self {
        info!("----- RendererBackend::init -----");

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
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn init_pipelines(&mut self) {
        self.device_handle.init_pipelines();
    }

    pub fn draw(&mut self) {
        self.device_handle.draw(self.frame_index);
        self.frame_index += 1;
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Drop for RendererBackend {
    fn drop(&mut self) {
        self.device_handle.cleanup();
        self.surface_handle.cleanup();
        self.debug_utils_manager.cleanup();
        self.instance_handle.cleanup();
    }
}
//----------------------------------------------------------------------------------------------------------------------
