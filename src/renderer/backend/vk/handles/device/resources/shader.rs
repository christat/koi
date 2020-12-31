use std::{fs::File, io::Read, path::Path};
//----------------------------------------------------------------------------------------------------------------------

use ash::{
    version::DeviceV1_0,
    vk::{ShaderModule, ShaderModuleCreateInfo},
    Device,
};

extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::handles::device::DeviceCleanup;
//----------------------------------------------------------------------------------------------------------------------

pub struct ShaderResource {
    pub shader: ShaderModule,
}
//----------------------------------------------------------------------------------------------------------------------

impl ShaderResource {
    pub fn create(device: &Device, file_path: &Path) -> Self {
        Self {
            shader: create_shader_module(device, file_path),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceCleanup for ShaderResource {
    fn cleanup(&mut self, device: &Device) {
        unsafe {
            device.destroy_shader_module(self.shader, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn read_shader(file_path: &Path) -> Vec<u32> {
    let shader_file = File::open(file_path).expect(&format!(
        "VkBackend::ShaderResource - Failed to read shader: {}",
        file_path.display()
    ));

    let shader_bytes = shader_file
        .bytes()
        .filter_map(|byte| byte.ok())
        .collect::<Vec<u8>>();

    let shader_raw: Vec<u32> = (0..shader_bytes.len())
        .step_by(4)
        .fold(vec![], |mut acc, i| {
            acc.push(LittleEndian::read_u32(&shader_bytes[i..]));
            acc
        });

    shader_raw
}
//----------------------------------------------------------------------------------------------------------------------

fn create_shader_module(device: &Device, file_path: &Path) -> ShaderModule {
    let shader_raw = read_shader(file_path);

    let create_info = ShaderModuleCreateInfo::builder().code(&shader_raw).build();

    let shader_module = unsafe {
        device
            .create_shader_module(&create_info, None)
            .expect("VkBackend::ShaderResource - Failed to create shader module!")
    };

    info!(
        "VkBackend::ShaderResource - Successfully loaded shader {}",
        file_path.to_str().unwrap()
    );

    shader_module
}
//----------------------------------------------------------------------------------------------------------------------
