use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::{
    handles::{DeviceHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle},
    BackendConfig, DebugUtilsManager,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct RendererBackend {
    config: BackendConfig,
    instance_handle: InstanceHandle,

    #[cfg(debug_assertions)]
    debug_utils_manager: DebugUtilsManager,

    surface_handle: SurfaceHandle,
    physical_device_handle: PhysicalDeviceHandle,
    device_handle: DeviceHandle,
}
//----------------------------------------------------------------------------------------------------------------------

impl RendererBackend {
    pub fn init(app_name: &str, window: &WinitWindow) -> Self {
        info!("----- RendererBackend::init -----");

        let (instance_handle, config) = InstanceHandle::init(app_name);

        #[cfg(debug_assertions)]
        let debug_utils_manager = DebugUtilsManager::init(&instance_handle);

        let surface_handle = SurfaceHandle::init(&instance_handle, window);

        let physical_device_handle =
            PhysicalDeviceHandle::init(&instance_handle, &surface_handle, &config);

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
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
