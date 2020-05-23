#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureBuildTypeKHR(pub(crate) i32);
impl AccelerationStructureBuildTypeKHR {
    pub const fn from_raw(x: i32) -> Self {
        AccelerationStructureBuildTypeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AccelerationStructureBuildTypeKHR {
    pub const HOST: Self = todo!();
    pub const DEVICE: Self = todo!();
    pub const HOST_OR_DEVICE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsTypeKHR(pub(crate) i32);
impl AccelerationStructureMemoryRequirementsTypeKHR {
    pub const fn from_raw(x: i32) -> Self {
        AccelerationStructureMemoryRequirementsTypeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AccelerationStructureMemoryRequirementsTypeKHR {
    pub const OBJECT: Self = todo!();
    pub const BUILD_SCRATCH: Self = todo!();
    pub const UPDATE_SCRATCH: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AccelerationStructureTypeKHR(pub(crate) i32);
impl AccelerationStructureTypeKHR {
    pub const fn from_raw(x: i32) -> Self {
        AccelerationStructureTypeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL: Self = todo!();
    pub const BOTTOM_LEVEL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentLoadOp(pub(crate) i32);
impl AttachmentLoadOp {
    pub const fn from_raw(x: i32) -> Self {
        AttachmentLoadOp(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AttachmentLoadOp {
    pub const LOAD: Self = todo!();
    pub const CLEAR: Self = todo!();
    pub const DONT_CARE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentStoreOp(pub(crate) i32);
impl AttachmentStoreOp {
    pub const fn from_raw(x: i32) -> Self {
        AttachmentStoreOp(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AttachmentStoreOp {
    pub const STORE: Self = todo!();
    pub const DONT_CARE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BlendFactor(pub(crate) i32);
impl BlendFactor {
    pub const fn from_raw(x: i32) -> Self {
        BlendFactor(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BlendFactor {
    pub const ZERO: Self = todo!();
    pub const ONE: Self = todo!();
    pub const SRC_COLOR: Self = todo!();
    pub const ONE_MINUS_SRC_COLOR: Self = todo!();
    pub const DST_COLOR: Self = todo!();
    pub const ONE_MINUS_DST_COLOR: Self = todo!();
    pub const SRC_ALPHA: Self = todo!();
    pub const ONE_MINUS_SRC_ALPHA: Self = todo!();
    pub const DST_ALPHA: Self = todo!();
    pub const ONE_MINUS_DST_ALPHA: Self = todo!();
    pub const CONSTANT_COLOR: Self = todo!();
    pub const ONE_MINUS_CONSTANT_COLOR: Self = todo!();
    pub const CONSTANT_ALPHA: Self = todo!();
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = todo!();
    pub const SRC_ALPHA_SATURATE: Self = todo!();
    pub const SRC1_COLOR: Self = todo!();
    pub const ONE_MINUS_SRC1_COLOR: Self = todo!();
    pub const SRC1_ALPHA: Self = todo!();
    pub const ONE_MINUS_SRC1_ALPHA: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BlendOp(pub(crate) i32);
impl BlendOp {
    pub const fn from_raw(x: i32) -> Self {
        BlendOp(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BlendOp {
    pub const ADD: Self = todo!();
    pub const SUBTRACT: Self = todo!();
    pub const REVERSE_SUBTRACT: Self = todo!();
    pub const MIN: Self = todo!();
    pub const MAX: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BlendOverlapEXT(pub(crate) i32);
impl BlendOverlapEXT {
    pub const fn from_raw(x: i32) -> Self {
        BlendOverlapEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BlendOverlapEXT {
    pub const UNCORRELATED: Self = todo!();
    pub const DISJOINT: Self = todo!();
    pub const CONJOINT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BorderColor(pub(crate) i32);
impl BorderColor {
    pub const fn from_raw(x: i32) -> Self {
        BorderColor(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = todo!();
    pub const INT_TRANSPARENT_BLACK: Self = todo!();
    pub const FLOAT_OPAQUE_BLACK: Self = todo!();
    pub const INT_OPAQUE_BLACK: Self = todo!();
    pub const FLOAT_OPAQUE_WHITE: Self = todo!();
    pub const INT_OPAQUE_WHITE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ChromaLocation(pub(crate) i32);
impl ChromaLocation {
    pub const fn from_raw(x: i32) -> Self {
        ChromaLocation(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ChromaLocation {
    pub const COSITED_EVEN: Self = todo!();
    pub const MIDPOINT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CoarseSampleOrderTypeNV(pub(crate) i32);
impl CoarseSampleOrderTypeNV {
    pub const fn from_raw(x: i32) -> Self {
        CoarseSampleOrderTypeNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CoarseSampleOrderTypeNV {
    pub const DEFAULT_NV: Self = todo!();
    pub const CUSTOM_NV: Self = todo!();
    pub const PIXEL_MAJOR_NV: Self = todo!();
    pub const SAMPLE_MAJOR_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ColorSpaceKHR(pub(crate) i32);
impl ColorSpaceKHR {
    pub const fn from_raw(x: i32) -> Self {
        ColorSpaceKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR: Self = todo!();
    pub const COLORSPACE_SRGB_NONLINEAR: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferLevel(pub(crate) i32);
impl CommandBufferLevel {
    pub const fn from_raw(x: i32) -> Self {
        CommandBufferLevel(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CommandBufferLevel {
    pub const PRIMARY: Self = todo!();
    pub const SECONDARY: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CompareOp(pub(crate) i32);
impl CompareOp {
    pub const fn from_raw(x: i32) -> Self {
        CompareOp(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CompareOp {
    pub const NEVER: Self = todo!();
    pub const LESS: Self = todo!();
    pub const EQUAL: Self = todo!();
    pub const LESS_OR_EQUAL: Self = todo!();
    pub const GREATER: Self = todo!();
    pub const NOT_EQUAL: Self = todo!();
    pub const GREATER_OR_EQUAL: Self = todo!();
    pub const ALWAYS: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ComponentSwizzle(pub(crate) i32);
impl ComponentSwizzle {
    pub const fn from_raw(x: i32) -> Self {
        ComponentSwizzle(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ComponentSwizzle {
    pub const IDENTITY: Self = todo!();
    pub const ZERO: Self = todo!();
    pub const ONE: Self = todo!();
    pub const R: Self = todo!();
    pub const G: Self = todo!();
    pub const B: Self = todo!();
    pub const A: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ComponentTypeNV(pub(crate) i32);
impl ComponentTypeNV {
    pub const fn from_raw(x: i32) -> Self {
        ComponentTypeNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ComponentTypeNV {
    pub const FLOAT16_NV: Self = todo!();
    pub const FLOAT32_NV: Self = todo!();
    pub const FLOAT64_NV: Self = todo!();
    pub const SINT8_NV: Self = todo!();
    pub const SINT16_NV: Self = todo!();
    pub const SINT32_NV: Self = todo!();
    pub const SINT64_NV: Self = todo!();
    pub const UINT8_NV: Self = todo!();
    pub const UINT16_NV: Self = todo!();
    pub const UINT32_NV: Self = todo!();
    pub const UINT64_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ConservativeRasterizationModeEXT(pub(crate) i32);
impl ConservativeRasterizationModeEXT {
    pub const fn from_raw(x: i32) -> Self {
        ConservativeRasterizationModeEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ConservativeRasterizationModeEXT {
    pub const DISABLED: Self = todo!();
    pub const OVERESTIMATE: Self = todo!();
    pub const UNDERESTIMATE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CopyAccelerationStructureModeKHR(pub(crate) i32);
impl CopyAccelerationStructureModeKHR {
    pub const fn from_raw(x: i32) -> Self {
        CopyAccelerationStructureModeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CopyAccelerationStructureModeKHR {
    pub const CLONE: Self = todo!();
    pub const COMPACT: Self = todo!();
    pub const SERIALIZE: Self = todo!();
    pub const DESERIALIZE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CoverageModulationModeNV(pub(crate) i32);
impl CoverageModulationModeNV {
    pub const fn from_raw(x: i32) -> Self {
        CoverageModulationModeNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CoverageModulationModeNV {
    pub const NONE_NV: Self = todo!();
    pub const RGB_NV: Self = todo!();
    pub const ALPHA_NV: Self = todo!();
    pub const RGBA_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CoverageReductionModeNV(pub(crate) i32);
impl CoverageReductionModeNV {
    pub const fn from_raw(x: i32) -> Self {
        CoverageReductionModeNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CoverageReductionModeNV {
    pub const MERGE_NV: Self = todo!();
    pub const TRUNCATE_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DebugReportObjectTypeEXT(pub(crate) i32);
impl DebugReportObjectTypeEXT {
    pub const fn from_raw(x: i32) -> Self {
        DebugReportObjectTypeEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DebugReportObjectTypeEXT {
    pub const UNKNOWN: Self = todo!();
    pub const INSTANCE: Self = todo!();
    pub const PHYSICAL_DEVICE: Self = todo!();
    pub const DEVICE: Self = todo!();
    pub const QUEUE: Self = todo!();
    pub const SEMAPHORE: Self = todo!();
    pub const COMMAND_BUFFER: Self = todo!();
    pub const FENCE: Self = todo!();
    pub const DEVICE_MEMORY: Self = todo!();
    pub const BUFFER: Self = todo!();
    pub const IMAGE: Self = todo!();
    pub const EVENT: Self = todo!();
    pub const QUERY_POOL: Self = todo!();
    pub const BUFFER_VIEW: Self = todo!();
    pub const IMAGE_VIEW: Self = todo!();
    pub const SHADER_MODULE: Self = todo!();
    pub const PIPELINE_CACHE: Self = todo!();
    pub const PIPELINE_LAYOUT: Self = todo!();
    pub const RENDER_PASS: Self = todo!();
    pub const PIPELINE: Self = todo!();
    pub const DESCRIPTOR_SET_LAYOUT: Self = todo!();
    pub const SAMPLER: Self = todo!();
    pub const DESCRIPTOR_POOL: Self = todo!();
    pub const DESCRIPTOR_SET: Self = todo!();
    pub const FRAMEBUFFER: Self = todo!();
    pub const COMMAND_POOL: Self = todo!();
    pub const SURFACE: Self = todo!();
    pub const SWAPCHAIN: Self = todo!();
    pub const DEBUG_REPORT_CALLBACK: Self = todo!();
    pub const DEBUG_REPORT: Self = todo!();
    pub const DISPLAY: Self = todo!();
    pub const DISPLAY_MODE: Self = todo!();
    pub const VALIDATION_CACHE: Self = todo!();
    pub const VALIDATION_CACHE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorType(pub(crate) i32);
impl DescriptorType {
    pub const fn from_raw(x: i32) -> Self {
        DescriptorType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DescriptorType {
    pub const SAMPLER: Self = todo!();
    pub const COMBINED_IMAGE_SAMPLER: Self = todo!();
    pub const SAMPLED_IMAGE: Self = todo!();
    pub const STORAGE_IMAGE: Self = todo!();
    pub const UNIFORM_TEXEL_BUFFER: Self = todo!();
    pub const STORAGE_TEXEL_BUFFER: Self = todo!();
    pub const UNIFORM_BUFFER: Self = todo!();
    pub const STORAGE_BUFFER: Self = todo!();
    pub const UNIFORM_BUFFER_DYNAMIC: Self = todo!();
    pub const STORAGE_BUFFER_DYNAMIC: Self = todo!();
    pub const INPUT_ATTACHMENT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateType(pub(crate) i32);
impl DescriptorUpdateTemplateType {
    pub const fn from_raw(x: i32) -> Self {
        DescriptorUpdateTemplateType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DeviceEventTypeEXT(pub(crate) i32);
impl DeviceEventTypeEXT {
    pub const fn from_raw(x: i32) -> Self {
        DeviceEventTypeEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DiscardRectangleModeEXT(pub(crate) i32);
impl DiscardRectangleModeEXT {
    pub const fn from_raw(x: i32) -> Self {
        DiscardRectangleModeEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DiscardRectangleModeEXT {
    pub const INCLUSIVE: Self = todo!();
    pub const EXCLUSIVE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DisplayEventTypeEXT(pub(crate) i32);
impl DisplayEventTypeEXT {
    pub const fn from_raw(x: i32) -> Self {
        DisplayEventTypeEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DisplayPowerStateEXT(pub(crate) i32);
impl DisplayPowerStateEXT {
    pub const fn from_raw(x: i32) -> Self {
        DisplayPowerStateEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DisplayPowerStateEXT {
    pub const OFF: Self = todo!();
    pub const SUSPEND: Self = todo!();
    pub const ON: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DriverId(pub(crate) i32);
impl DriverId {
    pub const fn from_raw(x: i32) -> Self {
        DriverId(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DriverId {
    pub const AMD_PROPRIETARY: Self = todo!();
    pub const AMD_OPEN_SOURCE: Self = todo!();
    pub const MESA_RADV: Self = todo!();
    pub const NVIDIA_PROPRIETARY: Self = todo!();
    pub const INTEL_PROPRIETARY_WINDOWS: Self = todo!();
    pub const INTEL_OPEN_SOURCE_MESA: Self = todo!();
    pub const IMAGINATION_PROPRIETARY: Self = todo!();
    pub const QUALCOMM_PROPRIETARY: Self = todo!();
    pub const ARM_PROPRIETARY: Self = todo!();
    pub const GOOGLE_SWIFTSHADER: Self = todo!();
    pub const GGP_PROPRIETARY: Self = todo!();
    pub const BROADCOM_PROPRIETARY: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DynamicState(pub(crate) i32);
impl DynamicState {
    pub const fn from_raw(x: i32) -> Self {
        DynamicState(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DynamicState {
    pub const VIEWPORT: Self = todo!();
    pub const SCISSOR: Self = todo!();
    pub const LINE_WIDTH: Self = todo!();
    pub const DEPTH_BIAS: Self = todo!();
    pub const BLEND_CONSTANTS: Self = todo!();
    pub const DEPTH_BOUNDS: Self = todo!();
    pub const STENCIL_COMPARE_MASK: Self = todo!();
    pub const STENCIL_WRITE_MASK: Self = todo!();
    pub const STENCIL_REFERENCE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Filter(pub(crate) i32);
impl Filter {
    pub const fn from_raw(x: i32) -> Self {
        Filter(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl Filter {
    pub const NEAREST: Self = todo!();
    pub const LINEAR: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Format(pub(crate) i32);
impl Format {
    pub const fn from_raw(x: i32) -> Self {
        Format(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl Format {
    pub const UNDEFINED: Self = todo!();
    pub const R4G4_UNORM_PACK8: Self = todo!();
    pub const R4G4B4A4_UNORM_PACK16: Self = todo!();
    pub const B4G4R4A4_UNORM_PACK16: Self = todo!();
    pub const R5G6B5_UNORM_PACK16: Self = todo!();
    pub const B5G6R5_UNORM_PACK16: Self = todo!();
    pub const R5G5B5A1_UNORM_PACK16: Self = todo!();
    pub const B5G5R5A1_UNORM_PACK16: Self = todo!();
    pub const A1R5G5B5_UNORM_PACK16: Self = todo!();
    pub const R8_UNORM: Self = todo!();
    pub const R8_SNORM: Self = todo!();
    pub const R8_USCALED: Self = todo!();
    pub const R8_SSCALED: Self = todo!();
    pub const R8_UINT: Self = todo!();
    pub const R8_SINT: Self = todo!();
    pub const R8_SRGB: Self = todo!();
    pub const R8G8_UNORM: Self = todo!();
    pub const R8G8_SNORM: Self = todo!();
    pub const R8G8_USCALED: Self = todo!();
    pub const R8G8_SSCALED: Self = todo!();
    pub const R8G8_UINT: Self = todo!();
    pub const R8G8_SINT: Self = todo!();
    pub const R8G8_SRGB: Self = todo!();
    pub const R8G8B8_UNORM: Self = todo!();
    pub const R8G8B8_SNORM: Self = todo!();
    pub const R8G8B8_USCALED: Self = todo!();
    pub const R8G8B8_SSCALED: Self = todo!();
    pub const R8G8B8_UINT: Self = todo!();
    pub const R8G8B8_SINT: Self = todo!();
    pub const R8G8B8_SRGB: Self = todo!();
    pub const B8G8R8_UNORM: Self = todo!();
    pub const B8G8R8_SNORM: Self = todo!();
    pub const B8G8R8_USCALED: Self = todo!();
    pub const B8G8R8_SSCALED: Self = todo!();
    pub const B8G8R8_UINT: Self = todo!();
    pub const B8G8R8_SINT: Self = todo!();
    pub const B8G8R8_SRGB: Self = todo!();
    pub const R8G8B8A8_UNORM: Self = todo!();
    pub const R8G8B8A8_SNORM: Self = todo!();
    pub const R8G8B8A8_USCALED: Self = todo!();
    pub const R8G8B8A8_SSCALED: Self = todo!();
    pub const R8G8B8A8_UINT: Self = todo!();
    pub const R8G8B8A8_SINT: Self = todo!();
    pub const R8G8B8A8_SRGB: Self = todo!();
    pub const B8G8R8A8_UNORM: Self = todo!();
    pub const B8G8R8A8_SNORM: Self = todo!();
    pub const B8G8R8A8_USCALED: Self = todo!();
    pub const B8G8R8A8_SSCALED: Self = todo!();
    pub const B8G8R8A8_UINT: Self = todo!();
    pub const B8G8R8A8_SINT: Self = todo!();
    pub const B8G8R8A8_SRGB: Self = todo!();
    pub const A8B8G8R8_UNORM_PACK32: Self = todo!();
    pub const A8B8G8R8_SNORM_PACK32: Self = todo!();
    pub const A8B8G8R8_USCALED_PACK32: Self = todo!();
    pub const A8B8G8R8_SSCALED_PACK32: Self = todo!();
    pub const A8B8G8R8_UINT_PACK32: Self = todo!();
    pub const A8B8G8R8_SINT_PACK32: Self = todo!();
    pub const A8B8G8R8_SRGB_PACK32: Self = todo!();
    pub const A2R10G10B10_UNORM_PACK32: Self = todo!();
    pub const A2R10G10B10_SNORM_PACK32: Self = todo!();
    pub const A2R10G10B10_USCALED_PACK32: Self = todo!();
    pub const A2R10G10B10_SSCALED_PACK32: Self = todo!();
    pub const A2R10G10B10_UINT_PACK32: Self = todo!();
    pub const A2R10G10B10_SINT_PACK32: Self = todo!();
    pub const A2B10G10R10_UNORM_PACK32: Self = todo!();
    pub const A2B10G10R10_SNORM_PACK32: Self = todo!();
    pub const A2B10G10R10_USCALED_PACK32: Self = todo!();
    pub const A2B10G10R10_SSCALED_PACK32: Self = todo!();
    pub const A2B10G10R10_UINT_PACK32: Self = todo!();
    pub const A2B10G10R10_SINT_PACK32: Self = todo!();
    pub const R16_UNORM: Self = todo!();
    pub const R16_SNORM: Self = todo!();
    pub const R16_USCALED: Self = todo!();
    pub const R16_SSCALED: Self = todo!();
    pub const R16_UINT: Self = todo!();
    pub const R16_SINT: Self = todo!();
    pub const R16_SFLOAT: Self = todo!();
    pub const R16G16_UNORM: Self = todo!();
    pub const R16G16_SNORM: Self = todo!();
    pub const R16G16_USCALED: Self = todo!();
    pub const R16G16_SSCALED: Self = todo!();
    pub const R16G16_UINT: Self = todo!();
    pub const R16G16_SINT: Self = todo!();
    pub const R16G16_SFLOAT: Self = todo!();
    pub const R16G16B16_UNORM: Self = todo!();
    pub const R16G16B16_SNORM: Self = todo!();
    pub const R16G16B16_USCALED: Self = todo!();
    pub const R16G16B16_SSCALED: Self = todo!();
    pub const R16G16B16_UINT: Self = todo!();
    pub const R16G16B16_SINT: Self = todo!();
    pub const R16G16B16_SFLOAT: Self = todo!();
    pub const R16G16B16A16_UNORM: Self = todo!();
    pub const R16G16B16A16_SNORM: Self = todo!();
    pub const R16G16B16A16_USCALED: Self = todo!();
    pub const R16G16B16A16_SSCALED: Self = todo!();
    pub const R16G16B16A16_UINT: Self = todo!();
    pub const R16G16B16A16_SINT: Self = todo!();
    pub const R16G16B16A16_SFLOAT: Self = todo!();
    pub const R32_UINT: Self = todo!();
    pub const R32_SINT: Self = todo!();
    pub const R32_SFLOAT: Self = todo!();
    pub const R32G32_UINT: Self = todo!();
    pub const R32G32_SINT: Self = todo!();
    pub const R32G32_SFLOAT: Self = todo!();
    pub const R32G32B32_UINT: Self = todo!();
    pub const R32G32B32_SINT: Self = todo!();
    pub const R32G32B32_SFLOAT: Self = todo!();
    pub const R32G32B32A32_UINT: Self = todo!();
    pub const R32G32B32A32_SINT: Self = todo!();
    pub const R32G32B32A32_SFLOAT: Self = todo!();
    pub const R64_UINT: Self = todo!();
    pub const R64_SINT: Self = todo!();
    pub const R64_SFLOAT: Self = todo!();
    pub const R64G64_UINT: Self = todo!();
    pub const R64G64_SINT: Self = todo!();
    pub const R64G64_SFLOAT: Self = todo!();
    pub const R64G64B64_UINT: Self = todo!();
    pub const R64G64B64_SINT: Self = todo!();
    pub const R64G64B64_SFLOAT: Self = todo!();
    pub const R64G64B64A64_UINT: Self = todo!();
    pub const R64G64B64A64_SINT: Self = todo!();
    pub const R64G64B64A64_SFLOAT: Self = todo!();
    pub const B10G11R11_UFLOAT_PACK32: Self = todo!();
    pub const E5B9G9R9_UFLOAT_PACK32: Self = todo!();
    pub const D16_UNORM: Self = todo!();
    pub const X8_D24_UNORM_PACK32: Self = todo!();
    pub const D32_SFLOAT: Self = todo!();
    pub const S8_UINT: Self = todo!();
    pub const D16_UNORM_S8_UINT: Self = todo!();
    pub const D24_UNORM_S8_UINT: Self = todo!();
    pub const D32_SFLOAT_S8_UINT: Self = todo!();
    pub const BC1_RGB_UNORM_BLOCK: Self = todo!();
    pub const BC1_RGB_SRGB_BLOCK: Self = todo!();
    pub const BC1_RGBA_UNORM_BLOCK: Self = todo!();
    pub const BC1_RGBA_SRGB_BLOCK: Self = todo!();
    pub const BC2_UNORM_BLOCK: Self = todo!();
    pub const BC2_SRGB_BLOCK: Self = todo!();
    pub const BC3_UNORM_BLOCK: Self = todo!();
    pub const BC3_SRGB_BLOCK: Self = todo!();
    pub const BC4_UNORM_BLOCK: Self = todo!();
    pub const BC4_SNORM_BLOCK: Self = todo!();
    pub const BC5_UNORM_BLOCK: Self = todo!();
    pub const BC5_SNORM_BLOCK: Self = todo!();
    pub const BC6H_UFLOAT_BLOCK: Self = todo!();
    pub const BC6H_SFLOAT_BLOCK: Self = todo!();
    pub const BC7_UNORM_BLOCK: Self = todo!();
    pub const BC7_SRGB_BLOCK: Self = todo!();
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = todo!();
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = todo!();
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = todo!();
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = todo!();
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = todo!();
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = todo!();
    pub const EAC_R11_UNORM_BLOCK: Self = todo!();
    pub const EAC_R11_SNORM_BLOCK: Self = todo!();
    pub const EAC_R11G11_UNORM_BLOCK: Self = todo!();
    pub const EAC_R11G11_SNORM_BLOCK: Self = todo!();
    pub const ASTC_4x4_UNORM_BLOCK: Self = todo!();
    pub const ASTC_4x4_SRGB_BLOCK: Self = todo!();
    pub const ASTC_5x4_UNORM_BLOCK: Self = todo!();
    pub const ASTC_5x4_SRGB_BLOCK: Self = todo!();
    pub const ASTC_5x5_UNORM_BLOCK: Self = todo!();
    pub const ASTC_5x5_SRGB_BLOCK: Self = todo!();
    pub const ASTC_6x5_UNORM_BLOCK: Self = todo!();
    pub const ASTC_6x5_SRGB_BLOCK: Self = todo!();
    pub const ASTC_6x6_UNORM_BLOCK: Self = todo!();
    pub const ASTC_6x6_SRGB_BLOCK: Self = todo!();
    pub const ASTC_8x5_UNORM_BLOCK: Self = todo!();
    pub const ASTC_8x5_SRGB_BLOCK: Self = todo!();
    pub const ASTC_8x6_UNORM_BLOCK: Self = todo!();
    pub const ASTC_8x6_SRGB_BLOCK: Self = todo!();
    pub const ASTC_8x8_UNORM_BLOCK: Self = todo!();
    pub const ASTC_8x8_SRGB_BLOCK: Self = todo!();
    pub const ASTC_10x5_UNORM_BLOCK: Self = todo!();
    pub const ASTC_10x5_SRGB_BLOCK: Self = todo!();
    pub const ASTC_10x6_UNORM_BLOCK: Self = todo!();
    pub const ASTC_10x6_SRGB_BLOCK: Self = todo!();
    pub const ASTC_10x8_UNORM_BLOCK: Self = todo!();
    pub const ASTC_10x8_SRGB_BLOCK: Self = todo!();
    pub const ASTC_10x10_UNORM_BLOCK: Self = todo!();
    pub const ASTC_10x10_SRGB_BLOCK: Self = todo!();
    pub const ASTC_12x10_UNORM_BLOCK: Self = todo!();
    pub const ASTC_12x10_SRGB_BLOCK: Self = todo!();
    pub const ASTC_12x12_UNORM_BLOCK: Self = todo!();
    pub const ASTC_12x12_SRGB_BLOCK: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FrontFace(pub(crate) i32);
impl FrontFace {
    pub const fn from_raw(x: i32) -> Self {
        FrontFace(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = todo!();
    pub const CLOCKWISE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FullScreenExclusiveEXT(pub(crate) i32);
impl FullScreenExclusiveEXT {
    pub const fn from_raw(x: i32) -> Self {
        FullScreenExclusiveEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl FullScreenExclusiveEXT {
    pub const DEFAULT: Self = todo!();
    pub const ALLOWED: Self = todo!();
    pub const DISALLOWED: Self = todo!();
    pub const APPLICATION_CONTROLLED: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct GeometryTypeKHR(pub(crate) i32);
impl GeometryTypeKHR {
    pub const fn from_raw(x: i32) -> Self {
        GeometryTypeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl GeometryTypeKHR {
    pub const TRIANGLES: Self = todo!();
    pub const AABBS: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageLayout(pub(crate) i32);
impl ImageLayout {
    pub const fn from_raw(x: i32) -> Self {
        ImageLayout(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageLayout {
    pub const UNDEFINED: Self = todo!();
    pub const GENERAL: Self = todo!();
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = todo!();
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = todo!();
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = todo!();
    pub const SHADER_READ_ONLY_OPTIMAL: Self = todo!();
    pub const TRANSFER_SRC_OPTIMAL: Self = todo!();
    pub const TRANSFER_DST_OPTIMAL: Self = todo!();
    pub const PREINITIALIZED: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageTiling(pub(crate) i32);
impl ImageTiling {
    pub const fn from_raw(x: i32) -> Self {
        ImageTiling(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageTiling {
    pub const OPTIMAL: Self = todo!();
    pub const LINEAR: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageType(pub(crate) i32);
impl ImageType {
    pub const fn from_raw(x: i32) -> Self {
        ImageType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageType {
    pub const 1D: Self = todo!();
    pub const 2D: Self = todo!();
    pub const 3D: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageViewType(pub(crate) i32);
impl ImageViewType {
    pub const fn from_raw(x: i32) -> Self {
        ImageViewType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageViewType {
    pub const 1D: Self = todo!();
    pub const 2D: Self = todo!();
    pub const 3D: Self = todo!();
    pub const CUBE: Self = todo!();
    pub const 1D_ARRAY: Self = todo!();
    pub const 2D_ARRAY: Self = todo!();
    pub const CUBE_ARRAY: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IndexType(pub(crate) i32);
impl IndexType {
    pub const fn from_raw(x: i32) -> Self {
        IndexType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl IndexType {
    pub const UINT16: Self = todo!();
    pub const UINT32: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IndirectCommandsTokenTypeNV(pub(crate) i32);
impl IndirectCommandsTokenTypeNV {
    pub const fn from_raw(x: i32) -> Self {
        IndirectCommandsTokenTypeNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl IndirectCommandsTokenTypeNV {
    pub const SHADER_GROUP_NV: Self = todo!();
    pub const STATE_FLAGS_NV: Self = todo!();
    pub const INDEX_BUFFER_NV: Self = todo!();
    pub const VERTEX_BUFFER_NV: Self = todo!();
    pub const PUSH_CONSTANT_NV: Self = todo!();
    pub const DRAW_INDEXED_NV: Self = todo!();
    pub const DRAW_NV: Self = todo!();
    pub const DRAW_TASKS_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InternalAllocationType(pub(crate) i32);
impl InternalAllocationType {
    pub const fn from_raw(x: i32) -> Self {
        InternalAllocationType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl InternalAllocationType {
    pub const EXECUTABLE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct LineRasterizationModeEXT(pub(crate) i32);
impl LineRasterizationModeEXT {
    pub const fn from_raw(x: i32) -> Self {
        LineRasterizationModeEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl LineRasterizationModeEXT {
    pub const DEFAULT: Self = todo!();
    pub const RECTANGULAR: Self = todo!();
    pub const BRESENHAM: Self = todo!();
    pub const RECTANGULAR_SMOOTH: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct LogicOp(pub(crate) i32);
impl LogicOp {
    pub const fn from_raw(x: i32) -> Self {
        LogicOp(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl LogicOp {
    pub const CLEAR: Self = todo!();
    pub const AND: Self = todo!();
    pub const AND_REVERSE: Self = todo!();
    pub const COPY: Self = todo!();
    pub const AND_INVERTED: Self = todo!();
    pub const NO_OP: Self = todo!();
    pub const XOR: Self = todo!();
    pub const OR: Self = todo!();
    pub const NOR: Self = todo!();
    pub const EQUIVALENT: Self = todo!();
    pub const INVERT: Self = todo!();
    pub const OR_REVERSE: Self = todo!();
    pub const COPY_INVERTED: Self = todo!();
    pub const OR_INVERTED: Self = todo!();
    pub const NAND: Self = todo!();
    pub const SET: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MemoryOverallocationBehaviorAMD(pub(crate) i32);
impl MemoryOverallocationBehaviorAMD {
    pub const fn from_raw(x: i32) -> Self {
        MemoryOverallocationBehaviorAMD(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl MemoryOverallocationBehaviorAMD {
    pub const DEFAULT_AMD: Self = todo!();
    pub const ALLOWED_AMD: Self = todo!();
    pub const DISALLOWED_AMD: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ObjectType(pub(crate) i32);
impl ObjectType {
    pub const fn from_raw(x: i32) -> Self {
        ObjectType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ObjectType {
    pub const UNKNOWN: Self = todo!();
    pub const INSTANCE: Self = todo!();
    pub const PHYSICAL_DEVICE: Self = todo!();
    pub const DEVICE: Self = todo!();
    pub const QUEUE: Self = todo!();
    pub const SEMAPHORE: Self = todo!();
    pub const COMMAND_BUFFER: Self = todo!();
    pub const FENCE: Self = todo!();
    pub const DEVICE_MEMORY: Self = todo!();
    pub const BUFFER: Self = todo!();
    pub const IMAGE: Self = todo!();
    pub const EVENT: Self = todo!();
    pub const QUERY_POOL: Self = todo!();
    pub const BUFFER_VIEW: Self = todo!();
    pub const IMAGE_VIEW: Self = todo!();
    pub const SHADER_MODULE: Self = todo!();
    pub const PIPELINE_CACHE: Self = todo!();
    pub const PIPELINE_LAYOUT: Self = todo!();
    pub const RENDER_PASS: Self = todo!();
    pub const PIPELINE: Self = todo!();
    pub const DESCRIPTOR_SET_LAYOUT: Self = todo!();
    pub const SAMPLER: Self = todo!();
    pub const DESCRIPTOR_POOL: Self = todo!();
    pub const DESCRIPTOR_SET: Self = todo!();
    pub const FRAMEBUFFER: Self = todo!();
    pub const COMMAND_POOL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceConfigurationTypeINTEL(pub(crate) i32);
impl PerformanceConfigurationTypeINTEL {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceConfigurationTypeINTEL(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterScopeKHR(pub(crate) i32);
impl PerformanceCounterScopeKHR {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceCounterScopeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceCounterScopeKHR {
    pub const COMMAND_BUFFER: Self = todo!();
    pub const RENDER_PASS: Self = todo!();
    pub const COMMAND: Self = todo!();
    pub const QUERY_SCOPE_COMMAND_BUFFER: Self = todo!();
    pub const QUERY_SCOPE_RENDER_PASS: Self = todo!();
    pub const QUERY_SCOPE_COMMAND: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterStorageKHR(pub(crate) i32);
impl PerformanceCounterStorageKHR {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceCounterStorageKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceCounterStorageKHR {
    pub const INT32: Self = todo!();
    pub const INT64: Self = todo!();
    pub const UINT32: Self = todo!();
    pub const UINT64: Self = todo!();
    pub const FLOAT32: Self = todo!();
    pub const FLOAT64: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterUnitKHR(pub(crate) i32);
impl PerformanceCounterUnitKHR {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceCounterUnitKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceCounterUnitKHR {
    pub const GENERIC: Self = todo!();
    pub const PERCENTAGE: Self = todo!();
    pub const NANOSECONDS: Self = todo!();
    pub const BYTES: Self = todo!();
    pub const BYTES_PER_SECOND: Self = todo!();
    pub const KELVIN: Self = todo!();
    pub const WATTS: Self = todo!();
    pub const VOLTS: Self = todo!();
    pub const AMPS: Self = todo!();
    pub const HERTZ: Self = todo!();
    pub const CYCLES: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceOverrideTypeINTEL(pub(crate) i32);
impl PerformanceOverrideTypeINTEL {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceOverrideTypeINTEL(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE_INTEL: Self = todo!();
    pub const FLUSH_GPU_CACHES_INTEL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceParameterTypeINTEL(pub(crate) i32);
impl PerformanceParameterTypeINTEL {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceParameterTypeINTEL(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED_INTEL: Self = todo!();
    pub const STREAM_MARKER_VALID_BITS_INTEL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceValueTypeINTEL(pub(crate) i32);
impl PerformanceValueTypeINTEL {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceValueTypeINTEL(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceValueTypeINTEL {
    pub const UINT32_INTEL: Self = todo!();
    pub const UINT64_INTEL: Self = todo!();
    pub const FLOAT_INTEL: Self = todo!();
    pub const BOOL_INTEL: Self = todo!();
    pub const STRING_INTEL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PhysicalDeviceType(pub(crate) i32);
impl PhysicalDeviceType {
    pub const fn from_raw(x: i32) -> Self {
        PhysicalDeviceType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PhysicalDeviceType {
    pub const OTHER: Self = todo!();
    pub const INTEGRATED_GPU: Self = todo!();
    pub const DISCRETE_GPU: Self = todo!();
    pub const VIRTUAL_GPU: Self = todo!();
    pub const CPU: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineBindPoint(pub(crate) i32);
impl PipelineBindPoint {
    pub const fn from_raw(x: i32) -> Self {
        PipelineBindPoint(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineBindPoint {
    pub const GRAPHICS: Self = todo!();
    pub const COMPUTE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCacheHeaderVersion(pub(crate) i32);
impl PipelineCacheHeaderVersion {
    pub const fn from_raw(x: i32) -> Self {
        PipelineCacheHeaderVersion(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineCacheHeaderVersion {
    pub const ONE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineExecutableStatisticFormatKHR(pub(crate) i32);
impl PipelineExecutableStatisticFormatKHR {
    pub const fn from_raw(x: i32) -> Self {
        PipelineExecutableStatisticFormatKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32: Self = todo!();
    pub const INT64: Self = todo!();
    pub const UINT64: Self = todo!();
    pub const FLOAT64: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PointClippingBehavior(pub(crate) i32);
impl PointClippingBehavior {
    pub const fn from_raw(x: i32) -> Self {
        PointClippingBehavior(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = todo!();
    pub const USER_CLIP_PLANES_ONLY: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PolygonMode(pub(crate) i32);
impl PolygonMode {
    pub const fn from_raw(x: i32) -> Self {
        PolygonMode(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PolygonMode {
    pub const FILL: Self = todo!();
    pub const LINE: Self = todo!();
    pub const POINT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PresentModeKHR(pub(crate) i32);
impl PresentModeKHR {
    pub const fn from_raw(x: i32) -> Self {
        PresentModeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PresentModeKHR {
    pub const IMMEDIATE: Self = todo!();
    pub const MAILBOX: Self = todo!();
    pub const FIFO: Self = todo!();
    pub const FIFO_RELAXED: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PrimitiveTopology(pub(crate) i32);
impl PrimitiveTopology {
    pub const fn from_raw(x: i32) -> Self {
        PrimitiveTopology(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PrimitiveTopology {
    pub const POINT_LIST: Self = todo!();
    pub const LINE_LIST: Self = todo!();
    pub const LINE_STRIP: Self = todo!();
    pub const TRIANGLE_LIST: Self = todo!();
    pub const TRIANGLE_STRIP: Self = todo!();
    pub const TRIANGLE_FAN: Self = todo!();
    pub const LINE_LIST_WITH_ADJACENCY: Self = todo!();
    pub const LINE_STRIP_WITH_ADJACENCY: Self = todo!();
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = todo!();
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = todo!();
    pub const PATCH_LIST: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueryPoolSamplingModeINTEL(pub(crate) i32);
impl QueryPoolSamplingModeINTEL {
    pub const fn from_raw(x: i32) -> Self {
        QueryPoolSamplingModeINTEL(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueryPoolSamplingModeINTEL {
    pub const MANUAL_INTEL: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueryType(pub(crate) i32);
impl QueryType {
    pub const fn from_raw(x: i32) -> Self {
        QueryType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueryType {
    pub const OCCLUSION: Self = todo!();
    pub const PIPELINE_STATISTICS: Self = todo!();
    pub const TIMESTAMP: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueueGlobalPriorityEXT(pub(crate) i32);
impl QueueGlobalPriorityEXT {
    pub const fn from_raw(x: i32) -> Self {
        QueueGlobalPriorityEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueueGlobalPriorityEXT {
    pub const LOW: Self = todo!();
    pub const MEDIUM: Self = todo!();
    pub const HIGH: Self = todo!();
    pub const REALTIME: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RasterizationOrderAMD(pub(crate) i32);
impl RasterizationOrderAMD {
    pub const fn from_raw(x: i32) -> Self {
        RasterizationOrderAMD(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl RasterizationOrderAMD {
    pub const STRICT_AMD: Self = todo!();
    pub const RELAXED_AMD: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RayTracingShaderGroupTypeKHR(pub(crate) i32);
impl RayTracingShaderGroupTypeKHR {
    pub const fn from_raw(x: i32) -> Self {
        RayTracingShaderGroupTypeKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL: Self = todo!();
    pub const TRIANGLES_HIT_GROUP: Self = todo!();
    pub const PROCEDURAL_HIT_GROUP: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Result(pub(crate) i32);
impl Result {
    pub const fn from_raw(x: i32) -> Self {
        Result(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl Result {
    pub const SUCCESS: Self = todo!();
    pub const NOT_READY: Self = todo!();
    pub const TIMEOUT: Self = todo!();
    pub const EVENT_SET: Self = todo!();
    pub const EVENT_RESET: Self = todo!();
    pub const INCOMPLETE: Self = todo!();
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = todo!();
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = todo!();
    pub const ERROR_INITIALIZATION_FAILED: Self = todo!();
    pub const ERROR_DEVICE_LOST: Self = todo!();
    pub const ERROR_MEMORY_MAP_FAILED: Self = todo!();
    pub const ERROR_LAYER_NOT_PRESENT: Self = todo!();
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = todo!();
    pub const ERROR_FEATURE_NOT_PRESENT: Self = todo!();
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = todo!();
    pub const ERROR_TOO_MANY_OBJECTS: Self = todo!();
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = todo!();
    pub const ERROR_FRAGMENTED_POOL: Self = todo!();
    pub const ERROR_UNKNOWN: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerAddressMode(pub(crate) i32);
impl SamplerAddressMode {
    pub const fn from_raw(x: i32) -> Self {
        SamplerAddressMode(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SamplerAddressMode {
    pub const REPEAT: Self = todo!();
    pub const MIRRORED_REPEAT: Self = todo!();
    pub const CLAMP_TO_EDGE: Self = todo!();
    pub const CLAMP_TO_BORDER: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerMipmapMode(pub(crate) i32);
impl SamplerMipmapMode {
    pub const fn from_raw(x: i32) -> Self {
        SamplerMipmapMode(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SamplerMipmapMode {
    pub const NEAREST: Self = todo!();
    pub const LINEAR: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerReductionMode(pub(crate) i32);
impl SamplerReductionMode {
    pub const fn from_raw(x: i32) -> Self {
        SamplerReductionMode(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = todo!();
    pub const MIN: Self = todo!();
    pub const MAX: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrModelConversion(pub(crate) i32);
impl SamplerYcbcrModelConversion {
    pub const fn from_raw(x: i32) -> Self {
        SamplerYcbcrModelConversion(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = todo!();
    pub const YCBCR_IDENTITY: Self = todo!();
    pub const YCBCR_709: Self = todo!();
    pub const YCBCR_601: Self = todo!();
    pub const YCBCR_2020: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrRange(pub(crate) i32);
impl SamplerYcbcrRange {
    pub const fn from_raw(x: i32) -> Self {
        SamplerYcbcrRange(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = todo!();
    pub const ITU_NARROW: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ScopeNV(pub(crate) i32);
impl ScopeNV {
    pub const fn from_raw(x: i32) -> Self {
        ScopeNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ScopeNV {
    pub const DEVICE_NV: Self = todo!();
    pub const WORKGROUP_NV: Self = todo!();
    pub const SUBGROUP_NV: Self = todo!();
    pub const QUEUE_FAMILY_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreType(pub(crate) i32);
impl SemaphoreType {
    pub const fn from_raw(x: i32) -> Self {
        SemaphoreType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SemaphoreType {
    pub const BINARY: Self = todo!();
    pub const TIMELINE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShaderFloatControlsIndependence(pub(crate) i32);
impl ShaderFloatControlsIndependence {
    pub const fn from_raw(x: i32) -> Self {
        ShaderFloatControlsIndependence(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ShaderFloatControlsIndependence {
    pub const 32_BIT_ONLY: Self = todo!();
    pub const ALL: Self = todo!();
    pub const NONE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShaderInfoTypeAMD(pub(crate) i32);
impl ShaderInfoTypeAMD {
    pub const fn from_raw(x: i32) -> Self {
        ShaderInfoTypeAMD(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ShaderInfoTypeAMD {
    pub const STATISTICS_AMD: Self = todo!();
    pub const BINARY_AMD: Self = todo!();
    pub const DISASSEMBLY_AMD: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShadingRatePaletteEntryNV(pub(crate) i32);
impl ShadingRatePaletteEntryNV {
    pub const fn from_raw(x: i32) -> Self {
        ShadingRatePaletteEntryNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ShadingRatePaletteEntryNV {
    pub const NO_INVOCATIONS_NV: Self = todo!();
    pub const 16_INVOCATIONS_PER_PIXEL_NV: Self = todo!();
    pub const 8_INVOCATIONS_PER_PIXEL_NV: Self = todo!();
    pub const 4_INVOCATIONS_PER_PIXEL_NV: Self = todo!();
    pub const 2_INVOCATIONS_PER_PIXEL_NV: Self = todo!();
    pub const 1_INVOCATION_PER_PIXEL_NV: Self = todo!();
    pub const 1_INVOCATION_PER_2X1_PIXELS_NV: Self = todo!();
    pub const 1_INVOCATION_PER_1X2_PIXELS_NV: Self = todo!();
    pub const 1_INVOCATION_PER_2X2_PIXELS_NV: Self = todo!();
    pub const 1_INVOCATION_PER_4X2_PIXELS_NV: Self = todo!();
    pub const 1_INVOCATION_PER_2X4_PIXELS_NV: Self = todo!();
    pub const 1_INVOCATION_PER_4X4_PIXELS_NV: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SharingMode(pub(crate) i32);
impl SharingMode {
    pub const fn from_raw(x: i32) -> Self {
        SharingMode(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SharingMode {
    pub const EXCLUSIVE: Self = todo!();
    pub const CONCURRENT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StencilOp(pub(crate) i32);
impl StencilOp {
    pub const fn from_raw(x: i32) -> Self {
        StencilOp(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl StencilOp {
    pub const KEEP: Self = todo!();
    pub const ZERO: Self = todo!();
    pub const REPLACE: Self = todo!();
    pub const INCREMENT_AND_CLAMP: Self = todo!();
    pub const DECREMENT_AND_CLAMP: Self = todo!();
    pub const INVERT: Self = todo!();
    pub const INCREMENT_AND_WRAP: Self = todo!();
    pub const DECREMENT_AND_WRAP: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StructureType(pub(crate) i32);
impl StructureType {
    pub const fn from_raw(x: i32) -> Self {
        StructureType(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl StructureType {
    pub const APPLICATION_INFO: Self = todo!();
    pub const INSTANCE_CREATE_INFO: Self = todo!();
    pub const DEVICE_QUEUE_CREATE_INFO: Self = todo!();
    pub const DEVICE_CREATE_INFO: Self = todo!();
    pub const SUBMIT_INFO: Self = todo!();
    pub const MEMORY_ALLOCATE_INFO: Self = todo!();
    pub const MAPPED_MEMORY_RANGE: Self = todo!();
    pub const BIND_SPARSE_INFO: Self = todo!();
    pub const FENCE_CREATE_INFO: Self = todo!();
    pub const SEMAPHORE_CREATE_INFO: Self = todo!();
    pub const EVENT_CREATE_INFO: Self = todo!();
    pub const QUERY_POOL_CREATE_INFO: Self = todo!();
    pub const BUFFER_CREATE_INFO: Self = todo!();
    pub const BUFFER_VIEW_CREATE_INFO: Self = todo!();
    pub const IMAGE_CREATE_INFO: Self = todo!();
    pub const IMAGE_VIEW_CREATE_INFO: Self = todo!();
    pub const SHADER_MODULE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_CACHE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = todo!();
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = todo!();
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = todo!();
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = todo!();
    pub const SAMPLER_CREATE_INFO: Self = todo!();
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = todo!();
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = todo!();
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = todo!();
    pub const WRITE_DESCRIPTOR_SET: Self = todo!();
    pub const COPY_DESCRIPTOR_SET: Self = todo!();
    pub const FRAMEBUFFER_CREATE_INFO: Self = todo!();
    pub const RENDER_PASS_CREATE_INFO: Self = todo!();
    pub const COMMAND_POOL_CREATE_INFO: Self = todo!();
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = todo!();
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = todo!();
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = todo!();
    pub const RENDER_PASS_BEGIN_INFO: Self = todo!();
    pub const BUFFER_MEMORY_BARRIER: Self = todo!();
    pub const IMAGE_MEMORY_BARRIER: Self = todo!();
    pub const MEMORY_BARRIER: Self = todo!();
    pub const LOADER_INSTANCE_CREATE_INFO: Self = todo!();
    pub const LOADER_DEVICE_CREATE_INFO: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubpassContents(pub(crate) i32);
impl SubpassContents {
    pub const fn from_raw(x: i32) -> Self {
        SubpassContents(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SubpassContents {
    pub const INLINE: Self = todo!();
    pub const SECONDARY_COMMAND_BUFFERS: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SystemAllocationScope(pub(crate) i32);
impl SystemAllocationScope {
    pub const fn from_raw(x: i32) -> Self {
        SystemAllocationScope(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SystemAllocationScope {
    pub const COMMAND: Self = todo!();
    pub const OBJECT: Self = todo!();
    pub const CACHE: Self = todo!();
    pub const DEVICE: Self = todo!();
    pub const INSTANCE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TessellationDomainOrigin(pub(crate) i32);
impl TessellationDomainOrigin {
    pub const fn from_raw(x: i32) -> Self {
        TessellationDomainOrigin(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = todo!();
    pub const LOWER_LEFT: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TimeDomainEXT(pub(crate) i32);
impl TimeDomainEXT {
    pub const fn from_raw(x: i32) -> Self {
        TimeDomainEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl TimeDomainEXT {
    pub const DEVICE: Self = todo!();
    pub const CLOCK_MONOTONIC: Self = todo!();
    pub const CLOCK_MONOTONIC_RAW: Self = todo!();
    pub const QUERY_PERFORMANCE_COUNTER: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCacheHeaderVersionEXT(pub(crate) i32);
impl ValidationCacheHeaderVersionEXT {
    pub const fn from_raw(x: i32) -> Self {
        ValidationCacheHeaderVersionEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ValidationCacheHeaderVersionEXT {
    pub const ONE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCheckEXT(pub(crate) i32);
impl ValidationCheckEXT {
    pub const fn from_raw(x: i32) -> Self {
        ValidationCheckEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ValidationCheckEXT {
    pub const ALL: Self = todo!();
    pub const SHADERS: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ValidationFeatureDisableEXT(pub(crate) i32);
impl ValidationFeatureDisableEXT {
    pub const fn from_raw(x: i32) -> Self {
        ValidationFeatureDisableEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ValidationFeatureDisableEXT {
    pub const ALL: Self = todo!();
    pub const SHADERS: Self = todo!();
    pub const THREAD_SAFETY: Self = todo!();
    pub const API_PARAMETERS: Self = todo!();
    pub const OBJECT_LIFETIMES: Self = todo!();
    pub const CORE_CHECKS: Self = todo!();
    pub const UNIQUE_HANDLES: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ValidationFeatureEnableEXT(pub(crate) i32);
impl ValidationFeatureEnableEXT {
    pub const fn from_raw(x: i32) -> Self {
        ValidationFeatureEnableEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ValidationFeatureEnableEXT {
    pub const GPU_ASSISTED: Self = todo!();
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = todo!();
    pub const BEST_PRACTICES: Self = todo!();
    pub const DEBUG_PRINTF: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VendorId(pub(crate) i32);
impl VendorId {
    pub const fn from_raw(x: i32) -> Self {
        VendorId(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl VendorId {
    pub const VIV: Self = todo!();
    pub const VSI: Self = todo!();
    pub const KAZAN: Self = todo!();
    pub const CODEPLAY: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VertexInputRate(pub(crate) i32);
impl VertexInputRate {
    pub const fn from_raw(x: i32) -> Self {
        VertexInputRate(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl VertexInputRate {
    pub const VERTEX: Self = todo!();
    pub const INSTANCE: Self = todo!();
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ViewportCoordinateSwizzleNV(pub(crate) i32);
impl ViewportCoordinateSwizzleNV {
    pub const fn from_raw(x: i32) -> Self {
        ViewportCoordinateSwizzleNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ViewportCoordinateSwizzleNV {
    pub const POSITIVE_X_NV: Self = todo!();
    pub const NEGATIVE_X_NV: Self = todo!();
    pub const POSITIVE_Y_NV: Self = todo!();
    pub const NEGATIVE_Y_NV: Self = todo!();
    pub const POSITIVE_Z_NV: Self = todo!();
    pub const NEGATIVE_Z_NV: Self = todo!();
    pub const POSITIVE_W_NV: Self = todo!();
    pub const NEGATIVE_W_NV: Self = todo!();
}
