#![allow(dead_code)]
use std::ptr;
use std::ffi::*;

use std::error;
use std::fmt;
use std::mem;
use std::sync::Arc;
use std::os::raw::*;
use std::cell::Cell;
use std::path::Path;
use vk_loader2 as vk;
// use feature;
use load;
use shared_library::dynamic_library::DynamicLibrary;
type VkResult<T> = Result<T, vk::Result>;

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("vulkan-1.dll")
}

#[cfg(all(unix, not(target_os = "android")))]
fn get_path() -> &'static Path {
    Path::new("libvulkan.so.1")
}

#[cfg(target_os = "android")]
fn get_path() -> &'static Path {
    Path::new("libvulkan.so")
}

#[derive(Debug)]
pub struct Instance<'r> {
    handle: vk::Instance,
    instance_fn: vk::InstanceFn,
    _lifetime: ::std::marker::PhantomData<&'r i32>,
}

pub struct Entry {
    lib: DynamicLibrary,
    static_fn: vk::Static,
    entry_fn: vk::EntryFn,
}

#[derive(Debug)]
pub enum LoadingError {
    LibraryLoadFailure(String),
    StaticLoadError(String),
    EntryLoadError(String),
}

impl Entry {
    pub fn load_vulkan_path(path: &Path) -> Result<Entry, LoadingError> {
        let lib = try!(DynamicLibrary::open(Some(path))
            .map_err(|err| LoadingError::LibraryLoadFailure(err)));
        let static_fn = try!(vk::Static::load(|name| unsafe {
                let name = name.to_str().unwrap();
                let f = match lib.symbol(name) {
                    Ok(s) => s,
                    Err(_) => ptr::null(),
                };
                f
            })
            .map_err(|err| LoadingError::StaticLoadError(err)));
        let entry_fn = try!(vk::EntryFn::load(|name| unsafe {
                mem::transmute(static_fn.get_instance_proc_addr(ptr::null_mut(), name.as_ptr()))
            })
            .map_err(|err| LoadingError::EntryLoadError(err)));
        Ok(Entry {
            lib: lib,
            static_fn: static_fn,
            entry_fn: entry_fn,
        })
    }
    pub fn load_vulkan() -> Result<Entry, LoadingError> {
        Entry::load_vulkan_path(get_path())
    }


    pub fn create_instance<I: Into<vk::InstanceCreateInfo>>(&self,
                                                            i: I)
                                                            -> Result<Instance, vk::Result> {
        let create_info = i.into();
        unsafe {
            let mut instance: vk::Instance = mem::uninitialized();
            let err_code = self.entry_fn.create_instance(&create_info, ptr::null(), &mut instance);
            if err_code != vk::Result::Success {
                return Err(err_code);
            }
            let instance_fn = vk::InstanceFn::load(|name| unsafe {
                    mem::transmute(self.static_fn.get_instance_proc_addr(instance, name.as_ptr()))
                })
                .unwrap();
            Ok(Instance {
                handle: instance,
                instance_fn: instance_fn,
                _lifetime: ::std::marker::PhantomData,
            })
        }
    }

    pub fn enumerate_instance_layer_properties(&self)
                                               -> Result<Vec<vk::LayerProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.entry_fn.enumerate_instance_layer_properties(&mut num, ptr::null_mut());

            let mut v = Vec::with_capacity(num as usize);
            let err_code = self.entry_fn
                .enumerate_instance_layer_properties(&mut num, v.as_mut_ptr());
            v.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn enumerate_instance_extension_properties
        (&self)
         -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.entry_fn
                .enumerate_instance_extension_properties(ptr::null(), &mut num, ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.entry_fn
                .enumerate_instance_extension_properties(ptr::null(), &mut num, data.as_mut_ptr());
            data.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(data),
                _ => Err(err_code),
            }
        }
    }
}

pub struct Device {
    handle: vk::Device,
    device_fn: vk::DeviceFn,
}
impl Device {
    pub fn destroy_device(&self) {
        unsafe {
            self.device_fn.destroy_device(self.handle, ptr::null());
        }
    }

    pub fn free_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            self.device_fn.free_memory(self.handle, memory, ptr::null());
        }
    }

    pub fn destroy_fence(&self, fence: vk::Fence) {
        unsafe {
            self.device_fn.destroy_fence(self.handle, fence, ptr::null());
        }
    }

    pub fn destroy_image(&self, image: vk::Image) {
        unsafe {
            self.device_fn.destroy_image(self.handle, image, ptr::null());
        }
    }

    pub fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.device_fn.destroy_command_pool(self.handle, pool, ptr::null());
        }
    }

    pub fn destroy_swapchain_khr(&self, swapchain: vk::SwapchainKHR) {
        unsafe {
            self.device_fn.destroy_swapchain_khr(self.handle, swapchain, ptr::null());
        }
    }

    pub fn destroy_image_view(&self, image_view: vk::ImageView) {
        unsafe {
            self.device_fn.destroy_image_view(self.handle, image_view, ptr::null());
        }
    }

    pub fn destroy_render_pass(&self, renderpass: vk::RenderPass) {
        unsafe {
            self.device_fn.destroy_render_pass(self.handle, renderpass, ptr::null());
        }
    }

    pub fn destroy_framebuffer(&self, framebuffer: vk::Framebuffer) {
        unsafe {
            self.device_fn.destroy_framebuffer(self.handle, framebuffer, ptr::null());
        }
    }

    pub fn destroy_buffer(&self, buffer: vk::Buffer) {
        unsafe {
            self.device_fn.destroy_buffer(self.handle, buffer, ptr::null());
        }
    }

    pub fn destroy_shader_module(&self, shader: vk::ShaderModule) {
        unsafe {
            self.device_fn.destroy_shader_module(self.handle, shader, ptr::null());
        }
    }
    pub fn create_buffer(&self, create_info: &vk::BufferCreateInfo) -> VkResult<vk::Buffer> {
        unsafe {
            let mut buffer = mem::uninitialized();
            let err_code = self.device_fn
                .create_buffer(self.handle, create_info, ptr::null(), &mut buffer);
            match err_code {
                vk::Result::Success => Ok(buffer),
                _ => Err(err_code),
            }
        }
    }

    pub fn map_memory<T>(&self,
                         memory: vk::DeviceMemory,
                         offset: vk::DeviceSize,
                         size: vk::DeviceSize,
                         flags: vk::MemoryMapFlags)
                         -> VkResult<&mut [T]> {

        unsafe {
            let mut data: *mut () = mem::uninitialized();
            let err_code = self.device_fn
                .map_memory(self.handle, memory, offset, size, flags, &mut data);
            let x: *mut T = data as *mut T;
            match err_code {
                vk::Result::Success => {
                    Ok(::std::slice::from_raw_parts_mut(x, size as usize / mem::size_of::<T>()))
                }
                _ => Err(err_code),
            }
        }
    }

    pub fn unmap_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            self.device_fn.unmap_memory(self.handle, memory);
        }
    }

    pub fn create_framebuffer(&self,
                              create_info: &vk::FramebufferCreateInfo)
                              -> VkResult<vk::Framebuffer> {
        unsafe {
            let mut framebuffer = mem::uninitialized();
            let err_code = self.device_fn
                .create_framebuffer(self.handle, create_info, ptr::null(), &mut framebuffer);
            match err_code {
                vk::Result::Success => Ok(framebuffer),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        unsafe {
            let mut queue = mem::uninitialized();
            self.device_fn
                .get_device_queue(self.handle, queue_family_index, queue_index, &mut queue);
            queue
        }
    }

    pub fn cmd_pipeline_barrier(&self,
                                command_buffer: vk::CommandBuffer,
                                src_stage_mask: vk::PipelineStageFlags,
                                dst_stage_mask: vk::PipelineStageFlags,
                                dependency_flags: vk::DependencyFlags,
                                memory_barriers: &[vk::MemoryBarrier],
                                buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                                image_memory_barriers: &[vk::ImageMemoryBarrier]) {
        unsafe {
            self.device_fn.cmd_pipeline_barrier(command_buffer,
                                                src_stage_mask,
                                                dst_stage_mask,
                                                dependency_flags,
                                                memory_barriers.len() as u32,
                                                memory_barriers.as_ptr(),
                                                buffer_memory_barriers.len() as u32,
                                                buffer_memory_barriers.as_ptr(),
                                                image_memory_barriers.len() as u32,
                                                image_memory_barriers.as_ptr());
        }
    }

    pub fn create_render_pass(&self,
                              create_info: &vk::RenderPassCreateInfo)
                              -> VkResult<vk::RenderPass> {
        unsafe {
            let mut renderpass = mem::uninitialized();
            let err_code = self.device_fn
                .create_render_pass(self.handle, create_info, ptr::null(), &mut renderpass);
            match err_code {
                vk::Result::Success => Ok(renderpass),
                _ => Err(err_code),
            }
        }
    }

    pub fn begin_command_buffer(&self,
                                command_buffer: vk::CommandBuffer,
                                create_info: &vk::CommandBufferBeginInfo)
                                -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .begin_command_buffer(command_buffer, create_info);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .end_command_buffer(command_buffer);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn wait_for_fences(&self,
                           fences: &[vk::Fence],
                           wait_all: bool,
                           timeout: u64)
                           -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .wait_for_fences(self.handle,
                                 fences.len() as u32,
                                 fences.as_ptr(),
                                 wait_all as u32,
                                 timeout);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
    pub fn queue_submit(&self,
                        queue: vk::Queue,
                        submit_count: u32,
                        p_submits: &vk::SubmitInfo,
                        fence: vk::Fence)
                        -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn.queue_submit(queue, submit_count, p_submits, fence);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_image_view(&self,
                             create_info: &vk::ImageViewCreateInfo)
                             -> VkResult<vk::ImageView> {
        unsafe {
            let mut image_view = mem::uninitialized();
            let err_code = self.device_fn
                .create_image_view(self.handle, create_info, ptr::null(), &mut image_view);
            match err_code {
                vk::Result::Success => Ok(image_view),
                _ => Err(err_code),
            }
        }
    }
    pub fn get_swapchain_images_khr(&self,
                                    swapchain: vk::SwapchainKHR)
                                    -> VkResult<Vec<vk::Image>> {
        unsafe {
            let mut count = 0;
            self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, ptr::null_mut());

            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn allocate_command_buffers<I: Into<vk::CommandBufferAllocateInfo>>
        (&self,
         i: I)
         -> VkResult<Vec<vk::CommandBuffer>> {
        let create_info = i.into();
        unsafe {
            let mut buffers = Vec::with_capacity(create_info.command_buffer_count as usize);
            let err_code = self.device_fn
                .allocate_command_buffers(self.handle, &create_info, buffers.as_mut_ptr());
            buffers.set_len(create_info.command_buffer_count as usize);
            match err_code {
                vk::Result::Success => Ok(buffers),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_command_pool<I: Into<vk::CommandPoolCreateInfo>>(&self,
                                                                   i: I)
                                                                   -> VkResult<vk::CommandPool> {
        let create_info = i.into();
        unsafe {
            let mut pool = mem::uninitialized();
            let err_code = self.device_fn
                .create_command_pool(self.handle, &create_info, ptr::null(), &mut pool);
            match err_code {
                vk::Result::Success => Ok(pool),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_swapchain_khr<I: Into<vk::SwapchainCreateInfoKHR>>
        (&self,
         i: I)
         -> VkResult<vk::SwapchainKHR> {
        let create_info = i.into();
        unsafe {
            let mut swapchain = mem::uninitialized();
            let err_code = self.device_fn
                .create_swapchain_khr(self.handle, &create_info, ptr::null(), &mut swapchain);
            match err_code {
                vk::Result::Success => Ok(swapchain),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_image(&self, create_info: &vk::ImageCreateInfo) -> VkResult<vk::Image> {
        unsafe {
            let mut image = mem::uninitialized();
            let err_code = self.device_fn
                .create_image(self.handle, create_info, ptr::null(), &mut image);
            match err_code {
                vk::Result::Success => Ok(image),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.device_fn
                .get_image_memory_requirements(self.handle, image, &mut mem_req);
            mem_req
        }
    }

    pub fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.device_fn
                .get_buffer_memory_requirements(self.handle, buffer, &mut mem_req);
            mem_req
        }
    }

    pub fn allocate_memory(&self,
                           create_info: &vk::MemoryAllocateInfo)
                           -> VkResult<vk::DeviceMemory> {
        unsafe {
            let mut memory = mem::uninitialized();
            let err_code = self.device_fn
                .allocate_memory(self.handle, create_info, ptr::null(), &mut memory);
            match err_code {
                vk::Result::Success => Ok(memory),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_shader_module(&self,
                                create_info: &vk::ShaderModuleCreateInfo)
                                -> VkResult<vk::ShaderModule> {
        unsafe {
            let mut shader = mem::uninitialized();
            let err_code = self.device_fn
                .create_shader_module(self.handle, create_info, ptr::null(), &mut shader);
            match err_code {
                vk::Result::Success => Ok(shader),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_fence(&self, create_info: &vk::FenceCreateInfo) -> VkResult<vk::Fence> {
        unsafe {
            let mut fence = mem::uninitialized();
            let err_code = self.device_fn
                .create_fence(self.handle, create_info, ptr::null(), &mut fence);
            match err_code {
                vk::Result::Success => Ok(fence),
                _ => Err(err_code),
            }
        }
    }

    pub fn bind_buffer_memory(&self,
                              buffer: vk::Buffer,
                              device_memory: vk::DeviceMemory,
                              offset: vk::DeviceSize)
                              -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .bind_buffer_memory(self.handle, buffer, device_memory, offset);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
    pub fn bind_image_memory(&self,
                             image: vk::Image,
                             device_memory: vk::DeviceMemory,
                             offset: vk::DeviceSize)
                             -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .bind_image_memory(self.handle, image, device_memory, offset);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
}

impl<'r> Instance<'r> {
    pub fn destroy_instance(&self) {
        unsafe {
            self.instance_fn.destroy_instance(self.handle, ptr::null());
        }
    }

    pub fn get_physical_device_memory_properties(&self,
                                                 physical_device: vk::PhysicalDevice)
                                                 -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_prop = mem::uninitialized();
            self.instance_fn
                .get_physical_device_memory_properties(physical_device, &mut memory_prop);
            memory_prop
        }
    }

    pub fn get_physical_device_surface_present_modes_khr(&self,
                                                         physical_device: vk::PhysicalDevice,
                                                         surface: vk::SurfaceKHR)
                                                         -> VkResult<Vec<vk::PresentModeKHR>> {
        unsafe {
            let mut count = 0;
            self.instance_fn.get_physical_device_surface_present_modes_khr(physical_device,
                                                                           surface,
                                                                           &mut count,
                                                                           ptr::null_mut());
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.instance_fn
                .get_physical_device_surface_present_modes_khr(physical_device,
                                                               surface,
                                                               &mut count,
                                                               v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }
    pub fn get_physical_device_surface_capabilities_khr(&self,
                                                        physical_device: vk::PhysicalDevice,
                                                        surface: vk::SurfaceKHR)
                                                        -> VkResult<vk::SurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = mem::uninitialized();
            let err_code = self.instance_fn
                .get_physical_device_surface_capabilities_khr(physical_device,
                                                              surface,
                                                              &mut surface_capabilities);
            match err_code {
                vk::Result::Success => Ok(surface_capabilities),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_physical_device_surface_formats_khr(&self,
                                                   physical_device: vk::PhysicalDevice,
                                                   surface: vk::SurfaceKHR)
                                                   -> VkResult<Vec<vk::SurfaceFormatKHR>> {
        unsafe {
            let mut count = 0;
            self.instance_fn.get_physical_device_surface_formats_khr(physical_device,
                                                                     surface,
                                                                     &mut count,
                                                                     ptr::null_mut());
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.instance_fn
                .get_physical_device_surface_formats_khr(physical_device,
                                                         surface,
                                                         &mut count,
                                                         v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn destroy_surface_khr(&self, surface: vk::SurfaceKHR) {
        unsafe {
            self.instance_fn.destroy_surface_khr(self.handle, surface, ptr::null());
        }
    }
    pub fn create_xlib_surface_khr<I: Into<vk::XlibSurfaceCreateInfoKHR>>
        (&self,
         i: I)
         -> VkResult<vk::SurfaceKHR> {
        let create_info = i.into();
        unsafe {
            let mut surface = mem::uninitialized();
            let err_code = self.instance_fn
                .create_xlib_surface_khr(self.handle, &create_info, ptr::null(), &mut surface);
            match err_code {
                vk::Result::Success => Ok(surface),
                _ => Err(err_code),
            }
        }

    }
    pub fn get_physical_device_surface_support_khr(&self,
                                                   physical_device: vk::PhysicalDevice,
                                                   queue_index: u32,
                                                   surface: vk::SurfaceKHR)
                                                   -> bool {
        unsafe {
            let mut b = mem::uninitialized();
            self.instance_fn
                .get_physical_device_surface_support_khr(physical_device,
                                                         queue_index,
                                                         surface,
                                                         &mut b);
            b > 0
        }

    }
    pub fn get_physical_device_queue_family_properties(&self,
                                                       physical_device: vk::PhysicalDevice)
                                                       -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            let mut queue_count = 0;
            self.instance_fn
                .get_physical_device_queue_family_properties(physical_device,
                                                             &mut queue_count,
                                                             ptr::null_mut());
            let mut queue_families_vec = Vec::with_capacity(queue_count as usize);
            let err_code = self.instance_fn
                .get_physical_device_queue_family_properties(physical_device,
                                                             &mut queue_count,
                                                             queue_families_vec.as_mut_ptr());
            queue_families_vec.set_len(queue_count as usize);
            queue_families_vec
        }
    }

    pub fn create_device<I: Into<vk::DeviceCreateInfo>>(&self,
                                                        physical_device: vk::PhysicalDevice,
                                                        i: I)
                                                        -> VkResult<Device> {
        let create_info = i.into();
        unsafe {
            let mut device = mem::uninitialized();
            let err_code = self.instance_fn
                .create_device(physical_device, &create_info, ptr::null(), &mut device);
            if err_code != vk::Result::Success {
                return Err(err_code);
            }
            let device_fn = vk::DeviceFn::load(|name| unsafe {
                    mem::transmute(self.instance_fn.get_device_proc_addr(device, name.as_ptr()))
                })
                .unwrap();
            Ok(Device {
                handle: device,
                device_fn: device_fn,
            })
        }
    }

    pub fn enumerate_physical_devices(&self) -> VkResult<Vec<vk::PhysicalDevice>> {
        unsafe {
            let mut num = mem::uninitialized();
            self.instance_fn
                .enumerate_physical_devices(self.handle, &mut num, ptr::null_mut());
            let mut physical_devices = Vec::<vk::PhysicalDevice>::with_capacity(num as usize);
            let err_code = self.instance_fn
                .enumerate_physical_devices(self.handle, &mut num, physical_devices.as_mut_ptr());
            physical_devices.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(physical_devices),
                _ => Err(err_code),
            }
        }
    }

    pub fn enumerate_device_extension_properties
        (&self,
         device: vk::PhysicalDevice)
         -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.instance_fn
                .enumerate_device_extension_properties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.instance_fn
                .enumerate_device_extension_properties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       data.as_mut_ptr());
            data.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(data),
                _ => Err(err_code),
            }
        }
    }
}
