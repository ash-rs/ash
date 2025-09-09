impl<'a> From<ash::vk::DeviceCreateInfo<'a>> for crate::vk::DeviceCreateInfo<'a> {
    fn from(value: ash::vk::DeviceCreateInfo<'a>) -> Self {
        unsafe {
            core::mem::transmute::<ash::vk::DeviceCreateInfo<'a>, crate::vk::DeviceCreateInfo<'a>>(
                value,
            )
        }
    }
}

impl<'a> From<crate::vk::DeviceCreateInfo<'a>> for ash::vk::DeviceCreateInfo<'a> {
    fn from(value: crate::vk::DeviceCreateInfo<'a>) -> Self {
        unsafe {
            core::mem::transmute::<crate::vk::DeviceCreateInfo<'a>, ash::vk::DeviceCreateInfo<'a>>(
                value,
            )
        }
    }
}
