use std::ptr;
//----------------------------------------------------------------------------------------------------------------------

use ash::{vk, Entry, Instance};
//----------------------------------------------------------------------------------------------------------------------

use crate::core::window::WindowHandle;
//----------------------------------------------------------------------------------------------------------------------

#[cfg(target_os = "windows")]
pub fn create_surface(entry: &Entry, instance: &Instance, window: &WindowHandle) -> vk::SurfaceKHR {
    use ash::extensions::khr::Win32Surface;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::platform::windows::WindowExtWindows;

    let win32_surface_loader = Win32Surface::new(entry, instance);

    let create_info = vk::Win32SurfaceCreateInfoKHR::builder()
        .hinstance(unsafe { GetModuleHandleW(ptr::null()) as vk::HINSTANCE })
        .hwnd(window.hwnd() as vk::HWND);

    unsafe {
        win32_surface_loader
            .create_win32_surface(&create_info, None)
            .expect("Failed to create win32 surface!")
    }
}

//----------------------------------------------------------------------------------------------------------------------

// TODO try targeting wayland as well?
#[cfg(target_os = "linux")]
pub fn create_surface(entry: &Entry, instance: &Instance, window: &WindowHandle) -> vk::SurfaceKHR {
    use ash::extensions::khr::XcbSurface;
    use winit::platform::unix::WindowExtUnix;

    let xcb_surface_loader = XcbSurface::new(entry, instance);

    // TODO
    let create_info = vk::XcbSurfaceCreateInfoKHR::builder();

    unsafe {
        xcb_surface_loader
            .create_xcb_surface(&create_info, None)
            .expect("Failed to create XCB surface!")
    }
}
