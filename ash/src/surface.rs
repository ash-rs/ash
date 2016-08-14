use vk_loader as vk;
use glfw::*;
use std::mem;
use instance::Instance;
use std::ptr;
use std::os::raw::c_void;
pub trait VulkanSurface {
    fn create_surface(&self, inst: &Instance) -> vk::SurfaceKHR;
}

impl VulkanSurface for Window {
    fn create_surface(&self, inst: &Instance) -> vk::SurfaceKHR {
        unsafe {
            let x11_display = self.glfw.get_x11_display();
            let x11_window = self.get_x11_window();
            let mut surface: vk::SurfaceKHR = mem::uninitialized();
            let create_info = vk::XlibSurfaceCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,
                window: x11_window as *mut c_void,
                dpy: x11_display as *mut c_void,
            };
            inst.ip.CreateXlibSurfaceKHR(inst.instance, &create_info, ptr::null(), &mut surface);
            surface
        }
    }
}

pub struct SurfaceFormat {
   pub format: Format,
}

pub struct Surface<'r> {
    pub instance: &'r Instance,
    pub handle: vk::SurfaceKHR,
}
impl<'r> Drop for Surface<'r> {
    fn drop(&mut self) {
        unsafe {
            self.instance.ip.DestroySurfaceKHR(self.instance.instance, self.handle, ptr::null());
        }
    }
}

macro_rules! c_enum {
    (
        pub enum $name:ident: $base_ty:ty {
            $($var_name:ident = $var_value: path),+
            $(,)*
        }
    ) => {
        pub enum $name {
            $(
                $var_name = $var_value as isize,
            )+
        }
        impl $name {
            pub fn from_number(val: $base_ty) -> Option<$name>{
                match val {
                    $(
                        $var_value => Some($name::$var_name),
                    )+
                    _ => None
                }
            }
        }
    }
}
c_enum!{
    pub enum Format: u32 {
        FormatUndefined                = vk::FORMAT_UNDEFINED,
        FormatR4g4UnormPack8           = vk::FORMAT_R4G4_UNORM_PACK8,
        FormatR4g4b4a4UnormPack16      = vk::FORMAT_R4G4B4A4_UNORM_PACK16,
        FormatB4g4r4a4UnormPack16      = vk::FORMAT_B4G4R4A4_UNORM_PACK16,
        FormatR5g6b5UnormPack16        = vk::FORMAT_R5G6B5_UNORM_PACK16,
        FormatB5g6r5UnormPack16        = vk::FORMAT_B5G6R5_UNORM_PACK16,
        FormatR5g5b5a1UnormPack16      = vk::FORMAT_R5G5B5A1_UNORM_PACK16,
        FormatB5g5r5a1UnormPack16      = vk::FORMAT_B5G5R5A1_UNORM_PACK16,
        FormatA1r5g5b5UnormPack16      = vk::FORMAT_A1R5G5B5_UNORM_PACK16,
        FormatR8Unorm                  = vk::FORMAT_R8_UNORM,
        FormatR8Snorm                  = vk::FORMAT_R8_SNORM,
        FormatR8Uscaled                = vk::FORMAT_R8_USCALED,
        FormatR8Sscaled                = vk::FORMAT_R8_SSCALED,
        FormatR8Uint                   = vk::FORMAT_R8_UINT,
        FormatR8Sint                   = vk::FORMAT_R8_SINT,
        FormatR8Srgb                   = vk::FORMAT_R8_SRGB,
        FormatR8g8Unorm                = vk::FORMAT_R8G8_UNORM,
        FormatR8g8Snorm                = vk::FORMAT_R8G8_SNORM,
        FormatR8g8Uscaled              = vk::FORMAT_R8G8_USCALED,
        FormatR8g8Sscaled              = vk::FORMAT_R8G8_SSCALED,
        FormatR8g8Uint                 = vk::FORMAT_R8G8_UINT,
        FormatR8g8Sint                 = vk::FORMAT_R8G8_SINT,
        FormatR8g8Srgb                 = vk::FORMAT_R8G8_SRGB,
        FormatR8g8b8Unorm              = vk::FORMAT_R8G8B8_UNORM,
        FormatR8g8b8Snorm              = vk::FORMAT_R8G8B8_SNORM,
        FormatR8g8b8Uscaled            = vk::FORMAT_R8G8B8_USCALED,
        FormatR8g8b8Sscaled            = vk::FORMAT_R8G8B8_SSCALED,
        FormatR8g8b8Uint               = vk::FORMAT_R8G8B8_UINT,
        FormatR8g8b8Sint               = vk::FORMAT_R8G8B8_SINT,
        FormatR8g8b8Srgb               = vk::FORMAT_R8G8B8_SRGB,
        FormatB8g8r8Unorm              = vk::FORMAT_B8G8R8_UNORM,
        FormatB8g8r8Snorm              = vk::FORMAT_B8G8R8_SNORM,
        FormatB8g8r8Uscaled            = vk::FORMAT_B8G8R8_USCALED,
        FormatB8g8r8Sscaled            = vk::FORMAT_B8G8R8_SSCALED,
        FormatB8g8r8Uint               = vk::FORMAT_B8G8R8_UINT,
        FormatB8g8r8Sint               = vk::FORMAT_B8G8R8_SINT,
        FormatB8g8r8Srgb               = vk::FORMAT_B8G8R8_SRGB,
        FormatR8g8b8a8Unorm            = vk::FORMAT_R8G8B8A8_UNORM,
        FormatR8g8b8a8Snorm            = vk::FORMAT_R8G8B8A8_SNORM,
        FormatR8g8b8a8Uscaled          = vk::FORMAT_R8G8B8A8_USCALED,
        FormatR8g8b8a8Sscaled          = vk::FORMAT_R8G8B8A8_SSCALED,
        FormatR8g8b8a8Uint             = vk::FORMAT_R8G8B8A8_UINT,
        FormatR8g8b8a8Sint             = vk::FORMAT_R8G8B8A8_SINT,
        FormatR8g8b8a8Srgb             = vk::FORMAT_R8G8B8A8_SRGB,
        FormatB8g8r8a8Unorm            = vk::FORMAT_B8G8R8A8_UNORM,
        FormatB8g8r8a8Snorm            = vk::FORMAT_B8G8R8A8_SNORM,
        FormatB8g8r8a8Uscaled          = vk::FORMAT_B8G8R8A8_USCALED,
        FormatB8g8r8a8Sscaled          = vk::FORMAT_B8G8R8A8_SSCALED,
        FormatB8g8r8a8Uint             = vk::FORMAT_B8G8R8A8_UINT,
        FormatB8g8r8a8Sint             = vk::FORMAT_B8G8R8A8_SINT,
        FormatB8g8r8a8Srgb             = vk::FORMAT_B8G8R8A8_SRGB,
        FormatA8b8g8r8UnormPack32      = vk::FORMAT_A8B8G8R8_UNORM_PACK32,
        FormatA8b8g8r8SnormPack32      = vk::FORMAT_A8B8G8R8_SNORM_PACK32,
        FormatA8b8g8r8UscaledPack32    = vk::FORMAT_A8B8G8R8_USCALED_PACK32,
        FormatA8b8g8r8SscaledPack32    = vk::FORMAT_A8B8G8R8_SSCALED_PACK32,
        FormatA8b8g8r8UintPack32       = vk::FORMAT_A8B8G8R8_UINT_PACK32,
        FormatA8b8g8r8SintPack32       = vk::FORMAT_A8B8G8R8_SINT_PACK32,
        FormatA8b8g8r8SrgbPack32       = vk::FORMAT_A8B8G8R8_SRGB_PACK32,
        FormatA2r10g10b10UnormPack32   = vk::FORMAT_A2R10G10B10_UNORM_PACK32,
        FormatA2r10g10b10SnormPack32   = vk::FORMAT_A2R10G10B10_SNORM_PACK32,
        FormatA2r10g10b10UscaledPack32 = vk::FORMAT_A2R10G10B10_USCALED_PACK32,
        FormatA2r10g10b10SscaledPack32 = vk::FORMAT_A2R10G10B10_SSCALED_PACK32,
        FormatA2r10g10b10UintPack32    = vk::FORMAT_A2R10G10B10_UINT_PACK32,
        FormatA2r10g10b10SintPack32    = vk::FORMAT_A2R10G10B10_SINT_PACK32,
        FormatA2b10g10r10UnormPack32   = vk::FORMAT_A2B10G10R10_UNORM_PACK32,
        FormatA2b10g10r10SnormPack32   = vk::FORMAT_A2B10G10R10_SNORM_PACK32,
        FormatA2b10g10r10UscaledPack32 = vk::FORMAT_A2B10G10R10_USCALED_PACK32,
        FormatA2b10g10r10SscaledPack32 = vk::FORMAT_A2B10G10R10_SSCALED_PACK32,
        FormatA2b10g10r10UintPack32    = vk::FORMAT_A2B10G10R10_UINT_PACK32,
        FormatA2b10g10r10SintPack32    = vk::FORMAT_A2B10G10R10_SINT_PACK32,
        FormatR16Unorm                 = vk::FORMAT_R16_UNORM,
        FormatR16Snorm                 = vk::FORMAT_R16_SNORM,
        FormatR16Uscaled               = vk::FORMAT_R16_USCALED,
        FormatR16Sscaled               = vk::FORMAT_R16_SSCALED,
        FormatR16Uint                  = vk::FORMAT_R16_UINT,
        FormatR16Sint                  = vk::FORMAT_R16_SINT,
        FormatR16Sfloat                = vk::FORMAT_R16_SFLOAT,
        FormatR16g16Unorm              = vk::FORMAT_R16G16_UNORM,
        FormatR16g16Snorm              = vk::FORMAT_R16G16_SNORM,
        FormatR16g16Uscaled            = vk::FORMAT_R16G16_USCALED,
        FormatR16g16Sscaled            = vk::FORMAT_R16G16_SSCALED,
        FormatR16g16Uint               = vk::FORMAT_R16G16_UINT,
        FormatR16g16Sint               = vk::FORMAT_R16G16_SINT,
        FormatR16g16Sfloat             = vk::FORMAT_R16G16_SFLOAT,
        FormatR16g16b16Unorm           = vk::FORMAT_R16G16B16_UNORM,
        FormatR16g16b16Snorm           = vk::FORMAT_R16G16B16_SNORM,
        FormatR16g16b16Uscaled         = vk::FORMAT_R16G16B16_USCALED,
        FormatR16g16b16Sscaled         = vk::FORMAT_R16G16B16_SSCALED,
        FormatR16g16b16Uint            = vk::FORMAT_R16G16B16_UINT,
        FormatR16g16b16Sint            = vk::FORMAT_R16G16B16_SINT,
        FormatR16g16b16Sfloat          = vk::FORMAT_R16G16B16_SFLOAT,
        FormatR16g16b16a16Unorm        = vk::FORMAT_R16G16B16A16_UNORM,
        FormatR16g16b16a16Snorm        = vk::FORMAT_R16G16B16A16_SNORM,
        FormatR16g16b16a16Uscaled      = vk::FORMAT_R16G16B16A16_USCALED,
        FormatR16g16b16a16Sscaled      = vk::FORMAT_R16G16B16A16_SSCALED,
        FormatR16g16b16a16Uint         = vk::FORMAT_R16G16B16A16_UINT,
        FormatR16g16b16a16Sint         = vk::FORMAT_R16G16B16A16_SINT,
        FormatR16g16b16a16Sfloat       = vk::FORMAT_R16G16B16A16_SFLOAT,
        FormatR32Uint                  = vk::FORMAT_R32_UINT,
        FormatR32Sint                  = vk::FORMAT_R32_SINT,
        FormatR32Sfloat                = vk::FORMAT_R32_SFLOAT,
        FormatR32g32Uint               = vk::FORMAT_R32G32_UINT,
        FormatR32g32Sint               = vk::FORMAT_R32G32_SINT,
        FormatR32g32Sfloat             = vk::FORMAT_R32G32_SFLOAT,
        FormatR32g32b32Uint            = vk::FORMAT_R32G32B32_UINT,
        FormatR32g32b32Sint            = vk::FORMAT_R32G32B32_SINT,
        FormatR32g32b32Sfloat          = vk::FORMAT_R32G32B32_SFLOAT,
        FormatR32g32b32a32Uint         = vk::FORMAT_R32G32B32A32_UINT,
        FormatR32g32b32a32Sint         = vk::FORMAT_R32G32B32A32_SINT,
        FormatR32g32b32a32Sfloat       = vk::FORMAT_R32G32B32A32_SFLOAT,
        FormatR64Uint                  = vk::FORMAT_R64_UINT,
        FormatR64Sint                  = vk::FORMAT_R64_SINT,
        FormatR64Sfloat                = vk::FORMAT_R64_SFLOAT,
        FormatR64g64Uint               = vk::FORMAT_R64G64_UINT,
        FormatR64g64Sint               = vk::FORMAT_R64G64_SINT,
        FormatR64g64Sfloat             = vk::FORMAT_R64G64_SFLOAT,
        FormatR64g64b64Uint            = vk::FORMAT_R64G64B64_UINT,
        FormatR64g64b64Sint            = vk::FORMAT_R64G64B64_SINT,
        FormatR64g64b64Sfloat          = vk::FORMAT_R64G64B64_SFLOAT,
        FormatR64g64b64a64Uint         = vk::FORMAT_R64G64B64A64_UINT,
        FormatR64g64b64a64Sint         = vk::FORMAT_R64G64B64A64_SINT,
        FormatR64g64b64a64Sfloat       = vk::FORMAT_R64G64B64A64_SFLOAT,
        FormatB10g11r11UfloatPack32    = vk::FORMAT_B10G11R11_UFLOAT_PACK32,
        FormatE5b9g9r9UfloatPack32     = vk::FORMAT_E5B9G9R9_UFLOAT_PACK32,
        FormatD16Unorm                 = vk::FORMAT_D16_UNORM,
        FormatX8D24UnormPack32         = vk::FORMAT_X8_D24_UNORM_PACK32,
        FormatD32Sfloat                = vk::FORMAT_D32_SFLOAT,
        FormatS8Uint                   = vk::FORMAT_S8_UINT,
        FormatD16UnormS8Uint           = vk::FORMAT_D16_UNORM_S8_UINT,
        FormatD24UnormS8Uint           = vk::FORMAT_D24_UNORM_S8_UINT,
        FormatD32SfloatS8Uint          = vk::FORMAT_D32_SFLOAT_S8_UINT,
        FormatBc1RgbUnormBlock         = vk::FORMAT_BC1_RGB_UNORM_BLOCK,
        FormatBc1RgbSrgbBlock          = vk::FORMAT_BC1_RGB_SRGB_BLOCK,
        FormatBc1RgbaUnormBlock        = vk::FORMAT_BC1_RGBA_UNORM_BLOCK,
        FormatBc1RgbaSrgbBlock         = vk::FORMAT_BC1_RGBA_SRGB_BLOCK,
        FormatBc2UnormBlock            = vk::FORMAT_BC2_UNORM_BLOCK,
        FormatBc2SrgbBlock             = vk::FORMAT_BC2_SRGB_BLOCK,
        FormatBc3UnormBlock            = vk::FORMAT_BC3_UNORM_BLOCK,
        FormatBc3SrgbBlock             = vk::FORMAT_BC3_SRGB_BLOCK,
        FormatBc4UnormBlock            = vk::FORMAT_BC4_UNORM_BLOCK,
        FormatBc4SnormBlock            = vk::FORMAT_BC4_SNORM_BLOCK,
        FormatBc5UnormBlock            = vk::FORMAT_BC5_UNORM_BLOCK,
        FormatBc5SnormBlock            = vk::FORMAT_BC5_SNORM_BLOCK,
        FormatBc6hUfloatBlock          = vk::FORMAT_BC6H_UFLOAT_BLOCK,
        FormatBc6hSfloatBlock          = vk::FORMAT_BC6H_SFLOAT_BLOCK,
        FormatBc7UnormBlock            = vk::FORMAT_BC7_UNORM_BLOCK,
        FormatBc7SrgbBlock             = vk::FORMAT_BC7_SRGB_BLOCK,
        FormatEtc2R8g8b8UnormBlock     = vk::FORMAT_ETC2_R8G8B8_UNORM_BLOCK,
        FormatEtc2R8g8b8SrgbBlock      = vk::FORMAT_ETC2_R8G8B8_SRGB_BLOCK,
        FormatEtc2R8g8b8a1UnormBlock   = vk::FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK,
        FormatEtc2R8g8b8a1SrgbBlock    = vk::FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK,
        FormatEtc2R8g8b8a8UnormBlock   = vk::FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK,
        FormatEtc2R8g8b8a8SrgbBlock    = vk::FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK,
        FormatEacR11UnormBlock         = vk::FORMAT_EAC_R11_UNORM_BLOCK,
        FormatEacR11SnormBlock         = vk::FORMAT_EAC_R11_SNORM_BLOCK,
        FormatEacR11g11UnormBlock      = vk::FORMAT_EAC_R11G11_UNORM_BLOCK,
        FormatEacR11g11SnormBlock      = vk::FORMAT_EAC_R11G11_SNORM_BLOCK,
        FormatAstc4x4UnormBlock        = vk::FORMAT_ASTC_4x4_UNORM_BLOCK,
        FormatAstc4x4SrgbBlock         = vk::FORMAT_ASTC_4x4_SRGB_BLOCK,
        FormatAstc5x4UnormBlock        = vk::FORMAT_ASTC_5x4_UNORM_BLOCK,
        FormatAstc5x4SrgbBlock         = vk::FORMAT_ASTC_5x4_SRGB_BLOCK,
        FormatAstc5x5UnormBlock        = vk::FORMAT_ASTC_5x5_UNORM_BLOCK,
        FormatAstc5x5SrgbBlock         = vk::FORMAT_ASTC_5x5_SRGB_BLOCK,
        FormatAstc6x5UnormBlock        = vk::FORMAT_ASTC_6x5_UNORM_BLOCK,
        FormatAstc6x5SrgbBlock         = vk::FORMAT_ASTC_6x5_SRGB_BLOCK,
        FormatAstc6x6UnormBlock        = vk::FORMAT_ASTC_6x6_UNORM_BLOCK,
        FormatAstc6x6SrgbBlock         = vk::FORMAT_ASTC_6x6_SRGB_BLOCK,
        FormatAstc8x5UnormBlock        = vk::FORMAT_ASTC_8x5_UNORM_BLOCK,
        FormatAstc8x5SrgbBlock         = vk::FORMAT_ASTC_8x5_SRGB_BLOCK,
        FormatAstc8x6UnormBlock        = vk::FORMAT_ASTC_8x6_UNORM_BLOCK,
        FormatAstc8x6SrgbBlock         = vk::FORMAT_ASTC_8x6_SRGB_BLOCK,
        FormatAstc8x8UnormBlock        = vk::FORMAT_ASTC_8x8_UNORM_BLOCK,
        FormatAstc8x8SrgbBlock         = vk::FORMAT_ASTC_8x8_SRGB_BLOCK,
        FormatAstc10x5UnormBlock       = vk::FORMAT_ASTC_10x5_UNORM_BLOCK,
        FormatAstc10x5SrgbBlock        = vk::FORMAT_ASTC_10x5_SRGB_BLOCK,
        FormatAstc10x6UnormBlock       = vk::FORMAT_ASTC_10x6_UNORM_BLOCK,
        FormatAstc10x6SrgbBlock        = vk::FORMAT_ASTC_10x6_SRGB_BLOCK,
        FormatAstc10x8UnormBlock       = vk::FORMAT_ASTC_10x8_UNORM_BLOCK,
        FormatAstc10x8SrgbBlock        = vk::FORMAT_ASTC_10x8_SRGB_BLOCK,
        FormatAstc10x10UnormBlock      = vk::FORMAT_ASTC_10x10_UNORM_BLOCK,
        FormatAstc10x10SrgbBlock       = vk::FORMAT_ASTC_10x10_SRGB_BLOCK,
        FormatAstc12x10UnormBlock      = vk::FORMAT_ASTC_12x10_UNORM_BLOCK,
        FormatAstc12x10SrgbBlock       = vk::FORMAT_ASTC_12x10_SRGB_BLOCK,
        FormatAstc12x12UnormBlock      = vk::FORMAT_ASTC_12x12_UNORM_BLOCK,
        FormatAstc12x12SrgbBlock       = vk::FORMAT_ASTC_12x12_SRGB_BLOCK,
    }
}
