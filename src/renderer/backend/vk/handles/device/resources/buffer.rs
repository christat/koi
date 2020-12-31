use ash::vk::Buffer;
use vk_mem::Allocation;
//----------------------------------------------------------------------------------------------------------------------

pub struct BufferResource {
    buffer: Buffer,
    allocation: Allocation,
}
//----------------------------------------------------------------------------------------------------------------------
//
// impl BufferResource {
//     pub fn allocate() -> Self {
//
//
//
//     }
// }
