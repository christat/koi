use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

pub fn immediate_submit(
    device: &Device,
    command_pool: vk::CommandPool,
    fence: vk::Fence,
    queue: &vk::Queue,
    submit_op: &dyn Fn(&vk::CommandBuffer),
) {
    let allocate_command_buffers_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .command_buffer_count(1);

    let cmd = unsafe {
        device
            .allocate_command_buffers(&allocate_command_buffers_info)
            .expect("immediate_submit - Failed to allocate command buffer!")[0]
    };

    let cmd_begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe {
        device
            .begin_command_buffer(cmd, &cmd_begin_info)
            .expect("immediate_submit - Failed to begin command buffer!");
    }

    submit_op(&cmd);

    unsafe {
        device
            .end_command_buffer(cmd)
            .expect("immediate_submit - Failed to end command buffer!");
    }

    let submits = [vk::SubmitInfo::builder().command_buffers(&[cmd]).build()];

    unsafe {
        device
            .queue_submit(*queue, &submits, fence)
            .expect("immediate_submit - Failed to submit command buffer to queue!");
        device
            .wait_for_fences(&[fence], true, 9999999999)
            .expect("immediate_submit - Failed to wait for upload fence!");
        device
            .reset_fences(&[fence])
            .expect("immediate_submit - Failed to reset upload fence!");
        device
            .reset_command_pool(command_pool, vk::CommandPoolResetFlags::default())
            .expect("immediate_submit - Failed to reset command pool!");
    }
}
//----------------------------------------------------------------------------------------------------------------------
