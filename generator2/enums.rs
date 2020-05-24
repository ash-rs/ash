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
    pub const HOST: Self = Self(0);
    pub const DEVICE: Self = Self(1);
    pub const HOST_OR_DEVICE: Self = Self(2);
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
    pub const OBJECT: Self = Self(0);
    pub const BUILD_SCRATCH: Self = Self(1);
    pub const UPDATE_SCRATCH: Self = Self(2);
}
    // VK_NV_ray_tracing
    pub const OBJECT: Self = Self::OBJECT;
    pub const BUILD_SCRATCH: Self = Self::BUILD_SCRATCH;
    pub const UPDATE_SCRATCH: Self = Self::UPDATE_SCRATCH;
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
    pub const TOP_LEVEL: Self = Self(0);
    pub const BOTTOM_LEVEL: Self = Self(1);
}
    // VK_NV_ray_tracing
    pub const TOP_LEVEL: Self = Self::TOP_LEVEL;
    pub const BOTTOM_LEVEL: Self = Self::BOTTOM_LEVEL;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AccessFlags(pub(crate) i32);
impl AccessFlags {
    pub const fn from_raw(x: i32) -> Self {
        AccessFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AccessFlags {
    pub const INDIRECT_COMMAND_READ: Self = Self(1);
    pub const INDEX_READ: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(4);
    pub const UNIFORM_READ: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ: Self = Self(16);
    pub const SHADER_READ: Self = Self(32);
    pub const SHADER_WRITE: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024);
    pub const TRANSFER_READ: Self = Self(2048);
    pub const TRANSFER_WRITE: Self = Self(4096);
    pub const HOST_READ: Self = Self(8192);
    pub const HOST_WRITE: Self = Self(16384);
    pub const MEMORY_READ: Self = Self(32768);
    pub const MEMORY_WRITE: Self = Self(65536);
}
    // VK_AMD_extension_24
    pub const RESERVED_30: Self = Self(1073741824);
    pub const RESERVED_31: Self = Self(2147483648);
    pub const RESERVED_28: Self = Self(268435456);
    pub const RESERVED_29: Self = Self(536870912);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT: Self = Self(524288);
    pub const CONDITIONAL_RENDERING_READ: Self = Self(1048576);
    pub const FRAGMENT_DENSITY_MAP_READ: Self = Self(16777216);
    pub const TRANSFORM_FEEDBACK_WRITE: Self = Self(33554432);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ: Self = Self(67108864);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE: Self = Self(134217728);
    pub const ACCELERATION_STRUCTURE_READ: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE: Self = Self(4194304);
    pub const COMMAND_PREPROCESS_READ: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE: Self = Self(262144);
    pub const ACCELERATION_STRUCTURE_READ: Self = Self::ACCELERATION_STRUCTURE_READ;
    pub const ACCELERATION_STRUCTURE_WRITE: Self = Self::ACCELERATION_STRUCTURE_WRITE;
    pub const SHADING_RATE_IMAGE_READ: Self = Self(8388608);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AcquireProfilingLockFlagsKHR(pub(crate) i32);
impl AcquireProfilingLockFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        AcquireProfilingLockFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AcquireProfilingLockFlagsKHR {
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentDescriptionFlags(pub(crate) i32);
impl AttachmentDescriptionFlags {
    pub const fn from_raw(x: i32) -> Self {
        AttachmentDescriptionFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl AttachmentDescriptionFlags {
    pub const MAY_ALIAS: Self = Self(1);
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
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
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
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
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
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
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
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
}
    // VK_EXT_blend_operation_advanced
    pub const ZERO: Self = Self(1000000000);
    pub const SRC: Self = Self(1000000001);
    pub const DST: Self = Self(1000000002);
    pub const SRC_OVER: Self = Self(1000000003);
    pub const DST_OVER: Self = Self(1000000004);
    pub const SRC_IN: Self = Self(1000000005);
    pub const DST_IN: Self = Self(1000000006);
    pub const SRC_OUT: Self = Self(1000000007);
    pub const DST_OUT: Self = Self(1000000008);
    pub const SRC_ATOP: Self = Self(1000000009);
    pub const DST_ATOP: Self = Self(1000000010);
    pub const XOR: Self = Self(1000000011);
    pub const MULTIPLY: Self = Self(1000000012);
    pub const SCREEN: Self = Self(1000000013);
    pub const OVERLAY: Self = Self(1000000014);
    pub const DARKEN: Self = Self(1000000015);
    pub const LIGHTEN: Self = Self(1000000016);
    pub const COLORDODGE: Self = Self(1000000017);
    pub const COLORBURN: Self = Self(1000000018);
    pub const HARDLIGHT: Self = Self(1000000019);
    pub const SOFTLIGHT: Self = Self(1000000020);
    pub const DIFFERENCE: Self = Self(1000000021);
    pub const EXCLUSION: Self = Self(1000000022);
    pub const INVERT: Self = Self(1000000023);
    pub const INVERT_RGB: Self = Self(1000000024);
    pub const LINEARDODGE: Self = Self(1000000025);
    pub const LINEARBURN: Self = Self(1000000026);
    pub const VIVIDLIGHT: Self = Self(1000000027);
    pub const LINEARLIGHT: Self = Self(1000000028);
    pub const PINLIGHT: Self = Self(1000000029);
    pub const HARDMIX: Self = Self(1000000030);
    pub const HSL_HUE: Self = Self(1000000031);
    pub const HSL_SATURATION: Self = Self(1000000032);
    pub const HSL_COLOR: Self = Self(1000000033);
    pub const HSL_LUMINOSITY: Self = Self(1000000034);
    pub const PLUS: Self = Self(1000000035);
    pub const PLUS_CLAMPED: Self = Self(1000000036);
    pub const PLUS_CLAMPED_ALPHA: Self = Self(1000000037);
    pub const PLUS_DARKER: Self = Self(1000000038);
    pub const MINUS: Self = Self(1000000039);
    pub const MINUS_CLAMPED: Self = Self(1000000040);
    pub const CONTRAST: Self = Self(1000000041);
    pub const INVERT_OVG: Self = Self(1000000042);
    pub const RED: Self = Self(1000000043);
    pub const GREEN: Self = Self(1000000044);
    pub const BLUE: Self = Self(1000000045);
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
    pub const UNCORRELATED: Self = Self(0);
    pub const DISJOINT: Self = Self(1);
    pub const CONJOINT: Self = Self(2);
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
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BufferCreateFlags(pub(crate) i32);
impl BufferCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        BufferCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BufferCreateFlags {
    pub const SPARSE_BINDING: Self = Self(1);
    pub const SPARSE_RESIDENCY: Self = Self(2);
    pub const SPARSE_ALIASED: Self = Self(4);
}
    // VK_EXT_buffer_device_address
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BufferUsageFlags(pub(crate) i32);
impl BufferUsageFlags {
    pub const fn from_raw(x: i32) -> Self {
        BufferUsageFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BufferUsageFlags {
    pub const TRANSFER_SRC: Self = Self(1);
    pub const TRANSFER_DST: Self = Self(2);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(8);
    pub const UNIFORM_BUFFER: Self = Self(16);
    pub const STORAGE_BUFFER: Self = Self(32);
    pub const INDEX_BUFFER: Self = Self(64);
    pub const VERTEX_BUFFER: Self = Self(128);
    pub const INDIRECT_BUFFER: Self = Self(256);
}
    // VK_AMD_extension_24
    pub const RESERVED_15: Self = Self(32768);
    pub const RESERVED_16: Self = Self(65536);
    pub const RESERVED_13: Self = Self(8192);
    pub const RESERVED_14: Self = Self(16384);
    pub const SHADER_DEVICE_ADDRESS: Self = Self::SHADER_DEVICE_ADDRESS;
    pub const CONDITIONAL_RENDERING: Self = Self(512);
    pub const TRANSFORM_FEEDBACK_BUFFER: Self = Self(2048);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER: Self = Self(4096);
    pub const SHADER_DEVICE_ADDRESS: Self = Self::SHADER_DEVICE_ADDRESS;
    pub const RAY_TRACING: Self = Self(1024);
    pub const RAY_TRACING: Self = Self::RAY_TRACING;
    pub const RESERVED_18: Self = Self(262144);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BuildAccelerationStructureFlagsKHR(pub(crate) i32);
impl BuildAccelerationStructureFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        BuildAccelerationStructureFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE: Self = Self(1);
    pub const ALLOW_COMPACTION: Self = Self(2);
    pub const PREFER_FAST_TRACE: Self = Self(4);
    pub const PREFER_FAST_BUILD: Self = Self(8);
    pub const LOW_MEMORY: Self = Self(16);
}
    // VK_NV_ray_tracing
    pub const ALLOW_UPDATE: Self = Self::ALLOW_UPDATE;
    pub const ALLOW_COMPACTION: Self = Self::ALLOW_COMPACTION;
    pub const PREFER_FAST_TRACE: Self = Self::PREFER_FAST_TRACE;
    pub const PREFER_FAST_BUILD: Self = Self::PREFER_FAST_BUILD;
    pub const LOW_MEMORY: Self = Self::LOW_MEMORY;
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
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}
    // VK_KHR_sampler_ycbcr_conversion
    pub const COSITED_EVEN: Self = Self::COSITED_EVEN;
    pub const MIDPOINT: Self = Self::MIDPOINT;
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
    pub const DEFAULT: Self = Self(0);
    pub const CUSTOM: Self = Self(1);
    pub const PIXEL_MAJOR: Self = Self(2);
    pub const SAMPLE_MAJOR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ColorComponentFlags(pub(crate) i32);
impl ColorComponentFlags {
    pub const fn from_raw(x: i32) -> Self {
        ColorComponentFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ColorComponentFlags {
    pub const R: Self = Self(1);
    pub const G: Self = Self(2);
    pub const B: Self = Self(4);
    pub const A: Self = Self(8);
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
    pub const SRGB_NONLINEAR: Self = Self(0);
    pub const COLORSPACE_SRGB_NONLINEAR: Self = Self::SRGB_NONLINEAR;
}
    // VK_AMD_display_native_hdr
    pub const DISPLAY_NATIVE: Self = Self(1000000000);
    pub const DISPLAY_P3_NONLINEAR: Self = Self(1000000001);
    pub const EXTENDED_SRGB_LINEAR: Self = Self(1000000002);
    pub const DISPLAY_P3_LINEAR: Self = Self(1000000003);
    pub const DCI_P3_NONLINEAR: Self = Self(1000000004);
    pub const BT709_LINEAR: Self = Self(1000000005);
    pub const BT709_NONLINEAR: Self = Self(1000000006);
    pub const BT2020_LINEAR: Self = Self(1000000007);
    pub const HDR10_ST2084: Self = Self(1000000008);
    pub const DOLBYVISION: Self = Self(1000000009);
    pub const HDR10_HLG: Self = Self(1000000010);
    pub const ADOBERGB_LINEAR: Self = Self(1000000011);
    pub const ADOBERGB_NONLINEAR: Self = Self(1000000012);
    pub const PASS_THROUGH: Self = Self(1000000013);
    pub const EXTENDED_SRGB_NONLINEAR: Self = Self(1000000014);
    pub const DCI_P3_LINEAR: Self = Self::DISPLAY_P3_LINEAR;
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
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferResetFlags(pub(crate) i32);
impl CommandBufferResetFlags {
    pub const fn from_raw(x: i32) -> Self {
        CommandBufferResetFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CommandBufferResetFlags {
    pub const RELEASE_RESOURCES: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferUsageFlags(pub(crate) i32);
impl CommandBufferUsageFlags {
    pub const fn from_raw(x: i32) -> Self {
        CommandBufferUsageFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = Self(1);
    pub const RENDER_PASS_CONTINUE: Self = Self(2);
    pub const SIMULTANEOUS_USE: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CommandPoolCreateFlags(pub(crate) i32);
impl CommandPoolCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        CommandPoolCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CommandPoolCreateFlags {
    pub const TRANSIENT: Self = Self(1);
    pub const RESET_COMMAND_BUFFER: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CommandPoolResetFlags(pub(crate) i32);
impl CommandPoolResetFlags {
    pub const fn from_raw(x: i32) -> Self {
        CommandPoolResetFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CommandPoolResetFlags {
    pub const RELEASE_RESOURCES: Self = Self(1);
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
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
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
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
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
    pub const FLOAT16: Self = Self(0);
    pub const FLOAT32: Self = Self(1);
    pub const FLOAT64: Self = Self(2);
    pub const SINT8: Self = Self(3);
    pub const SINT16: Self = Self(4);
    pub const SINT32: Self = Self(5);
    pub const SINT64: Self = Self(6);
    pub const UINT8: Self = Self(7);
    pub const UINT16: Self = Self(8);
    pub const UINT32: Self = Self(9);
    pub const UINT64: Self = Self(10);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CompositeAlphaFlagsKHR(pub(crate) i32);
impl CompositeAlphaFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        CompositeAlphaFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CompositeAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(1);
    pub const PRE_MULTIPLIED: Self = Self(2);
    pub const POST_MULTIPLIED: Self = Self(4);
    pub const INHERIT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ConditionalRenderingFlagsEXT(pub(crate) i32);
impl ConditionalRenderingFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        ConditionalRenderingFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ConditionalRenderingFlagsEXT {
    pub const INVERTED: Self = Self(1);
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
    pub const DISABLED: Self = Self(0);
    pub const OVERESTIMATE: Self = Self(1);
    pub const UNDERESTIMATE: Self = Self(2);
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
    pub const CLONE: Self = Self(0);
    pub const COMPACT: Self = Self(1);
    pub const SERIALIZE: Self = Self(2);
    pub const DESERIALIZE: Self = Self(3);
}
    // VK_NV_ray_tracing
    pub const CLONE: Self = Self::CLONE;
    pub const COMPACT: Self = Self::COMPACT;
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
    pub const NONE: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const ALPHA: Self = Self(2);
    pub const RGBA: Self = Self(3);
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
    pub const MERGE: Self = Self(0);
    pub const TRUNCATE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CullModeFlags(pub(crate) i32);
impl CullModeFlags {
    pub const fn from_raw(x: i32) -> Self {
        CullModeFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl CullModeFlags {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(1);
    pub const BACK: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DebugReportFlagsEXT(pub(crate) i32);
impl DebugReportFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        DebugReportFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DebugReportFlagsEXT {
    pub const INFORMATION: Self = Self(1);
    pub const WARNING: Self = Self(2);
    pub const PERFORMANCE_WARNING: Self = Self(4);
    pub const ERROR: Self = Self(8);
    pub const DEBUG: Self = Self(16);
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
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
    pub const SURFACE_KHR: Self = Self(26);
    pub const SWAPCHAIN_KHR: Self = Self(27);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(28);
    pub const DEBUG_REPORT: Self = Self::DEBUG_REPORT_CALLBACK_EXT;
    pub const DISPLAY_KHR: Self = Self(29);
    pub const DISPLAY_MODE_KHR: Self = Self(30);
    pub const VALIDATION_CACHE_EXT: Self = Self(33);
    pub const VALIDATION_CACHE: Self = Self::VALIDATION_CACHE_EXT;
}
    // VK_EXT_debug_report
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000165000);
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self::ACCELERATION_STRUCTURE_KHR;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub(crate) i32);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        DebugUtilsMessageSeverityFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE: Self = Self(1);
    pub const INFO: Self = Self(16);
    pub const WARNING: Self = Self(256);
    pub const ERROR: Self = Self(4096);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessageTypeFlagsEXT(pub(crate) i32);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        DebugUtilsMessageTypeFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL: Self = Self(1);
    pub const VALIDATION: Self = Self(2);
    pub const PERFORMANCE: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DependencyFlags(pub(crate) i32);
impl DependencyFlags {
    pub const fn from_raw(x: i32) -> Self {
        DependencyFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DependencyFlags {
    pub const BY_REGION: Self = Self(1);
}
    // VK_KHR_device_group
    pub const DEVICE_GROUP: Self = Self::DEVICE_GROUP;
    pub const VIEW_LOCAL: Self = Self::VIEW_LOCAL;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorBindingFlags(pub(crate) i32);
impl DescriptorBindingFlags {
    pub const fn from_raw(x: i32) -> Self {
        DescriptorBindingFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(1);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2);
    pub const PARTIALLY_BOUND: Self = Self(4);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8);
}
    // VK_EXT_descriptor_indexing
    pub const UPDATE_AFTER_BIND: Self = Self::UPDATE_AFTER_BIND;
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    pub const PARTIALLY_BOUND: Self = Self::PARTIALLY_BOUND;
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorPoolCreateFlags(pub(crate) i32);
impl DescriptorPoolCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        DescriptorPoolCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DescriptorPoolCreateFlags {
    pub const FREE_DESCRIPTOR_SET: Self = Self(1);
}
    // VK_EXT_descriptor_indexing
    pub const UPDATE_AFTER_BIND: Self = Self::UPDATE_AFTER_BIND;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorSetLayoutCreateFlags(pub(crate) i32);
impl DescriptorSetLayoutCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        DescriptorSetLayoutCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DescriptorSetLayoutCreateFlags {
}
    // VK_EXT_descriptor_indexing
    pub const UPDATE_AFTER_BIND_POOL: Self = Self::UPDATE_AFTER_BIND_POOL;
    pub const PUSH_DESCRIPTOR: Self = Self(1);
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
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
}
    // VK_EXT_inline_uniform_block
    pub const INLINE_UNIFORM_BLOCK: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE: Self = Self::ACCELERATION_STRUCTURE;
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
    pub const DESCRIPTOR_SET: Self = Self(0);
}
    // VK_KHR_descriptor_update_template
    pub const DESCRIPTOR_SET: Self = Self::DESCRIPTOR_SET;
    pub const PUSH_DESCRIPTORS: Self = Self(1);
    pub const PUSH_DESCRIPTORS: Self = Self(1);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigFlagsNV(pub(crate) i32);
impl DeviceDiagnosticsConfigFlagsNV {
    pub const fn from_raw(x: i32) -> Self {
        DeviceDiagnosticsConfigFlagsNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DeviceDiagnosticsConfigFlagsNV {
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(1);
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(2);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(4);
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
    pub const DISPLAY_HOTPLUG: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagsKHR(pub(crate) i32);
impl DeviceGroupPresentModeFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        DeviceGroupPresentModeFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DeviceGroupPresentModeFlagsKHR {
    pub const LOCAL: Self = Self(1);
    pub const REMOTE: Self = Self(2);
    pub const SUM: Self = Self(4);
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DeviceQueueCreateFlags(pub(crate) i32);
impl DeviceQueueCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        DeviceQueueCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DeviceQueueCreateFlags {
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
    pub const INCLUSIVE: Self = Self(0);
    pub const EXCLUSIVE: Self = Self(1);
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
    pub const FIRST_PIXEL_OUT: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DisplayPlaneAlphaFlagsKHR(pub(crate) i32);
impl DisplayPlaneAlphaFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        DisplayPlaneAlphaFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(1);
    pub const GLOBAL: Self = Self(2);
    pub const PER_PIXEL: Self = Self(4);
    pub const PER_PIXEL_PREMULTIPLIED: Self = Self(8);
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
    pub const OFF: Self = Self(0);
    pub const SUSPEND: Self = Self(1);
    pub const ON: Self = Self(2);
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
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
}
    // VK_KHR_driver_properties
    pub const AMD_PROPRIETARY: Self = Self::AMD_PROPRIETARY;
    pub const AMD_OPEN_SOURCE: Self = Self::AMD_OPEN_SOURCE;
    pub const MESA_RADV: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY: Self = Self::NVIDIA_PROPRIETARY;
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self::INTEL_OPEN_SOURCE;
    pub const IMAGINATION_PROPRIETARY: Self = Self::IMAGINATION_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY: Self = Self::QUALCOMM_PROPRIETARY;
    pub const ARM_PROPRIETARY: Self = Self::ARM_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER: Self = Self::GOOGLE_SWIFTSHADER;
    pub const GGP_PROPRIETARY: Self = Self::GGP_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY: Self = Self::BROADCOM_PROPRIETARY;
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
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
}
    // VK_EXT_discard_rectangles
    pub const DISCARD_RECTANGLE: Self = Self(1000000000);
    pub const LINE_STIPPLE: Self = Self(1000000000);
    pub const SAMPLE_LOCATIONS: Self = Self(1000000000);
    pub const VIEWPORT_W_SCALING: Self = Self(1000000000);
    pub const EXCLUSIVE_SCISSOR: Self = Self(1000000001);
    pub const VIEWPORT_SHADING_RATE_PALETTE: Self = Self(1000000004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER: Self = Self(1000000006);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalFenceFeatureFlags(pub(crate) i32);
impl ExternalFenceFeatureFlags {
    pub const fn from_raw(x: i32) -> Self {
        ExternalFenceFeatureFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE: Self = Self(1);
    pub const IMPORTABLE: Self = Self(2);
}
    // VK_KHR_external_fence_capabilities
    pub const EXPORTABLE: Self = Self::EXPORTABLE;
    pub const IMPORTABLE: Self = Self::IMPORTABLE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalFenceHandleTypeFlags(pub(crate) i32);
impl ExternalFenceHandleTypeFlags {
    pub const fn from_raw(x: i32) -> Self {
        ExternalFenceHandleTypeFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(1);
    pub const OPAQUE_WIN32: Self = Self(2);
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    pub const SYNC_FD: Self = Self(8);
}
    // VK_KHR_external_fence_capabilities
    pub const OPAQUE_FD: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD: Self = Self::SYNC_FD;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlags(pub(crate) i32);
impl ExternalMemoryFeatureFlags {
    pub const fn from_raw(x: i32) -> Self {
        ExternalMemoryFeatureFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY: Self = Self(1);
    pub const EXPORTABLE: Self = Self(2);
    pub const IMPORTABLE: Self = Self(4);
}
    // VK_KHR_external_memory_capabilities
    pub const DEDICATED_ONLY: Self = Self::DEDICATED_ONLY;
    pub const EXPORTABLE: Self = Self::EXPORTABLE;
    pub const IMPORTABLE: Self = Self::IMPORTABLE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlagsNV(pub(crate) i32);
impl ExternalMemoryFeatureFlagsNV {
    pub const fn from_raw(x: i32) -> Self {
        ExternalMemoryFeatureFlagsNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalMemoryFeatureFlagsNV {
    pub const DEDICATED_ONLY: Self = Self(1);
    pub const EXPORTABLE: Self = Self(2);
    pub const IMPORTABLE: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlags(pub(crate) i32);
impl ExternalMemoryHandleTypeFlags {
    pub const fn from_raw(x: i32) -> Self {
        ExternalMemoryHandleTypeFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(1);
    pub const OPAQUE_WIN32: Self = Self(2);
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    pub const D3D11_TEXTURE: Self = Self(8);
    pub const D3D11_TEXTURE_KMT: Self = Self(16);
    pub const D3D12_HEAP: Self = Self(32);
    pub const D3D12_RESOURCE: Self = Self(64);
}
    // VK_ANDROID_external_memory_android_hardware_buffer
    pub const ANDROID_HARDWARE_BUFFER: Self = Self(1024);
    pub const DMA_BUF: Self = Self(512);
    pub const HOST_ALLOCATION: Self = Self(128);
    pub const HOST_MAPPED_FOREIGN_MEMORY: Self = Self(256);
    pub const OPAQUE_FD: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D11_TEXTURE: Self = Self::D3D11_TEXTURE;
    pub const D3D11_TEXTURE_KMT: Self = Self::D3D11_TEXTURE_KMT;
    pub const D3D12_HEAP: Self = Self::D3D12_HEAP;
    pub const D3D12_RESOURCE: Self = Self::D3D12_RESOURCE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlagsNV(pub(crate) i32);
impl ExternalMemoryHandleTypeFlagsNV {
    pub const fn from_raw(x: i32) -> Self {
        ExternalMemoryHandleTypeFlagsNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalMemoryHandleTypeFlagsNV {
    pub const OPAQUE_WIN32: Self = Self(1);
    pub const OPAQUE_WIN32_KMT: Self = Self(2);
    pub const D3D11_IMAGE: Self = Self(4);
    pub const D3D11_IMAGE_KMT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalSemaphoreFeatureFlags(pub(crate) i32);
impl ExternalSemaphoreFeatureFlags {
    pub const fn from_raw(x: i32) -> Self {
        ExternalSemaphoreFeatureFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE: Self = Self(1);
    pub const IMPORTABLE: Self = Self(2);
}
    // VK_KHR_external_semaphore_capabilities
    pub const EXPORTABLE: Self = Self::EXPORTABLE;
    pub const IMPORTABLE: Self = Self::IMPORTABLE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ExternalSemaphoreHandleTypeFlags(pub(crate) i32);
impl ExternalSemaphoreHandleTypeFlags {
    pub const fn from_raw(x: i32) -> Self {
        ExternalSemaphoreHandleTypeFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(1);
    pub const OPAQUE_WIN32: Self = Self(2);
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    pub const D3D12_FENCE: Self = Self(8);
    pub const SYNC_FD: Self = Self(16);
}
    // VK_KHR_external_semaphore_capabilities
    pub const OPAQUE_FD: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D12_FENCE: Self = Self::D3D12_FENCE;
    pub const SYNC_FD: Self = Self::SYNC_FD;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FenceCreateFlags(pub(crate) i32);
impl FenceCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        FenceCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl FenceCreateFlags {
    pub const SIGNALED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FenceImportFlags(pub(crate) i32);
impl FenceImportFlags {
    pub const fn from_raw(x: i32) -> Self {
        FenceImportFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl FenceImportFlags {
    pub const TEMPORARY: Self = Self(1);
}
    // VK_KHR_external_fence
    pub const TEMPORARY: Self = Self::TEMPORARY;
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
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
    // VK_EXT_filter_cubic
    pub const CUBIC: Self = Self::CUBIC;
    pub const CUBIC: Self = Self(1000000000);
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
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4x4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4x4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5x4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5x4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5x5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5x5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6x5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6x5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6x6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6x6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8x5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8x5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8x6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8x6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8x8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8x8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10x5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10x5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10x6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10x6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10x8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10x8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10x10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10x10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12x10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12x10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12x12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12x12_SRGB_BLOCK: Self = Self(184);
}
    // VK_EXT_texture_compression_astc_hdr
    pub const ASTC_4x4_SFLOAT_BLOCK: Self = Self(1000066000);
    pub const ASTC_5x4_SFLOAT_BLOCK: Self = Self(1000066001);
    pub const ASTC_5x5_SFLOAT_BLOCK: Self = Self(1000066002);
    pub const ASTC_6x5_SFLOAT_BLOCK: Self = Self(1000066003);
    pub const ASTC_6x6_SFLOAT_BLOCK: Self = Self(1000066004);
    pub const ASTC_8x5_SFLOAT_BLOCK: Self = Self(1000066005);
    pub const ASTC_8x6_SFLOAT_BLOCK: Self = Self(1000066006);
    pub const ASTC_8x8_SFLOAT_BLOCK: Self = Self(1000066007);
    pub const ASTC_10x5_SFLOAT_BLOCK: Self = Self(1000066008);
    pub const ASTC_10x6_SFLOAT_BLOCK: Self = Self(1000066009);
    pub const ASTC_10x8_SFLOAT_BLOCK: Self = Self(1000066010);
    pub const ASTC_10x10_SFLOAT_BLOCK: Self = Self(1000066011);
    pub const ASTC_12x10_SFLOAT_BLOCK: Self = Self(1000066012);
    pub const ASTC_12x12_SFLOAT_BLOCK: Self = Self(1000066013);
    pub const PVRTC1_2BPP_UNORM_BLOCK: Self = Self(1000000000);
    pub const PVRTC1_4BPP_UNORM_BLOCK: Self = Self(1000000001);
    pub const PVRTC2_2BPP_UNORM_BLOCK: Self = Self(1000000002);
    pub const PVRTC2_4BPP_UNORM_BLOCK: Self = Self(1000000003);
    pub const PVRTC1_2BPP_SRGB_BLOCK: Self = Self(1000000004);
    pub const PVRTC1_4BPP_SRGB_BLOCK: Self = Self(1000000005);
    pub const PVRTC2_2BPP_SRGB_BLOCK: Self = Self(1000000006);
    pub const PVRTC2_4BPP_SRGB_BLOCK: Self = Self(1000000007);
    pub const G8B8G8R8_422_UNORM: Self = Self::G8B8G8R8_422_UNORM;
    pub const B8G8R8G8_422_UNORM: Self = Self::B8G8R8G8_422_UNORM;
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self::G8_B8R8_2PLANE_420_UNORM;
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self::G8_B8R8_2PLANE_422_UNORM;
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
    pub const R10X6_UNORM_PACK16: Self = Self::R10X6_UNORM_PACK16;
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self::R10X6G10X6_UNORM_2PACK16;
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
    pub const R12X4_UNORM_PACK16: Self = Self::R12X4_UNORM_PACK16;
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self::R12X4G12X4_UNORM_2PACK16;
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
    pub const G16B16G16R16_422_UNORM: Self = Self::G16B16G16R16_422_UNORM;
    pub const B16G16R16G16_422_UNORM: Self = Self::B16G16R16G16_422_UNORM;
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self::G16_B16R16_2PLANE_420_UNORM;
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self::G16_B16R16_2PLANE_422_UNORM;
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FormatFeatureFlags(pub(crate) i32);
impl FormatFeatureFlags {
    pub const fn from_raw(x: i32) -> Self {
        FormatFeatureFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE: Self = Self(1);
    pub const STORAGE_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32);
    pub const VERTEX_BUFFER: Self = Self(64);
    pub const COLOR_ATTACHMENT: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(512);
    pub const BLIT_SRC: Self = Self(1024);
    pub const BLIT_DST: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096);
}
    // VK_AMD_extension_24
    pub const RESERVED_27: Self = Self(134217728);
    pub const RESERVED_28: Self = Self(268435456);
    pub const RESERVED_25: Self = Self(33554432);
    pub const RESERVED_26: Self = Self(67108864);
    pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;
    pub const FRAGMENT_DENSITY_MAP: Self = Self(16777216);
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
    pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(8192);
    pub const TRANSFER_SRC: Self = Self::TRANSFER_SRC;
    pub const TRANSFER_DST: Self = Self::TRANSFER_DST;
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER: Self = Self(536870912);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    pub const DISJOINT: Self = Self::DISJOINT;
    pub const COSITED_CHROMA_SAMPLES: Self = Self::COSITED_CHROMA_SAMPLES;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FramebufferCreateFlags(pub(crate) i32);
impl FramebufferCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        FramebufferCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl FramebufferCreateFlags {
}
    // VK_KHR_imageless_framebuffer
    pub const IMAGELESS: Self = Self::IMAGELESS;
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
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
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
    pub const DEFAULT: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DISALLOWED: Self = Self(2);
    pub const APPLICATION_CONTROLLED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct GeometryFlagsKHR(pub(crate) i32);
impl GeometryFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        GeometryFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl GeometryFlagsKHR {
    pub const OPAQUE: Self = Self(1);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(2);
}
    // VK_NV_ray_tracing
    pub const OPAQUE: Self = Self::OPAQUE;
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct GeometryInstanceFlagsKHR(pub(crate) i32);
impl GeometryInstanceFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        GeometryInstanceFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(1);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self(2);
    pub const FORCE_OPAQUE: Self = Self(4);
    pub const FORCE_NO_OPAQUE: Self = Self(8);
}
    // VK_NV_ray_tracing
    pub const TRIANGLE_CULL_DISABLE: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    pub const FORCE_OPAQUE: Self = Self::FORCE_OPAQUE;
    pub const FORCE_NO_OPAQUE: Self = Self::FORCE_NO_OPAQUE;
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
    pub const TRIANGLES: Self = Self(0);
    pub const AABBS: Self = Self(1);
}
    // VK_KHR_ray_tracing
    pub const INSTANCES: Self = Self(1000000000);
    pub const TRIANGLES: Self = Self::TRIANGLES;
    pub const AABBS: Self = Self::AABBS;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageAspectFlags(pub(crate) i32);
impl ImageAspectFlags {
    pub const fn from_raw(x: i32) -> Self {
        ImageAspectFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageAspectFlags {
    pub const COLOR: Self = Self(1);
    pub const DEPTH: Self = Self(2);
    pub const STENCIL: Self = Self(4);
    pub const METADATA: Self = Self(8);
}
    // VK_EXT_image_drm_format_modifier
    pub const MEMORY_PLANE_0: Self = Self(128);
    pub const MEMORY_PLANE_1: Self = Self(256);
    pub const MEMORY_PLANE_2: Self = Self(512);
    pub const MEMORY_PLANE_3: Self = Self(1024);
    pub const PLANE_0: Self = Self::PLANE_0;
    pub const PLANE_1: Self = Self::PLANE_1;
    pub const PLANE_2: Self = Self::PLANE_2;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageCreateFlags(pub(crate) i32);
impl ImageCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        ImageCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageCreateFlags {
    pub const SPARSE_BINDING: Self = Self(1);
    pub const SPARSE_RESIDENCY: Self = Self(2);
    pub const SPARSE_ALIASED: Self = Self(4);
    pub const MUTABLE_FORMAT: Self = Self(8);
    pub const CUBE_COMPATIBLE: Self = Self(16);
}
    // VK_EXT_fragment_density_map
    pub const SUBSAMPLED: Self = Self(16384);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH: Self = Self(4096);
    pub const ALIAS: Self = Self::ALIAS;
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
    pub const 2D_ARRAY_COMPATIBLE: Self = Self::2D_ARRAY_COMPATIBLE;
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
    pub const EXTENDED_USAGE: Self = Self::EXTENDED_USAGE;
    pub const DISJOINT: Self = Self::DISJOINT;
    pub const CORNER_SAMPLED: Self = Self(8192);
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
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
}
    // VK_EXT_fragment_density_map
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL: Self = Self(1000000000);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
    pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
    pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
    pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
    pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
    pub const SHARED_PRESENT: Self = Self(1000000000);
    pub const PRESENT_SRC: Self = Self(1000000002);
    pub const SHADING_RATE_OPTIMAL: Self = Self(1000000003);
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
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
    // VK_EXT_image_drm_format_modifier
    pub const DRM_FORMAT_MODIFIER: Self = Self(1000000000);
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
    pub const 1D: Self = Self(0);
    pub const 2D: Self = Self(1);
    pub const 3D: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageUsageFlags(pub(crate) i32);
impl ImageUsageFlags {
    pub const fn from_raw(x: i32) -> Self {
        ImageUsageFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageUsageFlags {
    pub const TRANSFER_SRC: Self = Self(1);
    pub const TRANSFER_DST: Self = Self(2);
    pub const SAMPLED: Self = Self(4);
    pub const STORAGE: Self = Self(8);
    pub const COLOR_ATTACHMENT: Self = Self(16);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(32);
    pub const TRANSIENT_ATTACHMENT: Self = Self(64);
    pub const INPUT_ATTACHMENT: Self = Self(128);
}
    // VK_AMD_extension_24
    pub const RESERVED_13: Self = Self(8192);
    pub const RESERVED_14: Self = Self(16384);
    pub const RESERVED_15: Self = Self(32768);
    pub const RESERVED_10: Self = Self(1024);
    pub const RESERVED_11: Self = Self(2048);
    pub const RESERVED_12: Self = Self(4096);
    pub const FRAGMENT_DENSITY_MAP: Self = Self(512);
    pub const SHADING_RATE_IMAGE: Self = Self(256);
    pub const RESERVED_16: Self = Self(65536);
    pub const RESERVED_17: Self = Self(131072);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageViewCreateFlags(pub(crate) i32);
impl ImageViewCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        ImageViewCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ImageViewCreateFlags {
}
    // VK_EXT_fragment_density_map
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC: Self = Self(1);
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
    pub const 1D: Self = Self(0);
    pub const 2D: Self = Self(1);
    pub const 3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const 1D_ARRAY: Self = Self(4);
    pub const 2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
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
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
}
    // VK_EXT_index_type_uint8
    pub const UINT8: Self = Self(1000000000);
    pub const NONE: Self = Self(1000165000);
    pub const NONE: Self = Self::NONE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutUsageFlagsNV(pub(crate) i32);
impl IndirectCommandsLayoutUsageFlagsNV {
    pub const fn from_raw(x: i32) -> Self {
        IndirectCommandsLayoutUsageFlagsNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl IndirectCommandsLayoutUsageFlagsNV {
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    pub const INDEXED_SEQUENCES: Self = Self(2);
    pub const UNORDERED_SEQUENCES: Self = Self(4);
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
    pub const SHADER_GROUP: Self = Self(0);
    pub const STATE_FLAGS: Self = Self(1);
    pub const INDEX_BUFFER: Self = Self(2);
    pub const VERTEX_BUFFER: Self = Self(3);
    pub const PUSH_CONSTANT: Self = Self(4);
    pub const DRAW_INDEXED: Self = Self(5);
    pub const DRAW: Self = Self(6);
    pub const DRAW_TASKS: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IndirectStateFlagsNV(pub(crate) i32);
impl IndirectStateFlagsNV {
    pub const fn from_raw(x: i32) -> Self {
        IndirectStateFlagsNV(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl IndirectStateFlagsNV {
    pub const FRONTFACE: Self = Self(1);
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
    pub const EXECUTABLE: Self = Self(0);
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
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
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
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MemoryAllocateFlags(pub(crate) i32);
impl MemoryAllocateFlags {
    pub const fn from_raw(x: i32) -> Self {
        MemoryAllocateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl MemoryAllocateFlags {
    pub const DEVICE_MASK: Self = Self(1);
}
    // VK_KHR_buffer_device_address
    pub const DEVICE_ADDRESS: Self = Self::DEVICE_ADDRESS;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    pub const DEVICE_MASK: Self = Self::DEVICE_MASK;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MemoryHeapFlags(pub(crate) i32);
impl MemoryHeapFlags {
    pub const fn from_raw(x: i32) -> Self {
        MemoryHeapFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl MemoryHeapFlags {
    pub const DEVICE_LOCAL: Self = Self(1);
}
    // VK_KHR_device_group_creation
    pub const MULTI_INSTANCE: Self = Self::MULTI_INSTANCE;
    pub const RESERVED_2: Self = Self(4);
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
    pub const DEFAULT: Self = Self(0);
    pub const ALLOWED: Self = Self(1);
    pub const DISALLOWED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MemoryPropertyFlags(pub(crate) i32);
impl MemoryPropertyFlags {
    pub const fn from_raw(x: i32) -> Self {
        MemoryPropertyFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl MemoryPropertyFlags {
    pub const DEVICE_LOCAL: Self = Self(1);
    pub const HOST_VISIBLE: Self = Self(2);
    pub const HOST_COHERENT: Self = Self(4);
    pub const HOST_CACHED: Self = Self(8);
    pub const LAZILY_ALLOCATED: Self = Self(16);
}
    // VK_AMD_device_coherent_memory
    pub const DEVICE_COHERENT: Self = Self(64);
    pub const DEVICE_UNCACHED: Self = Self(128);
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
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
}
    // VK_EXT_debug_report
    pub const DEBUG_REPORT_CALLBACK: Self = Self(1000000000);
    pub const DEBUG_UTILS_MESSENGER: Self = Self(1000000000);
    pub const VALIDATION_CACHE: Self = Self(1000000000);
    pub const PERFORMANCE_CONFIGURATION: Self = Self(1000000000);
    pub const DEFERRED_OPERATION: Self = Self(1000000000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
    pub const DISPLAY: Self = Self(1000000000);
    pub const DISPLAY_MODE: Self = Self(1000000001);
    pub const ACCELERATION_STRUCTURE: Self = Self(1000165000);
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self::SAMPLER_YCBCR_CONVERSION;
    pub const SURFACE: Self = Self(1000000000);
    pub const SWAPCHAIN: Self = Self(1000000000);
    pub const INDIRECT_COMMANDS_LAYOUT: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE: Self = Self::ACCELERATION_STRUCTURE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PeerMemoryFeatureFlags(pub(crate) i32);
impl PeerMemoryFeatureFlags {
    pub const fn from_raw(x: i32) -> Self {
        PeerMemoryFeatureFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PeerMemoryFeatureFlags {
    pub const COPY_SRC: Self = Self(1);
    pub const COPY_DST: Self = Self(2);
    pub const GENERIC_SRC: Self = Self(4);
    pub const GENERIC_DST: Self = Self(8);
}
    // VK_KHR_device_group
    pub const COPY_SRC: Self = Self::COPY_SRC;
    pub const COPY_DST: Self = Self::COPY_DST;
    pub const GENERIC_SRC: Self = Self::GENERIC_SRC;
    pub const GENERIC_DST: Self = Self::GENERIC_DST;
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
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PerformanceCounterDescriptionFlagsKHR(pub(crate) i32);
impl PerformanceCounterDescriptionFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        PerformanceCounterDescriptionFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PerformanceCounterDescriptionFlagsKHR {
    pub const PERFORMANCE_IMPACTING: Self = Self(1);
    pub const CONCURRENTLY_IMPACTED: Self = Self(2);
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
    pub const COMMAND_BUFFER: Self = Self(0);
    pub const RENDER_PASS: Self = Self(1);
    pub const COMMAND: Self = Self(2);
    pub const QUERY_SCOPE_COMMAND_BUFFER: Self = Self::COMMAND_BUFFER;
    pub const QUERY_SCOPE_RENDER_PASS: Self = Self::RENDER_PASS;
    pub const QUERY_SCOPE_COMMAND: Self = Self::COMMAND;
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
    pub const INT32: Self = Self(0);
    pub const INT64: Self = Self(1);
    pub const UINT32: Self = Self(2);
    pub const UINT64: Self = Self(3);
    pub const FLOAT32: Self = Self(4);
    pub const FLOAT64: Self = Self(5);
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
    pub const GENERIC: Self = Self(0);
    pub const PERCENTAGE: Self = Self(1);
    pub const NANOSECONDS: Self = Self(2);
    pub const BYTES: Self = Self(3);
    pub const BYTES_PER_SECOND: Self = Self(4);
    pub const KELVIN: Self = Self(5);
    pub const WATTS: Self = Self(6);
    pub const VOLTS: Self = Self(7);
    pub const AMPS: Self = Self(8);
    pub const HERTZ: Self = Self(9);
    pub const CYCLES: Self = Self(10);
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
    pub const NULL_HARDWARE: Self = Self(0);
    pub const FLUSH_GPU_CACHES: Self = Self(1);
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
    pub const HW_COUNTERS_SUPPORTED: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS: Self = Self(1);
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
    pub const UINT32: Self = Self(0);
    pub const UINT64: Self = Self(1);
    pub const FLOAT: Self = Self(2);
    pub const BOOL: Self = Self(3);
    pub const STRING: Self = Self(4);
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
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
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
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
}
    // VK_KHR_ray_tracing
    pub const RAY_TRACING: Self = Self(1000165000);
    pub const RAY_TRACING: Self = Self::RAY_TRACING;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCacheCreateFlags(pub(crate) i32);
impl PipelineCacheCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        PipelineCacheCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineCacheCreateFlags {
}
    // VK_EXT_pipeline_creation_cache_control
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1);
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
    pub const ONE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagsAMD(pub(crate) i32);
impl PipelineCompilerControlFlagsAMD {
    pub const fn from_raw(x: i32) -> Self {
        PipelineCompilerControlFlagsAMD(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineCompilerControlFlagsAMD {
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCreateFlags(pub(crate) i32);
impl PipelineCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        PipelineCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION: Self = Self(1);
    pub const ALLOW_DERIVATIVES: Self = Self(2);
    pub const DERIVATIVE: Self = Self(4);
}
    // VK_EXT_pipeline_creation_cache_control
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(256);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(512);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
    pub const DISPATCH_BASE: Self = Self::DISPATCH_BASE;
    pub const CAPTURE_STATISTICS: Self = Self(64);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS: Self = Self(128);
    pub const LIBRARY: Self = Self(2048);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS: Self = Self(16384);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS: Self = Self(32768);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS: Self = Self(65536);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS: Self = Self(131072);
    pub const RAY_TRACING_SKIP_TRIANGLES: Self = Self(4096);
    pub const RAY_TRACING_SKIP_AABBS: Self = Self(8192);
    pub const INDIRECT_BINDABLE: Self = Self(262144);
    pub const DEFER_COMPILE: Self = Self(32);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCreationFeedbackFlagsEXT(pub(crate) i32);
impl PipelineCreationFeedbackFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        PipelineCreationFeedbackFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineCreationFeedbackFlagsEXT {
    pub const VALID: Self = Self(1);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(2);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(4);
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
    pub const BOOL32: Self = Self(0);
    pub const INT64: Self = Self(1);
    pub const UINT64: Self = Self(2);
    pub const FLOAT64: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineShaderStageCreateFlags(pub(crate) i32);
impl PipelineShaderStageCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        PipelineShaderStageCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineShaderStageCreateFlags {
}
    // VK_EXT_subgroup_size_control
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(1);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(2);
    pub const RESERVED_3: Self = Self(8);
    pub const RESERVED_2: Self = Self(4);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineStageFlags(pub(crate) i32);
impl PipelineStageFlags {
    pub const fn from_raw(x: i32) -> Self {
        PipelineStageFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl PipelineStageFlags {
    pub const TOP_OF_PIPE: Self = Self(1);
    pub const DRAW_INDIRECT: Self = Self(2);
    pub const VERTEX_INPUT: Self = Self(4);
    pub const VERTEX_SHADER: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(32);
    pub const GEOMETRY_SHADER: Self = Self(64);
    pub const FRAGMENT_SHADER: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1024);
    pub const COMPUTE_SHADER: Self = Self(2048);
    pub const TRANSFER: Self = Self(4096);
    pub const BOTTOM_OF_PIPE: Self = Self(8192);
    pub const HOST: Self = Self(16384);
    pub const ALL_GRAPHICS: Self = Self(32768);
    pub const ALL_COMMANDS: Self = Self(65536);
}
    // VK_AMD_extension_24
    pub const RESERVED_27: Self = Self(134217728);
    pub const RESERVED_26: Self = Self(67108864);
    pub const CONDITIONAL_RENDERING: Self = Self(262144);
    pub const FRAGMENT_DENSITY_PROCESS: Self = Self(8388608);
    pub const TRANSFORM_FEEDBACK: Self = Self(16777216);
    pub const RAY_TRACING_SHADER: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_BUILD: Self = Self(33554432);
    pub const COMMAND_PREPROCESS: Self = Self(131072);
    pub const TASK_SHADER: Self = Self(524288);
    pub const MESH_SHADER: Self = Self(1048576);
    pub const RAY_TRACING_SHADER: Self = Self::RAY_TRACING_SHADER;
    pub const ACCELERATION_STRUCTURE_BUILD: Self = Self::ACCELERATION_STRUCTURE_BUILD;
    pub const SHADING_RATE_IMAGE: Self = Self(4194304);
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
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
    // VK_KHR_maintenance2
    pub const ALL_CLIP_PLANES: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY: Self = Self::USER_CLIP_PLANES_ONLY;
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
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
}
    // VK_NV_fill_rectangle
    pub const FILL_RECTANGLE: Self = Self(1000000000);
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
    pub const IMMEDIATE: Self = Self(0);
    pub const MAILBOX: Self = Self(1);
    pub const FIFO: Self = Self(2);
    pub const FIFO_RELAXED: Self = Self(3);
}
    // VK_KHR_shared_presentable_image
    pub const SHARED_DEMAND_REFRESH: Self = Self(1000000000);
    pub const SHARED_CONTINUOUS_REFRESH: Self = Self(1000000001);
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
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueryControlFlags(pub(crate) i32);
impl QueryControlFlags {
    pub const fn from_raw(x: i32) -> Self {
        QueryControlFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueryControlFlags {
    pub const PRECISE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueryPipelineStatisticFlags(pub(crate) i32);
impl QueryPipelineStatisticFlags {
    pub const fn from_raw(x: i32) -> Self {
        QueryPipelineStatisticFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueryPipelineStatisticFlags {
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1);
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(2);
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(4);
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(8);
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(16);
    pub const CLIPPING_INVOCATIONS: Self = Self(32);
    pub const CLIPPING_PRIMITIVES: Self = Self(64);
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(128);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(256);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(512);
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1024);
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
    pub const MANUAL: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueryResultFlags(pub(crate) i32);
impl QueryResultFlags {
    pub const fn from_raw(x: i32) -> Self {
        QueryResultFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueryResultFlags {
    pub const 64: Self = Self(1);
    pub const WAIT: Self = Self(2);
    pub const WITH_AVAILABILITY: Self = Self(4);
    pub const PARTIAL: Self = Self(8);
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
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
}
    // VK_AMD_extension_24
    pub const RESERVED_8: Self = Self(1000000008);
    pub const RESERVED_4: Self = Self(1000000004);
    pub const TRANSFORM_FEEDBACK_STREAM: Self = Self(1000000004);
    pub const PERFORMANCE_QUERY: Self = Self(1000000000);
    pub const PERFORMANCE_QUERY: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE: Self = Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueueFlags(pub(crate) i32);
impl QueueFlags {
    pub const fn from_raw(x: i32) -> Self {
        QueueFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl QueueFlags {
    pub const GRAPHICS: Self = Self(1);
    pub const COMPUTE: Self = Self(2);
    pub const TRANSFER: Self = Self(4);
    pub const SPARSE_BINDING: Self = Self(8);
}
    // VK_AMD_extension_24
    pub const RESERVED_6: Self = Self(64);
    pub const RESERVED_5: Self = Self(32);
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
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1024);
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
    pub const STRICT: Self = Self(0);
    pub const RELAXED: Self = Self(1);
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
    pub const GENERAL: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP: Self = Self(2);
}
    // VK_NV_ray_tracing
    pub const GENERAL: Self = Self::GENERAL;
    pub const TRIANGLES_HIT_GROUP: Self = Self::TRIANGLES_HIT_GROUP;
    pub const PROCEDURAL_HIT_GROUP: Self = Self::PROCEDURAL_HIT_GROUP;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RenderPassCreateFlags(pub(crate) i32);
impl RenderPassCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        RenderPassCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl RenderPassCreateFlags {
}
    // VK_KHR_extension_221
    pub const RESERVED_0: Self = Self(1);
    pub const TRANSFORM: Self = Self(2);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ResolveModeFlags(pub(crate) i32);
impl ResolveModeFlags {
    pub const fn from_raw(x: i32) -> Self {
        ResolveModeFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ResolveModeFlags {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO: Self = Self(1);
    pub const AVERAGE: Self = Self(2);
    pub const MIN: Self = Self(4);
    pub const MAX: Self = Self(8);
}
    // VK_KHR_depth_stencil_resolve
    pub const NONE: Self = Self::NONE;
    pub const SAMPLE_ZERO: Self = Self::SAMPLE_ZERO;
    pub const AVERAGE: Self = Self::AVERAGE;
    pub const MIN: Self = Self::MIN;
    pub const MAX: Self = Self::MAX;
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
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const ERROR_UNKNOWN: Self = Self(-13);
}
    // VK_EXT_buffer_device_address
    pub const ERROR_INVALID_DEVICE_ADDRESS: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    pub const ERROR_VALIDATION_FAILED: Self = Self(-1000000001);
    pub const ERROR_FRAGMENTATION: Self = Self::ERROR_FRAGMENTATION;
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST: Self = Self(-1000000000);
    pub const ERROR_NOT_PERMITTED: Self = Self(-1000000001);
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT: Self = Self(-1000000000);
    pub const ERROR_PIPELINE_COMPILE_REQUIRED: Self = Self(1000000000);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    pub const THREAD_IDLE: Self = Self(1000000000);
    pub const THREAD_DONE: Self = Self(1000000001);
    pub const OPERATION_DEFERRED: Self = Self(1000000002);
    pub const OPERATION_NOT_DEFERRED: Self = Self(1000000003);
    pub const ERROR_INCOMPATIBLE_DISPLAY: Self = Self(-1000000001);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
    pub const ERROR_INCOMPATIBLE_VERSION: Self = Self(-1000000000);
    pub const ERROR_SURFACE_LOST: Self = Self(-1000000000);
    pub const ERROR_NATIVE_WINDOW_IN_USE: Self = Self(-1000000001);
    pub const SUBOPTIMAL: Self = Self(1000000003);
    pub const ERROR_OUT_OF_DATE: Self = Self(-1000000004);
    pub const ERROR_INVALID_SHADER: Self = Self(-1000000000);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SampleCountFlags(pub(crate) i32);
impl SampleCountFlags {
    pub const fn from_raw(x: i32) -> Self {
        SampleCountFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SampleCountFlags {
    pub const 1: Self = Self(1);
    pub const 2: Self = Self(2);
    pub const 4: Self = Self(4);
    pub const 8: Self = Self(8);
    pub const 16: Self = Self(16);
    pub const 32: Self = Self(32);
    pub const 64: Self = Self(64);
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
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
}
    // VK_KHR_sampler_mirror_clamp_to_edge
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self::MIRROR_CLAMP_TO_EDGE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerCreateFlags(pub(crate) i32);
impl SamplerCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        SamplerCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SamplerCreateFlags {
}
    // VK_EXT_fragment_density_map
    pub const SUBSAMPLED: Self = Self(1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION: Self = Self(2);
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
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
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
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
}
    // VK_EXT_sampler_filter_minmax
    pub const WEIGHTED_AVERAGE: Self = Self::WEIGHTED_AVERAGE;
    pub const MIN: Self = Self::MIN;
    pub const MAX: Self = Self::MAX;
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
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
}
    // VK_KHR_sampler_ycbcr_conversion
    pub const RGB_IDENTITY: Self = Self::RGB_IDENTITY;
    pub const YCBCR_IDENTITY: Self = Self::YCBCR_IDENTITY;
    pub const YCBCR_709: Self = Self::YCBCR_709;
    pub const YCBCR_601: Self = Self::YCBCR_601;
    pub const YCBCR_2020: Self = Self::YCBCR_2020;
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
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
}
    // VK_KHR_sampler_ycbcr_conversion
    pub const ITU_FULL: Self = Self::ITU_FULL;
    pub const ITU_NARROW: Self = Self::ITU_NARROW;
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
    pub const DEVICE: Self = Self(1);
    pub const WORKGROUP: Self = Self(2);
    pub const SUBGROUP: Self = Self(3);
    pub const QUEUE_FAMILY: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreCreateFlags(pub(crate) i32);
impl SemaphoreCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        SemaphoreCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SemaphoreCreateFlags {
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreImportFlags(pub(crate) i32);
impl SemaphoreImportFlags {
    pub const fn from_raw(x: i32) -> Self {
        SemaphoreImportFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SemaphoreImportFlags {
    pub const TEMPORARY: Self = Self(1);
}
    // VK_KHR_external_semaphore
    pub const TEMPORARY: Self = Self::TEMPORARY;
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
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}
    // VK_KHR_timeline_semaphore
    pub const BINARY: Self = Self::BINARY;
    pub const TIMELINE: Self = Self::TIMELINE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SemaphoreWaitFlags(pub(crate) i32);
impl SemaphoreWaitFlags {
    pub const fn from_raw(x: i32) -> Self {
        SemaphoreWaitFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SemaphoreWaitFlags {
    pub const ANY: Self = Self(1);
}
    // VK_KHR_timeline_semaphore
    pub const ANY: Self = Self::ANY;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShaderCorePropertiesFlagsAMD(pub(crate) i32);
impl ShaderCorePropertiesFlagsAMD {
    pub const fn from_raw(x: i32) -> Self {
        ShaderCorePropertiesFlagsAMD(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ShaderCorePropertiesFlagsAMD {
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
    pub const 32_BIT_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
}
    // VK_KHR_shader_float_controls
    pub const 32_BIT_ONLY: Self = Self::32_BIT_ONLY;
    pub const ALL: Self = Self::ALL;
    pub const NONE: Self = Self::NONE;
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
    pub const STATISTICS: Self = Self(0);
    pub const BINARY: Self = Self(1);
    pub const DISASSEMBLY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShaderModuleCreateFlags(pub(crate) i32);
impl ShaderModuleCreateFlags {
    pub const fn from_raw(x: i32) -> Self {
        ShaderModuleCreateFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ShaderModuleCreateFlags {
}
    // VK_NV_extension_52
    pub const RESERVED_0: Self = Self(1);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShaderStageFlags(pub(crate) i32);
impl ShaderStageFlags {
    pub const fn from_raw(x: i32) -> Self {
        ShaderStageFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ShaderStageFlags {
    pub const VERTEX: Self = Self(1);
    pub const TESSELLATION_CONTROL: Self = Self(2);
    pub const TESSELLATION_EVALUATION: Self = Self(4);
    pub const GEOMETRY: Self = Self(8);
    pub const FRAGMENT: Self = Self(16);
    pub const COMPUTE: Self = Self(32);
    pub const ALL_GRAPHICS: Self = Self(31);
    pub const ALL: Self = Self(2147483647);
}
    // VK_KHR_ray_tracing
    pub const RAYGEN: Self = Self(256);
    pub const ANY_HIT: Self = Self(512);
    pub const CLOSEST_HIT: Self = Self(1024);
    pub const MISS: Self = Self(2048);
    pub const INTERSECTION: Self = Self(4096);
    pub const CALLABLE: Self = Self(8192);
    pub const TASK: Self = Self(64);
    pub const MESH: Self = Self(128);
    pub const RAYGEN: Self = Self::RAYGEN;
    pub const ANY_HIT: Self = Self::ANY_HIT;
    pub const CLOSEST_HIT: Self = Self::CLOSEST_HIT;
    pub const MISS: Self = Self::MISS;
    pub const INTERSECTION: Self = Self::INTERSECTION;
    pub const CALLABLE: Self = Self::CALLABLE;
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
    pub const NO_INVOCATIONS: Self = Self(0);
    pub const 16_INVOCATIONS_PER_PIXEL: Self = Self(1);
    pub const 8_INVOCATIONS_PER_PIXEL: Self = Self(2);
    pub const 4_INVOCATIONS_PER_PIXEL: Self = Self(3);
    pub const 2_INVOCATIONS_PER_PIXEL: Self = Self(4);
    pub const 1_INVOCATION_PER_PIXEL: Self = Self(5);
    pub const 1_INVOCATION_PER_2X1_PIXELS: Self = Self(6);
    pub const 1_INVOCATION_PER_1X2_PIXELS: Self = Self(7);
    pub const 1_INVOCATION_PER_2X2_PIXELS: Self = Self(8);
    pub const 1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    pub const 1_INVOCATION_PER_2X4_PIXELS: Self = Self(10);
    pub const 1_INVOCATION_PER_4X4_PIXELS: Self = Self(11);
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
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SparseImageFormatFlags(pub(crate) i32);
impl SparseImageFormatFlags {
    pub const fn from_raw(x: i32) -> Self {
        SparseImageFormatFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SparseImageFormatFlags {
    pub const SINGLE_MIPTAIL: Self = Self(1);
    pub const ALIGNED_MIP_SIZE: Self = Self(2);
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SparseMemoryBindFlags(pub(crate) i32);
impl SparseMemoryBindFlags {
    pub const fn from_raw(x: i32) -> Self {
        SparseMemoryBindFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SparseMemoryBindFlags {
    pub const METADATA: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StencilFaceFlags(pub(crate) i32);
impl StencilFaceFlags {
    pub const fn from_raw(x: i32) -> Self {
        StencilFaceFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl StencilFaceFlags {
    pub const FRONT: Self = Self(1);
    pub const BACK: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
    pub const FRONT_AND_BACK: Self = Self::FRONT_AND_BACK;
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
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
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
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
}
    // VK_AMD_device_coherent_memory
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES: Self = Self(1000000000);
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES: Self = Self(1000000000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO: Self = Self(1000000001);
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO: Self = Self(1000000000);
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO: Self = Self(1000000000);
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2: Self = Self(1000000000);
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES: Self = Self(1000000000);
    pub const ANDROID_HARDWARE_BUFFER_USAGE: Self = Self(1000000000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES: Self = Self(1000000001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES: Self = Self(1000000002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO: Self = Self(1000000003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO: Self = Self(1000000004);
    pub const EXTERNAL_FORMAT: Self = Self(1000000005);
    pub const NATIVE_BUFFER: Self = Self(1000000000);
    pub const SWAPCHAIN_IMAGE_CREATE_INFO: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_PRESENTATION_PROPERTIES: Self = Self(1000000002);
    pub const IMAGE_VIEW_ASTC_DECODE_MODE: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES: Self = Self(1000000001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES: Self = Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO: Self = Self(1000000002);
    pub const CALIBRATED_TIMESTAMP_INFO: Self = Self(1000000000);
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES: Self = Self(1000000001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES: Self = Self(1000000000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const DEBUG_MARKER_OBJECT_NAME_INFO: Self = Self(1000000000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO: Self = Self(1000000001);
    pub const DEBUG_MARKER_MARKER_INFO: Self = Self(1000000002);
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO: Self = Self(1000000000);
    pub const DEBUG_REPORT_CREATE_INFO: Self = Self::DEBUG_REPORT_CALLBACK_CREATE_INFO;
    pub const DEBUG_UTILS_OBJECT_NAME_INFO: Self = Self(1000000000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO: Self = Self(1000000001);
    pub const DEBUG_UTILS_LABEL: Self = Self(1000000002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA: Self = Self(1000000003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO: Self = Self(1000000004);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES: Self = Self(1000000000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const DISPLAY_POWER_INFO: Self = Self(1000000000);
    pub const DEVICE_EVENT_INFO: Self = Self(1000000001);
    pub const DISPLAY_EVENT_INFO: Self = Self(1000000002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO: Self = Self(1000000003);
    pub const SURFACE_CAPABILITIES_2: Self = Self(1000000000);
    pub const SURFACE_CAPABILITIES2: Self = Self::SURFACE_CAPABILITIES_2;
    pub const IMPORT_MEMORY_HOST_POINTER_INFO: Self = Self(1000000000);
    pub const MEMORY_HOST_POINTER_PROPERTIES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO: Self = Self(1000000000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES: Self = Self(1000000001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES: Self = Self(1000000000);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO: Self = Self(1000000000);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE: Self = Self(1000000002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO: Self = Self(1000000001);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000000000);
    pub const HDR_METADATA: Self = Self(1000000000);
    pub const HEADLESS_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST: Self = Self(1000000000);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO: Self = Self(1000000002);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO: Self = Self(1000000003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO: Self = Self(1000000004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES: Self = Self(1000000005);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000000001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000000002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000000003);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES: Self = Self(1000000000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO: Self = Self(1000000001);
    pub const METAL_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000000000);
    pub const SAMPLE_LOCATIONS_INFO: Self = Self(1000000000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO: Self = Self(1000000001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES: Self = Self(1000000003);
    pub const MULTISAMPLE_PROPERTIES: Self = Self(1000000004);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000000000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES: Self = Self(1000000001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO: Self = Self(1000000002);
    pub const VALIDATION_CACHE_CREATE_INFO: Self = Self(1000000000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO: Self = Self(1000000001);
    pub const VALIDATION_FEATURES: Self = Self(1000000000);
    pub const VALIDATION_FLAGS: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000000000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES: Self = Self(1000000000);
    pub const IMAGEPIPE_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const PRESENT_FRAME_TOKEN: Self = Self(1000000000);
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const PRESENT_TIMES_INFO: Self = Self(1000000000);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(1000000000);
    pub const INITIALIZE_PERFORMANCE_API_INFO: Self = Self(1000000001);
    pub const PERFORMANCE_MARKER_INFO: Self = Self(1000000002);
    pub const PERFORMANCE_STREAM_MARKER_INFO: Self = Self(1000000003);
    pub const PERFORMANCE_OVERRIDE_INFO: Self = Self(1000000004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO: Self = Self(1000000005);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
    pub const ANDROID_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self::BIND_IMAGE_MEMORY_INFO;
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self::ATTACHMENT_DESCRIPTION_2;
    pub const ATTACHMENT_REFERENCE_2: Self = Self::ATTACHMENT_REFERENCE_2;
    pub const SUBPASS_DESCRIPTION_2: Self = Self::SUBPASS_DESCRIPTION_2;
    pub const SUBPASS_DEPENDENCY_2: Self = Self::SUBPASS_DEPENDENCY_2;
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self::RENDER_PASS_CREATE_INFO_2;
    pub const SUBPASS_BEGIN_INFO: Self = Self::SUBPASS_BEGIN_INFO;
    pub const SUBPASS_END_INFO: Self = Self::SUBPASS_END_INFO;
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
    pub const DEFERRED_OPERATION_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES: Self = Self(1000000007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO: Self = Self(1000000008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO: Self = Self(1000000009);
    pub const ACQUIRE_NEXT_IMAGE_INFO: Self = Self(1000000010);
    pub const DEVICE_GROUP_PRESENT_INFO: Self = Self(1000000011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO: Self = Self(1000000012);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
    pub const DISPLAY_MODE_CREATE_INFO: Self = Self(1000000000);
    pub const DISPLAY_SURFACE_CREATE_INFO: Self = Self(1000000001);
    pub const DISPLAY_PRESENT_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self::EXPORT_FENCE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self::EXTERNAL_FENCE_PROPERTIES;
    pub const IMPORT_FENCE_FD_INFO: Self = Self(1000000000);
    pub const FENCE_GET_FD_INFO: Self = Self(1000000001);
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO: Self = Self(1000000000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO: Self = Self(1000000001);
    pub const FENCE_GET_WIN32_HANDLE_INFO: Self = Self(1000000002);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
    pub const IMPORT_MEMORY_FD_INFO: Self = Self(1000000000);
    pub const MEMORY_FD_PROPERTIES: Self = Self(1000000001);
    pub const MEMORY_GET_FD_INFO: Self = Self(1000000002);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO: Self = Self(1000000000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO: Self = Self(1000000001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES: Self = Self(1000000002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO: Self = Self(1000000003);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
    pub const IMPORT_SEMAPHORE_FD_INFO: Self = Self(1000000000);
    pub const SEMAPHORE_GET_FD_INFO: Self = Self(1000000001);
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO: Self = Self(1000000000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO: Self = Self(1000000001);
    pub const D3D12_FENCE_SUBMIT_INFO: Self = Self(1000000002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO: Self = Self(1000000003);
    pub const DISPLAY_PROPERTIES_2: Self = Self(1000000000);
    pub const DISPLAY_PLANE_PROPERTIES_2: Self = Self(1000000001);
    pub const DISPLAY_MODE_PROPERTIES_2: Self = Self(1000000002);
    pub const DISPLAY_PLANE_INFO_2: Self = Self(1000000003);
    pub const DISPLAY_PLANE_CAPABILITIES_2: Self = Self(1000000004);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    pub const MEMORY_REQUIREMENTS_2: Self = Self::MEMORY_REQUIREMENTS_2;
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
    pub const FORMAT_PROPERTIES_2: Self = Self::FORMAT_PROPERTIES_2;
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2: Self = Self(1000000000);
    pub const SURFACE_CAPABILITIES_2: Self = Self(1000000001);
    pub const SURFACE_FORMAT_2: Self = Self(1000000002);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
    pub const PRESENT_REGIONS: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES: Self = Self(1000000001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO: Self = Self(1000000002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO: Self = Self(1000000003);
    pub const ACQUIRE_PROFILING_LOCK_INFO: Self = Self(1000000004);
    pub const PERFORMANCE_COUNTER: Self = Self(1000000005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION: Self = Self(1000000006);
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_INFO: Self = Self(1000000001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES: Self = Self(1000000002);
    pub const PIPELINE_EXECUTABLE_INFO: Self = Self(1000000003);
    pub const PIPELINE_EXECUTABLE_STATISTIC: Self = Self(1000000004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION: Self = Self(1000000005);
    pub const PIPELINE_LIBRARY_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000000000);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO: Self = Self(1000165006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE: Self = Self(1000165007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO: Self = Self(1000000001);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO: Self = Self(1000000002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA: Self = Self(1000000003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA: Self = Self(1000000004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA: Self = Self(1000000005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY: Self = Self(1000000006);
    pub const ACCELERATION_STRUCTURE_INFO: Self = Self(1000000007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000000008);
    pub const ACCELERATION_STRUCTURE_VERSION: Self = Self(1000000009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO: Self = Self(1000000010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO: Self = Self(1000000011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO: Self = Self(1000000012);
    pub const PHYSICAL_DEVICE_RAY_TRACING_FEATURES: Self = Self(1000000013);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES: Self = Self(1000000014);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO: Self = Self(1000000015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO: Self = Self(1000000016);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO: Self = Self(1000000017);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO: Self = Self(1000000018);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES: Self = Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES: Self = Self(1000000000);
    pub const SURFACE_PROTECTED_CAPABILITIES: Self = Self(1000000000);
    pub const SWAPCHAIN_CREATE_INFO: Self = Self(1000000000);
    pub const PRESENT_INFO: Self = Self(1000000001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES: Self = Self(1000060007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO: Self = Self(1000060008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO: Self = Self(1000060009);
    pub const ACQUIRE_NEXT_IMAGE_INFO: Self = Self(1000060010);
    pub const DEVICE_GROUP_PRESENT_INFO: Self = Self(1000060011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO: Self = Self(1000060012);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    pub const SEMAPHORE_WAIT_INFO: Self = Self::SEMAPHORE_WAIT_INFO;
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self::SEMAPHORE_SIGNAL_INFO;
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self::PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES;
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
    pub const WAYLAND_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO: Self = Self(1000000000);
    pub const WIN32_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const XCB_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const XLIB_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const IOS_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const MACOS_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const VI_SURFACE_CREATE_INFO: Self = Self(1000000000);
    pub const IMAGE_VIEW_HANDLE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES: Self = Self(1000000000);
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES: Self = Self(1000000000);
    pub const COOPERATIVE_MATRIX_PROPERTIES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION: Self = Self(1000000002);
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO: Self = Self(1000000000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO: Self = Self(1000000001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES: Self = Self(1000000000);
    pub const CHECKPOINT_DATA: Self = Self(1000000000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES: Self = Self(1000000000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES: Self = Self(1000000000);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO: Self = Self(1000000001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO: Self = Self(1000000002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN: Self = Self(1000000003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO: Self = Self(1000000004);
    pub const GENERATED_COMMANDS_INFO: Self = Self(1000000005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO: Self = Self(1000000006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES: Self = Self(1000000007);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000000000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000000001);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO: Self = Self(1000000000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO: Self = Self(1000000001);
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES: Self = Self(1000000001);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO: Self = Self(1000000000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO: Self = Self(1000000001);
    pub const GEOMETRY: Self = Self(1000000003);
    pub const GEOMETRY_TRIANGLES: Self = Self(1000000004);
    pub const GEOMETRY_AABB: Self = Self(1000000005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO: Self = Self::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO;
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE: Self = Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE;
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000000008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES: Self = Self(1000000009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO: Self = Self(1000000011);
    pub const ACCELERATION_STRUCTURE_INFO: Self = Self(1000000012);
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES: Self = Self(1000000000);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO: Self = Self(1000000001);
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES: Self = Self(1000000002);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES: Self = Self(1000000001);
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO: Self = Self(1000000000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES: Self = Self(1000000001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES: Self = Self(1000000002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO: Self = Self(1000000005);
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO: Self = Self(1000000000);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO: Self = Self(1000000000);
    pub const RESERVED: Self = Self(1000000000);
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO: Self = Self(1000000000);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO: Self = Self(1000000001);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubgroupFeatureFlags(pub(crate) i32);
impl SubgroupFeatureFlags {
    pub const fn from_raw(x: i32) -> Self {
        SubgroupFeatureFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SubgroupFeatureFlags {
    pub const BASIC: Self = Self(1);
    pub const VOTE: Self = Self(2);
    pub const ARITHMETIC: Self = Self(4);
    pub const BALLOT: Self = Self(8);
    pub const SHUFFLE: Self = Self(16);
    pub const SHUFFLE_RELATIVE: Self = Self(32);
    pub const CLUSTERED: Self = Self(64);
    pub const QUAD: Self = Self(128);
}
    // VK_NV_shader_subgroup_partitioned
    pub const PARTITIONED: Self = Self(256);
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
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubpassDescriptionFlags(pub(crate) i32);
impl SubpassDescriptionFlags {
    pub const fn from_raw(x: i32) -> Self {
        SubpassDescriptionFlags(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SubpassDescriptionFlags {
}
    // VK_NVX_multiview_per_view_attributes
    pub const PER_VIEW_ATTRIBUTES: Self = Self(1);
    pub const PER_VIEW_POSITION_X_ONLY: Self = Self(2);
    pub const RESERVED_2: Self = Self(4);
    pub const RESERVED_3: Self = Self(8);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SurfaceCounterFlagsEXT(pub(crate) i32);
impl SurfaceCounterFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        SurfaceCounterFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SurfaceCounterFlagsEXT {
    pub const VBLANK: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SurfaceTransformFlagsKHR(pub(crate) i32);
impl SurfaceTransformFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        SurfaceTransformFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SurfaceTransformFlagsKHR {
    pub const IDENTITY: Self = Self(1);
    pub const ROTATE_90: Self = Self(2);
    pub const ROTATE_180: Self = Self(4);
    pub const ROTATE_270: Self = Self(8);
    pub const HORIZONTAL_MIRROR: Self = Self(16);
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(32);
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = Self(64);
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = Self(128);
    pub const INHERIT: Self = Self(256);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SwapchainCreateFlagsKHR(pub(crate) i32);
impl SwapchainCreateFlagsKHR {
    pub const fn from_raw(x: i32) -> Self {
        SwapchainCreateFlagsKHR(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SwapchainCreateFlagsKHR {
}
    // VK_KHR_device_group
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    pub const PROTECTED: Self = Self(2);
    pub const MUTABLE_FORMAT: Self = Self(4);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SwapchainImageUsageFlagsANDROID(pub(crate) i32);
impl SwapchainImageUsageFlagsANDROID {
    pub const fn from_raw(x: i32) -> Self {
        SwapchainImageUsageFlagsANDROID(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl SwapchainImageUsageFlagsANDROID {
    pub const SHARED: Self = Self(1);
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
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
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
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
    // VK_KHR_maintenance2
    pub const UPPER_LEFT: Self = Self::UPPER_LEFT;
    pub const LOWER_LEFT: Self = Self::LOWER_LEFT;
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
    pub const DEVICE: Self = Self(0);
    pub const CLOCK_MONOTONIC: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ToolPurposeFlagsEXT(pub(crate) i32);
impl ToolPurposeFlagsEXT {
    pub const fn from_raw(x: i32) -> Self {
        ToolPurposeFlagsEXT(x)
    }
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl ToolPurposeFlagsEXT {
    pub const VALIDATION: Self = Self(1);
    pub const PROFILING: Self = Self(2);
    pub const TRACING: Self = Self(4);
    pub const ADDITIONAL_FEATURES: Self = Self(8);
    pub const MODIFYING_FEATURES: Self = Self(16);
}
    // VK_EXT_tooling_info
    pub const DEBUG_REPORTING: Self = Self(32);
    pub const DEBUG_MARKERS: Self = Self(64);
    pub const DEBUG_REPORTING: Self = Self(32);
    pub const DEBUG_MARKERS: Self = Self(64);
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
    pub const ONE: Self = Self(1);
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
    pub const ALL: Self = Self(0);
    pub const SHADERS: Self = Self(1);
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
    pub const ALL: Self = Self(0);
    pub const SHADERS: Self = Self(1);
    pub const THREAD_SAFETY: Self = Self(2);
    pub const API_PARAMETERS: Self = Self(3);
    pub const OBJECT_LIFETIMES: Self = Self(4);
    pub const CORE_CHECKS: Self = Self(5);
    pub const UNIQUE_HANDLES: Self = Self(6);
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
    pub const GPU_ASSISTED: Self = Self(0);
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = Self(1);
    pub const BEST_PRACTICES: Self = Self(2);
    pub const DEBUG_PRINTF: Self = Self(3);
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
    pub const VIV: Self = Self(65537);
    pub const VSI: Self = Self(65538);
    pub const KAZAN: Self = Self(65539);
    pub const CODEPLAY: Self = Self(65540);
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
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
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
    pub const POSITIVE_X: Self = Self(0);
    pub const NEGATIVE_X: Self = Self(1);
    pub const POSITIVE_Y: Self = Self(2);
    pub const NEGATIVE_Y: Self = Self(3);
    pub const POSITIVE_Z: Self = Self(4);
    pub const NEGATIVE_Z: Self = Self(5);
    pub const POSITIVE_W: Self = Self(6);
    pub const NEGATIVE_W: Self = Self(7);
}
