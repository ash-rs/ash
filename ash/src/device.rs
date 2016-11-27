use std::mem;
use fence;
use vk_loader as vk;
use std::ptr;
use feature;
use surface;
use extensions::*;
use commandpool;
use instance;
use std::sync::Arc;
use std::marker;
use std::ops::Drop;

#[derive(Clone)]
pub struct PhysicalDevice {
    pub instance: instance::Instance,
    pub handle: vk::PhysicalDevice,
}

impl PhysicalDevice {
    pub fn handle(&self) -> usize {
        self.handle
    }
    pub fn get_device_memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut props = mem::uninitialized();
            self.instance.ip().GetPhysicalDeviceMemoryProperties(self.handle(), &mut props);
            props
        }
    }
    pub fn get_surface_presentmodes(&self, surface: &surface::Surface) -> Vec<vk::PresentModeKHR> {
        unsafe {
            let mut surface_presentmode_count = 0;
            self.instance
                .inner
                .ip
                .GetPhysicalDeviceSurfacePresentModesKHR(self.handle,
                                                         surface.handle,
                                                         &mut surface_presentmode_count,
                                                         ptr::null_mut());
            let mut present_modes = Vec::with_capacity(surface_presentmode_count as usize);
            self.instance
                .inner
                .ip
                .GetPhysicalDeviceSurfacePresentModesKHR(self.handle,
                                                         surface.handle,
                                                         &mut surface_presentmode_count,
                                                         present_modes.as_mut_ptr());
            present_modes.set_len(surface_presentmode_count as usize);
            present_modes
        }
    }
    pub fn get_surface_capabilities(&self,
                                    surface: &surface::Surface)
                                    -> vk::SurfaceCapabilitiesKHR {
        unsafe {
            let mut capabilities = mem::uninitialized();
            self.instance.inner.ip.GetPhysicalDeviceSurfaceCapabilitiesKHR(self.handle,
                                                                           surface.handle,
                                                                           &mut capabilities);
            capabilities
        }
    }
    pub fn get_surface_formats(&self, surface: &surface::Surface) -> Vec<surface::SurfaceFormat> {
        unsafe {
            let mut num = mem::uninitialized();
            self.instance.inner.ip.GetPhysicalDeviceSurfaceFormatsKHR(self.handle,
                                                                      surface.handle,
                                                                      &mut num,
                                                                      ptr::null_mut());
            let mut formats = Vec::with_capacity(num as usize);
            self.instance.inner.ip.GetPhysicalDeviceSurfaceFormatsKHR(self.handle,
                                                                      surface.handle,
                                                                      &mut num,
                                                                      formats.as_mut_ptr());
            formats.set_len(num as usize);
            formats.into_iter()
                .map(|f| {
                    surface::SurfaceFormat {
                        format: surface::Format::from_number(f.format)
                            .expect("Unable to create a Format"),
                        color_space: surface::ColorSpace::from_number(f.colorSpace)
                            .expect("Unable to create a Format"),
                    }
                })
                .collect()
        }
    }

    pub fn has_surface_support(&self, index: u32, surface: &surface::Surface) -> bool {
        unsafe {
            let mut output: u32 = mem::uninitialized();
            self.instance.inner.ip.GetPhysicalDeviceSurfaceSupportKHR(self.handle,
                                                                      index,
                                                                      surface.handle,
                                                                      &mut output);
            output != 0
        }
    }

    pub fn get_physical_device_infos(&self) -> PhysicalDeviceInfos {
        PhysicalDeviceInfos {
            properties: self.get_physical_device_properties(),
            queue_families: self.get_queue_families(),
            memory: self.get_memory_properties(),
            features: self.get_device_features(),
        }
    }

    pub fn get_physical_device_properties(&self) -> vk::PhysicalDeviceProperties {
        unsafe {
            let mut device_prop: vk::PhysicalDeviceProperties = mem::uninitialized();
            self.instance.inner.ip.GetPhysicalDeviceProperties(self.handle, &mut device_prop);
            device_prop
        }
    }

    pub fn get_queue_families(&self) -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            let mut queue_count = 0;
            self.instance
                .inner
                .ip
                .GetPhysicalDeviceQueueFamilyProperties(self.handle,
                                                        &mut queue_count,
                                                        ptr::null_mut());
            let mut queue_families_vec = Vec::with_capacity(queue_count as usize);
            self.instance
                .inner
                .ip
                .GetPhysicalDeviceQueueFamilyProperties(self.handle,
                                                        &mut queue_count,
                                                        queue_families_vec.as_mut_ptr());
            queue_families_vec.set_len(queue_count as usize);
            queue_families_vec
        }
    }

    pub fn get_memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut output = mem::uninitialized();
            self.instance.inner.ip.GetPhysicalDeviceMemoryProperties(self.handle, &mut output);
            output
        }
    }

    pub fn get_device_features(&self) -> feature::Features {
        let available_features: vk::PhysicalDeviceFeatures = unsafe {
            let mut output = mem::uninitialized();
            self.instance.inner.ip.GetPhysicalDeviceFeatures(self.handle, &mut output);
            output
        };
        feature::Features::from(available_features)
    }

    pub fn create_device(&self,
                         present: u32,
                         ext: &DeviceExtension,
                         features: &feature::Features)
                         -> Device {
        let extension_list = ext.extension_list();
        let extension_list_raw =
            extension_list.iter().map(|extension| extension.as_ptr()).collect::<Vec<_>>();

        let f = vk::PhysicalDeviceFeatures::from(*features);
        let priorities = [1.0];
        let queue_info = vk::DeviceQueueCreateInfo {
            sType: vk::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueFamilyIndex: present,
            pQueuePriorities: priorities.as_ptr(),
            queueCount: priorities.len() as u32,
        };

        let create_info = vk::DeviceCreateInfo {
            sType: vk::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueCreateInfoCount: 1,
            pQueueCreateInfos: &queue_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: extension_list_raw.len() as u32,
            ppEnabledExtensionNames: extension_list_raw.as_ptr(),
            pEnabledFeatures: ptr::null(),
        };

        let mut vk_device = unsafe { mem::uninitialized() };
        unsafe {
            assert!(self.instance
                        .inner
                        .ip
                        .CreateDevice(self.handle, &create_info, ptr::null(), &mut vk_device) ==
                    0,
                    "device");
        }
        let dp = vk::DevicePointers::load(|name| {
            unsafe {
                self.instance.inner.ip.GetDeviceProcAddr(vk_device, name.as_ptr()) as *const _
            }
        });
        Device {
            inner: Arc::new(DeviceInner {
                dp: dp,
                device: vk_device,
            }),
        }
    }
}

pub struct PhysicalDeviceInfos {
    pub properties: vk::PhysicalDeviceProperties,
    pub queue_families: Vec<vk::QueueFamilyProperties>,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub features: feature::Features,
}

impl PhysicalDeviceInfos {
    fn has_surface_support(&self) -> bool {
        true
    }
}

pub trait QueueFamily {
    fn constrain(&self, queueFlags: u32) -> bool;
}

pub struct GraphicsQueueFamily;
impl QueueFamily for GraphicsQueueFamily {
    fn constrain(&self, queueFlags: u32) -> bool {
        queueFlags & vk::QUEUE_GRAPHICS_BIT != 0
    }
}

pub struct Queue<T: QueueFamily> {
    pub device: Device,
    pub handle: vk::Queue,
    pub family: u32,
    pub index: u32,
    pub _qf: marker::PhantomData<T>,
}

#[derive(Clone)]
pub struct Device {
    pub inner: Arc<DeviceInner>,
}

#[derive(Clone)]
pub struct DeviceInner {
    pub dp: vk::DevicePointers,
    pub device: vk::Device,
}

pub struct Swapchain {
    pub device: Device,
    pub handle: vk::SwapchainKHR,
}
impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.device
                .inner
                .dp
                .DestroySwapchainKHR(self.device.handle(), self.handle, ptr::null());
        }
    }
}

impl Swapchain {
    pub fn get_images(&self) -> Vec<vk::Image> {
        unsafe {
            let mut image_count = 0;
            self.device.dp().GetSwapchainImagesKHR(self.device.handle(),
                                                   self.handle,
                                                   &mut image_count,
                                                   ptr::null_mut());
            let mut images = Vec::with_capacity(image_count as usize);
            self.device.dp().GetSwapchainImagesKHR(self.device.handle(),
                                                   self.handle,
                                                   &mut image_count,
                                                   images.as_mut_ptr());
            images.set_len(image_count as usize);
            images
        }
    }
}
pub struct Image {
    pub device: Device,
    pub handle: vk::Image,
}
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.device.dp().DestroyImage(self.device.handle(), self.handle, ptr::null());
        }
    }
}
pub struct ImageView {
    pub device: Device,
    pub handle: vk::ImageView,
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe {
            self.device.dp().DestroyImageView(self.device.handle(), self.handle, ptr::null());
        }
    }
}

impl Device {
    pub fn handle(&self) -> usize {
        self.inner.device
    }

    pub fn dp(&self) -> &vk::DevicePointers {
        &self.inner.dp
    }
    pub fn cmd_pipeline_barrier<I: Into<vk::ImageMemoryBarrier>>(&self,
                                command_buffer: vk::CommandBuffer,
                                src_stage_mask: vk::PipelineStageFlags,
                                dst_stage_mask: vk::PipelineStageFlags,
                                dependecy_flags: vk::DependencyFlags,
                                memory_barrier_count: u32,
                                p_memory_barriers: *const vk::MemoryBarrier,
                                buffer_memory_barrier_count: u32,
                                p_buffer_memory_barriers: *const vk::BufferMemoryBarrier,
                                image_memory_barrier_count: u32,
                                p_image_memory_barriers: I) {
        let vk_p_image_memory_barriers = p_image_memory_barriers.into();
        unsafe {
            self.dp().CmdPipelineBarrier(command_buffer,
                                         src_stage_mask,
                                         dst_stage_mask,
                                         dependecy_flags,
                                         memory_barrier_count,
                                         p_memory_barriers,
                                         buffer_memory_barrier_count,
                                         p_buffer_memory_barriers,
                                         image_memory_barrier_count,
                                         &vk_p_image_memory_barriers);
        }
    }
    pub fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.dp().EndCommandBuffer(command_buffer);
        }
    }
    pub fn begin_command_buffer(&self,
                                command_buffer: vk::CommandBuffer,
                                begin_info: vk::CommandBufferBeginInfo) {
        unsafe {
            self.dp().BeginCommandBuffer(command_buffer, &begin_info);
        }
    }
    pub fn bind_image_memory(&self,
                             image: vk::Image,
                             memory: vk::DeviceMemory,
                             offset: vk::DeviceSize) {
        unsafe {
            self.dp().BindImageMemory(self.handle(), image, memory, offset);
        }

    }
    pub fn allocate_memory(&self,
                           mem_reqs: vk::MemoryRequirements,
                           mem_prop: vk::PhysicalDeviceMemoryProperties,
                           flags: vk::MemoryPropertyFlags)
                           -> Option<vk::DeviceMemory> {
        use buffer::find_memorytype_index;
        if let Some(index) = find_memorytype_index(&mem_reqs, &mem_prop, flags) {
            let create_info = vk::MemoryAllocateInfo {
                sType: vk::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                pNext: ptr::null(),
                allocationSize: mem_reqs.size,
                memoryTypeIndex: index,
            };

            unsafe {
                let mut device_memory = mem::uninitialized();
                self.dp()
                    .AllocateMemory(self.handle(), &create_info, ptr::null(), &mut device_memory);
                return Some(device_memory);
            }
        }
        None
    }
    pub fn get_image_memory_requirements(&self, image: &Image) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.dp().GetImageMemoryRequirements(self.handle(), image.handle, &mut mem_req);
            mem_req
        }
    }
    pub fn create_fence(&self) -> fence::Fence {
        unsafe {
            let create_info = vk::FenceCreateInfo {
                sType: vk::STRUCTURE_TYPE_FENCE_CREATE_INFO,
                flags: 0,
                pNext: ptr::null(),
            };
            let mut fence = mem::uninitialized();
            self.dp().CreateFence(self.handle(), &create_info, ptr::null(), &mut fence);

            fence::Fence {
                handle: fence,
                device: self.clone(),
            }
        }
    }
    pub fn create_image(&self,
                        flags: vk::ImageCreateFlags,
                        image_type: vk::ImageType,
                        format: vk::Format,
                        extent: vk::Extent3D,
                        mip_levels: u32,
                        array_layers: u32,
                        samples: vk::SampleCountFlagBits,
                        tiling: vk::ImageTiling,
                        usage: vk::ImageUsageFlags,
                        sharing_mode: vk::SharingMode,
                        queue_family_index_count: u32,
                        p_queue_family_indices: *const u32,
                        initial_layout: vk::ImageLayout)
                        -> Image {

        let create_info = vk::ImageCreateInfo {
            sType: vk::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags,
            imageType: image_type,
            format: format,
            extent: extent,
            mipLevels: mip_levels,
            arrayLayers: array_layers,
            samples: samples,
            tiling: tiling,
            usage: usage,
            sharingMode: sharing_mode,
            queueFamilyIndexCount: queue_family_index_count,
            pQueueFamilyIndices: p_queue_family_indices,
            initialLayout: initial_layout,
        };

        unsafe {
            let mut image = mem::uninitialized();
            self.dp().CreateImage(self.handle(), &create_info, ptr::null(), &mut image);
            Image {
                device: self.clone(),
                handle: image,
            }
        }
    }

    pub fn create_image_view(&self,
                             flags: vk::ImageViewCreateFlags,
                             image: vk::Image,
                             view_type: vk::ImageViewType,
                             format: vk::Format,
                             components: vk::ComponentMapping,
                             subresource_range: vk::ImageSubresourceRange)
                             -> ImageView {
        let create_info = vk::ImageViewCreateInfo {
            sType: vk::STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags,
            image: image,
            viewType: view_type,
            format: format,
            components: components,
            subresourceRange: subresource_range,
        };
        unsafe {
            let mut image_view = mem::uninitialized();
            self.dp().CreateImageView(self.handle(), &create_info, ptr::null(), &mut image_view);
            ImageView {
                device: self.clone(),
                handle: image_view,
            }
        }
    }
    pub fn create_swapchain(&self,
                            surface: &surface::Surface,
                            image_format: surface::Format,
                            min_image_count: u32,
                            image_color_space: surface::ColorSpace,
                            image_extent: vk::Extent2D,
                            image_array_layers: u32,
                            image_usage: vk::ImageUsageFlags,
                            image_sharing_mode: vk::SharingMode,
                            queue_family_index_count: u32,
                            p_queue_family_indices: *const u32,
                            pre_transform: vk::SurfaceTransformFlagBitsKHR,
                            composite_alpha: vk::CompositeAlphaFlagBitsKHR,
                            present_mode: vk::PresentModeKHR,
                            clipped: bool,
                            old_swapchain: vk::SwapchainKHR)
                            -> Swapchain {
        unsafe {
            let mut swapchain = mem::uninitialized();
            let create_info = vk::SwapchainCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,
                surface: surface.handle,
                minImageCount: min_image_count,
                imageFormat: image_format.to_number(),
                imageColorSpace: image_color_space.to_number(),
                imageExtent: image_extent,
                imageArrayLayers: image_array_layers,
                imageUsage: image_usage,
                imageSharingMode: image_sharing_mode,
                queueFamilyIndexCount: queue_family_index_count,
                pQueueFamilyIndices: p_queue_family_indices,
                preTransform: pre_transform,
                compositeAlpha: composite_alpha,
                presentMode: present_mode,
                clipped: clipped as u32,
                oldSwapchain: old_swapchain,
            };
            self.inner
                .dp
                .CreateSwapchainKHR(self.inner.device, &create_info, ptr::null(), &mut swapchain);
            Swapchain {
                device: self.clone(),
                handle: swapchain,
            }
        }
    }
    pub fn get_device_queue<T: QueueFamily>(&self, family: u32, index: u32) -> Queue<T> {
        unsafe {
            let mut queue = mem::uninitialized();
            self.inner.dp.GetDeviceQueue(self.inner.device, family, index, &mut queue);
            Queue {
                device: self.clone(),
                handle: queue,
                family: family,
                index: index,
                _qf: marker::PhantomData,
            }
        }
    }
    pub fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.dp().DestroyCommandPool(self.handle(), pool, ptr::null());
        }
    }

    pub fn allocate_command_buffers(&self,
                                    allocate_info: vk::CommandBufferAllocateInfo)
                                    -> Vec<vk::CommandBuffer> {
        unsafe {
            let mut command_buffers = Vec::with_capacity(allocate_info.commandBufferCount as usize);
            self.dp().AllocateCommandBuffers(self.handle(),
                                             &allocate_info,
                                             command_buffers.as_mut_ptr());
            command_buffers.set_len(allocate_info.commandBufferCount as usize);
            command_buffers
        }
    }

    pub fn create_commandpool(&self,
                              pool_info: vk::CommandPoolCreateInfo,
                              queue: vk::Queue)
                              -> vk::CommandPool {
        unsafe {
            let mut vk_cmd_pool = mem::uninitialized();
            self.dp()
                .CreateCommandPool(self.handle(), &pool_info, ptr::null(), &mut vk_cmd_pool);
            vk_cmd_pool
        }
    }

    // pub fn create_commandpool<T: QueueFamily>(&self, queue: &Queue<T>) -> commandpool::CommandPool {
    //    let create_info = vk::CommandPoolCreateInfo {
    //        pNext: ptr::null(),
    //        sType: vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    //        flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
    //        queueFamilyIndex: queue.index,
    //    };
    //    unsafe {
    //        let mut vk_cmd_pool = mem::uninitialized();
    //        self.inner.dp.CreateCommandPool(self.inner.device,
    //                                        &create_info,
    //                                        ptr::null(),
    //                                        &mut vk_cmd_pool);
    //        commandpool::CommandPool {
    //            device: self.clone(),
    //            pool: vk_cmd_pool,
    //        }
    //    }
    // }
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe {
            self.dp.DestroyDevice(self.device, ptr::null());
        }
    }
}
