use std::mem;
use vk_loader as vk;
use std::ptr;
use feature;
use surface;
use extensions::*;

#[derive(Clone)]
pub struct PhysicalDevice {
    pub ip: vk::InstancePointers,
    pub handle: vk::PhysicalDevice,
}

impl PhysicalDevice {
    pub fn get_surface_formats(&self, surface: surface::Surface) -> Vec<surface::SurfaceFormat> {
        unsafe {
            let mut num = mem::uninitialized();
            self.ip.GetPhysicalDeviceSurfaceFormatsKHR(self.handle,
                                                       surface.handle,
                                                       &mut num,
                                                       ptr::null_mut());
            let mut formats = Vec::with_capacity(num as usize);
            self.ip.GetPhysicalDeviceSurfaceFormatsKHR(self.handle,
                                                       surface.handle,
                                                       &mut num,
                                                       formats.as_mut_ptr());
            formats.set_len(num as usize);
            formats.into_iter()
                .map(|f| {
                    surface::SurfaceFormat {
                        format: surface::Format::from_number(f.format)
                            .expect("Unable to create a Format"),
                    }
                })
                .collect()
        }
    }

    pub fn has_surface_support(&self, index: u32, surface: &surface::Surface) -> bool {
        unsafe {
            let mut output: u32 = mem::uninitialized();
            self.ip.GetPhysicalDeviceSurfaceSupportKHR(self.handle,
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
            self.ip.GetPhysicalDeviceProperties(self.handle, &mut device_prop);
            device_prop
        }
    }

    pub fn get_queue_families(&self) -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            let mut queue_count = 0;
            self.ip
                .GetPhysicalDeviceQueueFamilyProperties(self.handle,
                                                        &mut queue_count,
                                                        ptr::null_mut());
            let mut queue_families_vec = Vec::with_capacity(queue_count as usize);
            self.ip
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
            self.ip.GetPhysicalDeviceMemoryProperties(self.handle, &mut output);
            output
        }
    }

    pub fn get_device_features(&self) -> feature::Features {
        let available_features: vk::PhysicalDeviceFeatures = unsafe {
            let mut output = mem::uninitialized();
            self.ip.GetPhysicalDeviceFeatures(self.handle, &mut output);
            output
        };
        feature::Features::from(available_features)
    }

    pub fn create_device(&self,
                         present: u32,
                         ext: &DeviceExtension,
                         features: &feature::Features)
                         -> Device {
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
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
            pEnabledFeatures: ptr::null(),
        };

        let mut vk_device = unsafe { mem::uninitialized() };
        unsafe {
            assert!(self.ip.CreateDevice(self.handle, &create_info, ptr::null(), &mut vk_device) ==
                    0,
                    "device");
        }
        let dp = vk::DevicePointers::load(|name| {
            unsafe { self.ip.GetDeviceProcAddr(vk_device, name.as_ptr()) as *const _ }
        });
        Device {
            dp: dp,
            device: vk_device,
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

pub struct Queue {
    handle: vk::Queue,
}

pub struct Device {
    dp: vk::DevicePointers,
    device: vk::Device,
}

impl Device {
    pub fn get_device_queue(&self, family: u32, index: u32) -> Queue {
        unsafe {
            let mut queue = mem::uninitialized();
            self.dp.GetDeviceQueue(self.device, family, index, &mut queue);
            Queue { handle: queue }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.dp.DestroyDevice(self.device, ptr::null());
        }
    }
}
