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

use crate::{renderer::backend::vk::DeviceDestroy, utils::ffi};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkShader {
    module: ShaderModule,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkShader {
    pub(in crate::renderer::backend::vk::resources) fn new(device: &Device, path: &Path) -> Self {
        Self {
            module: create_shader_module(device, path),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> ShaderModule {
        self.module.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_default_shader_entry_point() -> ffi::CString {
        ffi::CString::new("main").unwrap()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkShader {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_shader_module(self.module, None);
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

fn create_shader_module(device: &Device, path: &Path) -> ShaderModule {
    let shader_raw = read_shader(path);

    let create_info = ShaderModuleCreateInfo::builder().code(&shader_raw).build();

    let shader_module = unsafe {
        device
            .create_shader_module(&create_info, None)
            .expect("VkBackend::ShaderResource - Failed to create shader module!")
    };

    info!(
        "VkBackend::ShaderResource - Successfully loaded shader {}",
        path.to_str().unwrap()
    );

    shader_module
}
//----------------------------------------------------------------------------------------------------------------------
