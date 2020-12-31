use crate::core::Window;
use crate::renderer::backend::vk::VkBackend;
use crate::renderer::{Renderer, RendererBackend};
//----------------------------------------------------------------------------------------------------------------------

pub fn init_vk(app_name: &str, window: &Window) -> Renderer<VkBackend> {
    Renderer::init(VkBackend::init(app_name, &window.window_handle))
}
//----------------------------------------------------------------------------------------------------------------------
