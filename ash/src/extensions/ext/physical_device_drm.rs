use crate::vk;
use crate::Instance;
use std::ffi::CStr;

#[derive(Clone)]
pub struct PhysicalDeviceDrm;

impl PhysicalDeviceDrm {
    pub unsafe fn get_properties(
        instance: &Instance,
        pdevice: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceDrmPropertiesEXT {
        let mut props_drm = vk::PhysicalDeviceDrmPropertiesEXT::default();
        {
            let mut props = vk::PhysicalDeviceProperties2::default();
            props.p_next = &mut props_drm as *mut _ as *mut _;
            instance.get_physical_device_properties2(pdevice, &mut props);
        }
        props_drm
    }

    pub fn name() -> &'static CStr {
        vk::ExtPhysicalDeviceDrmFn::name()
    }
}
