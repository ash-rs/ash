//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_info.html>

use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::amd::shader_info::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::amd::shader_info::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::amd::shader_info::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html>
    #[inline]
    pub unsafe fn get_shader_info(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
        info_type: vk::ShaderInfoTypeAMD,
    ) -> VkResult<ShaderInfoResult> {
        let load_data = |count: &mut usize, data: *mut u8| {
            (self.fp.get_shader_info_amd)(
                self.handle,
                pipeline,
                shader_stage,
                info_type,
                count,
                data.cast(),
            )
        };

        match info_type {
            vk::ShaderInfoTypeAMD::STATISTICS => {
                let mut statistics_info = mem::MaybeUninit::<vk::ShaderStatisticsInfoAMD>::uninit();
                load_data(
                    &mut mem::size_of_val(&statistics_info),
                    statistics_info.as_mut_ptr().cast(),
                )
                .result()?;
                Ok(ShaderInfoResult::StatisticsInfo(
                    statistics_info.assume_init(),
                ))
            }
            vk::ShaderInfoTypeAMD::BINARY => {
                read_into_uninitialized_vector(load_data).map(ShaderInfoResult::Binary)
            }
            vk::ShaderInfoTypeAMD::DISASSEMBLY => {
                read_into_uninitialized_vector(load_data).map(ShaderInfoResult::Disassembly)
            }
            #[cfg(feature = "debug")]
            x => unimplemented!("ShaderInfoTypeAMD {:?}", x),
            #[cfg(not(feature = "debug"))]
            x => unimplemented!("ShaderInfoTypeAMD {}", x.0),
        }
    }

    #[inline]
    pub fn fp(&self) -> &vk::amd::shader_info::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum ShaderInfoResult {
    StatisticsInfo(vk::ShaderStatisticsInfoAMD),
    Binary(Vec<u8>),
    Disassembly(Vec<u8>),
}
