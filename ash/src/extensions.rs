use vk_loader as vk;
use std::ffi::{CString, CStr};
macro_rules! extensions {
    ($struct_name: ident, $struct_name_prop: ident, $($name: ident => $str_name: expr,)*) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(
                pub $name: bool,
            )+
        }
        impl $struct_name {
            pub fn empty() -> $struct_name {
                $struct_name{
                    $(
                        $name: false,
                    )+
                }
            }
            pub fn extension_list(&self) -> Vec<CString>{
                let mut vec = Vec::new();
                $(
                    if self.$name{
                        vec.push(CString::new($str_name).unwrap());
                    }
                )+
                vec
            }

            pub fn subset_of(&self, other: &$struct_name) -> bool{
                $(!self.$name | other.$name)&&+
            }
        }
        pub struct $struct_name_prop {
            pub ext_props: Vec<vk::ExtensionProperties>
        }

        impl From<$struct_name_prop> for $struct_name{
            fn from(ep: $struct_name_prop) -> $struct_name{
                let mut ext = $struct_name::empty();
                for ext_prop in ep.ext_props.iter() {
                    let name = unsafe{
                        CStr::from_ptr(ext_prop.extensionName.as_ptr()).to_str().unwrap()
                    };
                    $(
                        if name == $str_name {
                            ext.$name = true;
                        }
                    )+
                }
                ext
            }
        }
    }
}

extensions!{
    InstanceExtension,
    InstanceExtensionProperties,
    khr_surface => "VK_KHR_surface",
    khr_display => "VK_KHR_display",
    khr_xlib_surface => "VK_KHR_xlib_surface",
    khr_xcb_surface => "VK_KHR_xcb_surface",
    khr_wayland_surface => "VK_KHR_wayland_surface",
    khr_mir_surface => "VK_KHR_mir_surface",
    khr_android_surface => "VK_KHR_android_surface",
    khr_win32_surface => "VK_KHR_win32_surface",
    khr_ext_debug_report => "VK_EXT_debug_report",
}

extensions! {
    DeviceExtension,
    DeviceExtensionProperties,
    khr_swapchain => "VK_KHR_swapchain",
    khr_display_swapchain => "VK_KHR_display_swapchain",
}
