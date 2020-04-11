use std::fmt;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageLayout.html>"]
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
    #[doc = "Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)"]
    pub const UNDEFINED: Self = ImageLayout(0);
    #[doc = "General layout when image can be used for any kind of access"]
    pub const GENERAL: Self = ImageLayout(1);
    #[doc = "Optimal layout when image is only used for color attachment read/write"]
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = ImageLayout(2);
    #[doc = "Optimal layout when image is only used for depth/stencil attachment read/write"]
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = ImageLayout(3);
    #[doc = "Optimal layout when image is used for read only depth/stencil attachment and shader access"]
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = ImageLayout(4);
    #[doc = "Optimal layout when image is used for read only shader access"]
    pub const SHADER_READ_ONLY_OPTIMAL: Self = ImageLayout(5);
    #[doc = "Optimal layout when image is used only as source of transfer operations"]
    pub const TRANSFER_SRC_OPTIMAL: Self = ImageLayout(6);
    #[doc = "Optimal layout when image is used only as destination of transfer operations"]
    pub const TRANSFER_DST_OPTIMAL: Self = ImageLayout(7);
    #[doc = "Initial layout used when the data is populated by the CPU"]
    pub const PREINITIALIZED: Self = ImageLayout(8);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentLoadOp.html>"]
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
    pub const LOAD: Self = AttachmentLoadOp(0);
    pub const CLEAR: Self = AttachmentLoadOp(1);
    pub const DONT_CARE: Self = AttachmentLoadOp(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentStoreOp.html>"]
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
    pub const STORE: Self = AttachmentStoreOp(0);
    pub const DONT_CARE: Self = AttachmentStoreOp(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageType.html>"]
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
    pub const TYPE_1D: Self = ImageType(0);
    pub const TYPE_2D: Self = ImageType(1);
    pub const TYPE_3D: Self = ImageType(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageTiling.html>"]
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
    pub const OPTIMAL: Self = ImageTiling(0);
    pub const LINEAR: Self = ImageTiling(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewType.html>"]
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
    pub const TYPE_1D: Self = ImageViewType(0);
    pub const TYPE_2D: Self = ImageViewType(1);
    pub const TYPE_3D: Self = ImageViewType(2);
    pub const CUBE: Self = ImageViewType(3);
    pub const TYPE_1D_ARRAY: Self = ImageViewType(4);
    pub const TYPE_2D_ARRAY: Self = ImageViewType(5);
    pub const CUBE_ARRAY: Self = ImageViewType(6);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferLevel.html>"]
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
    pub const PRIMARY: Self = CommandBufferLevel(0);
    pub const SECONDARY: Self = CommandBufferLevel(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentSwizzle.html>"]
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
    pub const IDENTITY: Self = ComponentSwizzle(0);
    pub const ZERO: Self = ComponentSwizzle(1);
    pub const ONE: Self = ComponentSwizzle(2);
    pub const R: Self = ComponentSwizzle(3);
    pub const G: Self = ComponentSwizzle(4);
    pub const B: Self = ComponentSwizzle(5);
    pub const A: Self = ComponentSwizzle(6);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorType.html>"]
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
    pub const SAMPLER: Self = DescriptorType(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = DescriptorType(1);
    pub const SAMPLED_IMAGE: Self = DescriptorType(2);
    pub const STORAGE_IMAGE: Self = DescriptorType(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = DescriptorType(4);
    pub const STORAGE_TEXEL_BUFFER: Self = DescriptorType(5);
    pub const UNIFORM_BUFFER: Self = DescriptorType(6);
    pub const STORAGE_BUFFER: Self = DescriptorType(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = DescriptorType(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = DescriptorType(9);
    pub const INPUT_ATTACHMENT: Self = DescriptorType(10);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryType.html>"]
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
    pub const OCCLUSION: Self = QueryType(0);
    #[doc = "Optional"]
    pub const PIPELINE_STATISTICS: Self = QueryType(1);
    pub const TIMESTAMP: Self = QueryType(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBorderColor.html>"]
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
    pub const FLOAT_TRANSPARENT_BLACK: Self = BorderColor(0);
    pub const INT_TRANSPARENT_BLACK: Self = BorderColor(1);
    pub const FLOAT_OPAQUE_BLACK: Self = BorderColor(2);
    pub const INT_OPAQUE_BLACK: Self = BorderColor(3);
    pub const FLOAT_OPAQUE_WHITE: Self = BorderColor(4);
    pub const INT_OPAQUE_WHITE: Self = BorderColor(5);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineBindPoint.html>"]
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
    pub const GRAPHICS: Self = PipelineBindPoint(0);
    pub const COMPUTE: Self = PipelineBindPoint(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheHeaderVersion.html>"]
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
    pub const ONE: Self = PipelineCacheHeaderVersion(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrimitiveTopology.html>"]
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
    pub const POINT_LIST: Self = PrimitiveTopology(0);
    pub const LINE_LIST: Self = PrimitiveTopology(1);
    pub const LINE_STRIP: Self = PrimitiveTopology(2);
    pub const TRIANGLE_LIST: Self = PrimitiveTopology(3);
    pub const TRIANGLE_STRIP: Self = PrimitiveTopology(4);
    pub const TRIANGLE_FAN: Self = PrimitiveTopology(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = PrimitiveTopology(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = PrimitiveTopology(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = PrimitiveTopology(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = PrimitiveTopology(9);
    pub const PATCH_LIST: Self = PrimitiveTopology(10);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharingMode.html>"]
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
    pub const EXCLUSIVE: Self = SharingMode(0);
    pub const CONCURRENT: Self = SharingMode(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndexType.html>"]
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
    pub const UINT16: Self = IndexType(0);
    pub const UINT32: Self = IndexType(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilter.html>"]
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
    pub const NEAREST: Self = Filter(0);
    pub const LINEAR: Self = Filter(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerMipmapMode.html>"]
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
    #[doc = "Choose nearest mip level"]
    pub const NEAREST: Self = SamplerMipmapMode(0);
    #[doc = "Linear filter between mip levels"]
    pub const LINEAR: Self = SamplerMipmapMode(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerAddressMode.html>"]
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
    pub const REPEAT: Self = SamplerAddressMode(0);
    pub const MIRRORED_REPEAT: Self = SamplerAddressMode(1);
    pub const CLAMP_TO_EDGE: Self = SamplerAddressMode(2);
    pub const CLAMP_TO_BORDER: Self = SamplerAddressMode(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompareOp.html>"]
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
    pub const NEVER: Self = CompareOp(0);
    pub const LESS: Self = CompareOp(1);
    pub const EQUAL: Self = CompareOp(2);
    pub const LESS_OR_EQUAL: Self = CompareOp(3);
    pub const GREATER: Self = CompareOp(4);
    pub const NOT_EQUAL: Self = CompareOp(5);
    pub const GREATER_OR_EQUAL: Self = CompareOp(6);
    pub const ALWAYS: Self = CompareOp(7);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPolygonMode.html>"]
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
    pub const FILL: Self = PolygonMode(0);
    pub const LINE: Self = PolygonMode(1);
    pub const POINT: Self = PolygonMode(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFrontFace.html>"]
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
    pub const COUNTER_CLOCKWISE: Self = FrontFace(0);
    pub const CLOCKWISE: Self = FrontFace(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendFactor.html>"]
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
    pub const ZERO: Self = BlendFactor(0);
    pub const ONE: Self = BlendFactor(1);
    pub const SRC_COLOR: Self = BlendFactor(2);
    pub const ONE_MINUS_SRC_COLOR: Self = BlendFactor(3);
    pub const DST_COLOR: Self = BlendFactor(4);
    pub const ONE_MINUS_DST_COLOR: Self = BlendFactor(5);
    pub const SRC_ALPHA: Self = BlendFactor(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = BlendFactor(7);
    pub const DST_ALPHA: Self = BlendFactor(8);
    pub const ONE_MINUS_DST_ALPHA: Self = BlendFactor(9);
    pub const CONSTANT_COLOR: Self = BlendFactor(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = BlendFactor(11);
    pub const CONSTANT_ALPHA: Self = BlendFactor(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = BlendFactor(13);
    pub const SRC_ALPHA_SATURATE: Self = BlendFactor(14);
    pub const SRC1_COLOR: Self = BlendFactor(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = BlendFactor(16);
    pub const SRC1_ALPHA: Self = BlendFactor(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = BlendFactor(18);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOp.html>"]
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
    pub const ADD: Self = BlendOp(0);
    pub const SUBTRACT: Self = BlendOp(1);
    pub const REVERSE_SUBTRACT: Self = BlendOp(2);
    pub const MIN: Self = BlendOp(3);
    pub const MAX: Self = BlendOp(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOp.html>"]
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
    pub const KEEP: Self = StencilOp(0);
    pub const ZERO: Self = StencilOp(1);
    pub const REPLACE: Self = StencilOp(2);
    pub const INCREMENT_AND_CLAMP: Self = StencilOp(3);
    pub const DECREMENT_AND_CLAMP: Self = StencilOp(4);
    pub const INVERT: Self = StencilOp(5);
    pub const INCREMENT_AND_WRAP: Self = StencilOp(6);
    pub const DECREMENT_AND_WRAP: Self = StencilOp(7);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLogicOp.html>"]
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
    pub const CLEAR: Self = LogicOp(0);
    pub const AND: Self = LogicOp(1);
    pub const AND_REVERSE: Self = LogicOp(2);
    pub const COPY: Self = LogicOp(3);
    pub const AND_INVERTED: Self = LogicOp(4);
    pub const NO_OP: Self = LogicOp(5);
    pub const XOR: Self = LogicOp(6);
    pub const OR: Self = LogicOp(7);
    pub const NOR: Self = LogicOp(8);
    pub const EQUIVALENT: Self = LogicOp(9);
    pub const INVERT: Self = LogicOp(10);
    pub const OR_REVERSE: Self = LogicOp(11);
    pub const COPY_INVERTED: Self = LogicOp(12);
    pub const OR_INVERTED: Self = LogicOp(13);
    pub const NAND: Self = LogicOp(14);
    pub const SET: Self = LogicOp(15);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInternalAllocationType.html>"]
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
    pub const EXECUTABLE: Self = InternalAllocationType(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSystemAllocationScope.html>"]
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
    pub const COMMAND: Self = SystemAllocationScope(0);
    pub const OBJECT: Self = SystemAllocationScope(1);
    pub const CACHE: Self = SystemAllocationScope(2);
    pub const DEVICE: Self = SystemAllocationScope(3);
    pub const INSTANCE: Self = SystemAllocationScope(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceType.html>"]
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
    pub const OTHER: Self = PhysicalDeviceType(0);
    pub const INTEGRATED_GPU: Self = PhysicalDeviceType(1);
    pub const DISCRETE_GPU: Self = PhysicalDeviceType(2);
    pub const VIRTUAL_GPU: Self = PhysicalDeviceType(3);
    pub const CPU: Self = PhysicalDeviceType(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputRate.html>"]
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
    pub const VERTEX: Self = VertexInputRate(0);
    pub const INSTANCE: Self = VertexInputRate(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormat.html>"]
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
    pub const UNDEFINED: Self = Format(0);
    pub const R4G4_UNORM_PACK8: Self = Format(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Format(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Format(3);
    pub const R5G6B5_UNORM_PACK16: Self = Format(4);
    pub const B5G6R5_UNORM_PACK16: Self = Format(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Format(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Format(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Format(8);
    pub const R8_UNORM: Self = Format(9);
    pub const R8_SNORM: Self = Format(10);
    pub const R8_USCALED: Self = Format(11);
    pub const R8_SSCALED: Self = Format(12);
    pub const R8_UINT: Self = Format(13);
    pub const R8_SINT: Self = Format(14);
    pub const R8_SRGB: Self = Format(15);
    pub const R8G8_UNORM: Self = Format(16);
    pub const R8G8_SNORM: Self = Format(17);
    pub const R8G8_USCALED: Self = Format(18);
    pub const R8G8_SSCALED: Self = Format(19);
    pub const R8G8_UINT: Self = Format(20);
    pub const R8G8_SINT: Self = Format(21);
    pub const R8G8_SRGB: Self = Format(22);
    pub const R8G8B8_UNORM: Self = Format(23);
    pub const R8G8B8_SNORM: Self = Format(24);
    pub const R8G8B8_USCALED: Self = Format(25);
    pub const R8G8B8_SSCALED: Self = Format(26);
    pub const R8G8B8_UINT: Self = Format(27);
    pub const R8G8B8_SINT: Self = Format(28);
    pub const R8G8B8_SRGB: Self = Format(29);
    pub const B8G8R8_UNORM: Self = Format(30);
    pub const B8G8R8_SNORM: Self = Format(31);
    pub const B8G8R8_USCALED: Self = Format(32);
    pub const B8G8R8_SSCALED: Self = Format(33);
    pub const B8G8R8_UINT: Self = Format(34);
    pub const B8G8R8_SINT: Self = Format(35);
    pub const B8G8R8_SRGB: Self = Format(36);
    pub const R8G8B8A8_UNORM: Self = Format(37);
    pub const R8G8B8A8_SNORM: Self = Format(38);
    pub const R8G8B8A8_USCALED: Self = Format(39);
    pub const R8G8B8A8_SSCALED: Self = Format(40);
    pub const R8G8B8A8_UINT: Self = Format(41);
    pub const R8G8B8A8_SINT: Self = Format(42);
    pub const R8G8B8A8_SRGB: Self = Format(43);
    pub const B8G8R8A8_UNORM: Self = Format(44);
    pub const B8G8R8A8_SNORM: Self = Format(45);
    pub const B8G8R8A8_USCALED: Self = Format(46);
    pub const B8G8R8A8_SSCALED: Self = Format(47);
    pub const B8G8R8A8_UINT: Self = Format(48);
    pub const B8G8R8A8_SINT: Self = Format(49);
    pub const B8G8R8A8_SRGB: Self = Format(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Format(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Format(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Format(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Format(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Format(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Format(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Format(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Format(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Format(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Format(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Format(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Format(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Format(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Format(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Format(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Format(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Format(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Format(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Format(69);
    pub const R16_UNORM: Self = Format(70);
    pub const R16_SNORM: Self = Format(71);
    pub const R16_USCALED: Self = Format(72);
    pub const R16_SSCALED: Self = Format(73);
    pub const R16_UINT: Self = Format(74);
    pub const R16_SINT: Self = Format(75);
    pub const R16_SFLOAT: Self = Format(76);
    pub const R16G16_UNORM: Self = Format(77);
    pub const R16G16_SNORM: Self = Format(78);
    pub const R16G16_USCALED: Self = Format(79);
    pub const R16G16_SSCALED: Self = Format(80);
    pub const R16G16_UINT: Self = Format(81);
    pub const R16G16_SINT: Self = Format(82);
    pub const R16G16_SFLOAT: Self = Format(83);
    pub const R16G16B16_UNORM: Self = Format(84);
    pub const R16G16B16_SNORM: Self = Format(85);
    pub const R16G16B16_USCALED: Self = Format(86);
    pub const R16G16B16_SSCALED: Self = Format(87);
    pub const R16G16B16_UINT: Self = Format(88);
    pub const R16G16B16_SINT: Self = Format(89);
    pub const R16G16B16_SFLOAT: Self = Format(90);
    pub const R16G16B16A16_UNORM: Self = Format(91);
    pub const R16G16B16A16_SNORM: Self = Format(92);
    pub const R16G16B16A16_USCALED: Self = Format(93);
    pub const R16G16B16A16_SSCALED: Self = Format(94);
    pub const R16G16B16A16_UINT: Self = Format(95);
    pub const R16G16B16A16_SINT: Self = Format(96);
    pub const R16G16B16A16_SFLOAT: Self = Format(97);
    pub const R32_UINT: Self = Format(98);
    pub const R32_SINT: Self = Format(99);
    pub const R32_SFLOAT: Self = Format(100);
    pub const R32G32_UINT: Self = Format(101);
    pub const R32G32_SINT: Self = Format(102);
    pub const R32G32_SFLOAT: Self = Format(103);
    pub const R32G32B32_UINT: Self = Format(104);
    pub const R32G32B32_SINT: Self = Format(105);
    pub const R32G32B32_SFLOAT: Self = Format(106);
    pub const R32G32B32A32_UINT: Self = Format(107);
    pub const R32G32B32A32_SINT: Self = Format(108);
    pub const R32G32B32A32_SFLOAT: Self = Format(109);
    pub const R64_UINT: Self = Format(110);
    pub const R64_SINT: Self = Format(111);
    pub const R64_SFLOAT: Self = Format(112);
    pub const R64G64_UINT: Self = Format(113);
    pub const R64G64_SINT: Self = Format(114);
    pub const R64G64_SFLOAT: Self = Format(115);
    pub const R64G64B64_UINT: Self = Format(116);
    pub const R64G64B64_SINT: Self = Format(117);
    pub const R64G64B64_SFLOAT: Self = Format(118);
    pub const R64G64B64A64_UINT: Self = Format(119);
    pub const R64G64B64A64_SINT: Self = Format(120);
    pub const R64G64B64A64_SFLOAT: Self = Format(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Format(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Format(123);
    pub const D16_UNORM: Self = Format(124);
    pub const X8_D24_UNORM_PACK32: Self = Format(125);
    pub const D32_SFLOAT: Self = Format(126);
    pub const S8_UINT: Self = Format(127);
    pub const D16_UNORM_S8_UINT: Self = Format(128);
    pub const D24_UNORM_S8_UINT: Self = Format(129);
    pub const D32_SFLOAT_S8_UINT: Self = Format(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Format(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Format(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Format(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Format(134);
    pub const BC2_UNORM_BLOCK: Self = Format(135);
    pub const BC2_SRGB_BLOCK: Self = Format(136);
    pub const BC3_UNORM_BLOCK: Self = Format(137);
    pub const BC3_SRGB_BLOCK: Self = Format(138);
    pub const BC4_UNORM_BLOCK: Self = Format(139);
    pub const BC4_SNORM_BLOCK: Self = Format(140);
    pub const BC5_UNORM_BLOCK: Self = Format(141);
    pub const BC5_SNORM_BLOCK: Self = Format(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Format(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Format(144);
    pub const BC7_UNORM_BLOCK: Self = Format(145);
    pub const BC7_SRGB_BLOCK: Self = Format(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Format(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Format(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Format(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Format(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Format(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Format(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Format(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Format(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Format(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Format(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Format(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Format(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Format(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Format(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Format(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Format(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Format(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Format(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Format(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Format(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Format(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Format(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Format(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Format(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Format(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Format(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Format(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Format(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Format(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Format(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Format(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Format(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Format(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Format(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Format(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Format(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Format(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Format(184);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStructureType.html>"]
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
    pub const APPLICATION_INFO: Self = StructureType(0);
    pub const INSTANCE_CREATE_INFO: Self = StructureType(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = StructureType(2);
    pub const DEVICE_CREATE_INFO: Self = StructureType(3);
    pub const SUBMIT_INFO: Self = StructureType(4);
    pub const MEMORY_ALLOCATE_INFO: Self = StructureType(5);
    pub const MAPPED_MEMORY_RANGE: Self = StructureType(6);
    pub const BIND_SPARSE_INFO: Self = StructureType(7);
    pub const FENCE_CREATE_INFO: Self = StructureType(8);
    pub const SEMAPHORE_CREATE_INFO: Self = StructureType(9);
    pub const EVENT_CREATE_INFO: Self = StructureType(10);
    pub const QUERY_POOL_CREATE_INFO: Self = StructureType(11);
    pub const BUFFER_CREATE_INFO: Self = StructureType(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = StructureType(13);
    pub const IMAGE_CREATE_INFO: Self = StructureType(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = StructureType(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = StructureType(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = StructureType(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = StructureType(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = StructureType(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = StructureType(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = StructureType(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = StructureType(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = StructureType(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = StructureType(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = StructureType(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = StructureType(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = StructureType(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = StructureType(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = StructureType(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = StructureType(30);
    pub const SAMPLER_CREATE_INFO: Self = StructureType(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = StructureType(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = StructureType(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = StructureType(34);
    pub const WRITE_DESCRIPTOR_SET: Self = StructureType(35);
    pub const COPY_DESCRIPTOR_SET: Self = StructureType(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = StructureType(37);
    pub const RENDER_PASS_CREATE_INFO: Self = StructureType(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = StructureType(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = StructureType(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = StructureType(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = StructureType(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = StructureType(43);
    pub const BUFFER_MEMORY_BARRIER: Self = StructureType(44);
    pub const IMAGE_MEMORY_BARRIER: Self = StructureType(45);
    pub const MEMORY_BARRIER: Self = StructureType(46);
    #[doc = "Reserved for internal use by the loader, layers, and ICDs"]
    pub const LOADER_INSTANCE_CREATE_INFO: Self = StructureType(47);
    #[doc = "Reserved for internal use by the loader, layers, and ICDs"]
    pub const LOADER_DEVICE_CREATE_INFO: Self = StructureType(48);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassContents.html>"]
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
    pub const INLINE: Self = SubpassContents(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = SubpassContents(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResult.html>"]
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
    #[doc = "Command completed successfully"]
    pub const SUCCESS: Self = Result(0);
    #[doc = "A fence or query has not yet completed"]
    pub const NOT_READY: Self = Result(1);
    #[doc = "A wait operation has not completed in the specified time"]
    pub const TIMEOUT: Self = Result(2);
    #[doc = "An event is signaled"]
    pub const EVENT_SET: Self = Result(3);
    #[doc = "An event is unsignaled"]
    pub const EVENT_RESET: Self = Result(4);
    #[doc = "A return array was too small for the result"]
    pub const INCOMPLETE: Self = Result(5);
    #[doc = "A host memory allocation has failed"]
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Result(-1);
    #[doc = "A device memory allocation has failed"]
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Result(-2);
    #[doc = "Initialization of a object has failed"]
    pub const ERROR_INITIALIZATION_FAILED: Self = Result(-3);
    #[doc = "The logical device has been lost. See <<devsandqueues-lost-device>>"]
    pub const ERROR_DEVICE_LOST: Self = Result(-4);
    #[doc = "Mapping of a memory object has failed"]
    pub const ERROR_MEMORY_MAP_FAILED: Self = Result(-5);
    #[doc = "Layer specified does not exist"]
    pub const ERROR_LAYER_NOT_PRESENT: Self = Result(-6);
    #[doc = "Extension specified does not exist"]
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Result(-7);
    #[doc = "Requested feature is not available on this device"]
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Result(-8);
    #[doc = "Unable to find a Vulkan driver"]
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Result(-9);
    #[doc = "Too many objects of the type have already been created"]
    pub const ERROR_TOO_MANY_OBJECTS: Self = Result(-10);
    #[doc = "Requested format is not supported on this device"]
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Result(-11);
    #[doc = "A requested pool allocation has failed due to fragmentation of the pool\'s memory"]
    pub const ERROR_FRAGMENTED_POOL: Self = Result(-12);
    #[doc = "An unknown error has occurred, due to an implementation or application bug"]
    pub const ERROR_UNKNOWN: Self = Result(-13);
}
impl ::std::error::Error for Result {
    fn description(&self) -> &str {
        let name = match *self {
            Result::SUCCESS => Some("Command completed successfully"),
            Result::NOT_READY => Some("A fence or query has not yet completed"),
            Result::TIMEOUT => Some("A wait operation has not completed in the specified time"),
            Result::EVENT_SET => Some("An event is signaled"),
            Result::EVENT_RESET => Some("An event is unsignaled"),
            Result::INCOMPLETE => Some("A return array was too small for the result"),
            Result::ERROR_OUT_OF_HOST_MEMORY => Some("A host memory allocation has failed"),
            Result::ERROR_OUT_OF_DEVICE_MEMORY => Some("A device memory allocation has failed"),
            Result::ERROR_INITIALIZATION_FAILED => Some("Initialization of a object has failed"),
            Result::ERROR_DEVICE_LOST => {
                Some("The logical device has been lost. See <<devsandqueues-lost-device>>")
            }
            Result::ERROR_MEMORY_MAP_FAILED => Some("Mapping of a memory object has failed"),
            Result::ERROR_LAYER_NOT_PRESENT => Some("Layer specified does not exist"),
            Result::ERROR_EXTENSION_NOT_PRESENT => Some("Extension specified does not exist"),
            Result::ERROR_FEATURE_NOT_PRESENT => {
                Some("Requested feature is not available on this device")
            }
            Result::ERROR_INCOMPATIBLE_DRIVER => Some("Unable to find a Vulkan driver"),
            Result::ERROR_TOO_MANY_OBJECTS => {
                Some("Too many objects of the type have already been created")
            }
            Result::ERROR_FORMAT_NOT_SUPPORTED => {
                Some("Requested format is not supported on this device")
            }
            Result::ERROR_FRAGMENTED_POOL => Some(
                "A requested pool allocation has failed due to fragmentation of the pool\'s memory",
            ),
            Result::ERROR_UNKNOWN => {
                Some("An unknown error has occurred, due to an implementation or application bug")
            }
            _ => None,
        };
        name.unwrap_or("unknown error")
    }
}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Result::SUCCESS => Some("Command completed successfully"),
            Result::NOT_READY => Some("A fence or query has not yet completed"),
            Result::TIMEOUT => Some("A wait operation has not completed in the specified time"),
            Result::EVENT_SET => Some("An event is signaled"),
            Result::EVENT_RESET => Some("An event is unsignaled"),
            Result::INCOMPLETE => Some("A return array was too small for the result"),
            Result::ERROR_OUT_OF_HOST_MEMORY => Some("A host memory allocation has failed"),
            Result::ERROR_OUT_OF_DEVICE_MEMORY => Some("A device memory allocation has failed"),
            Result::ERROR_INITIALIZATION_FAILED => Some("Initialization of a object has failed"),
            Result::ERROR_DEVICE_LOST => {
                Some("The logical device has been lost. See <<devsandqueues-lost-device>>")
            }
            Result::ERROR_MEMORY_MAP_FAILED => Some("Mapping of a memory object has failed"),
            Result::ERROR_LAYER_NOT_PRESENT => Some("Layer specified does not exist"),
            Result::ERROR_EXTENSION_NOT_PRESENT => Some("Extension specified does not exist"),
            Result::ERROR_FEATURE_NOT_PRESENT => {
                Some("Requested feature is not available on this device")
            }
            Result::ERROR_INCOMPATIBLE_DRIVER => Some("Unable to find a Vulkan driver"),
            Result::ERROR_TOO_MANY_OBJECTS => {
                Some("Too many objects of the type have already been created")
            }
            Result::ERROR_FORMAT_NOT_SUPPORTED => {
                Some("Requested format is not supported on this device")
            }
            Result::ERROR_FRAGMENTED_POOL => Some(
                "A requested pool allocation has failed due to fragmentation of the pool\'s memory",
            ),
            Result::ERROR_UNKNOWN => {
                Some("An unknown error has occurred, due to an implementation or application bug")
            }
            _ => None,
        };
        if let Some(x) = name {
            fmt.write_str(x)
        } else {
            self.0.fmt(fmt)
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDynamicState.html>"]
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
    pub const VIEWPORT: Self = DynamicState(0);
    pub const SCISSOR: Self = DynamicState(1);
    pub const LINE_WIDTH: Self = DynamicState(2);
    pub const DEPTH_BIAS: Self = DynamicState(3);
    pub const BLEND_CONSTANTS: Self = DynamicState(4);
    pub const DEPTH_BOUNDS: Self = DynamicState(5);
    pub const STENCIL_COMPARE_MASK: Self = DynamicState(6);
    pub const STENCIL_WRITE_MASK: Self = DynamicState(7);
    pub const STENCIL_REFERENCE: Self = DynamicState(8);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateType.html>"]
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
    #[doc = "Create descriptor update template for descriptor set updates"]
    pub const DESCRIPTOR_SET: Self = DescriptorUpdateTemplateType(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkObjectType.html>"]
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
    pub const UNKNOWN: Self = ObjectType(0);
    #[doc = "VkInstance"]
    pub const INSTANCE: Self = ObjectType(1);
    #[doc = "VkPhysicalDevice"]
    pub const PHYSICAL_DEVICE: Self = ObjectType(2);
    #[doc = "VkDevice"]
    pub const DEVICE: Self = ObjectType(3);
    #[doc = "VkQueue"]
    pub const QUEUE: Self = ObjectType(4);
    #[doc = "VkSemaphore"]
    pub const SEMAPHORE: Self = ObjectType(5);
    #[doc = "VkCommandBuffer"]
    pub const COMMAND_BUFFER: Self = ObjectType(6);
    #[doc = "VkFence"]
    pub const FENCE: Self = ObjectType(7);
    #[doc = "VkDeviceMemory"]
    pub const DEVICE_MEMORY: Self = ObjectType(8);
    #[doc = "VkBuffer"]
    pub const BUFFER: Self = ObjectType(9);
    #[doc = "VkImage"]
    pub const IMAGE: Self = ObjectType(10);
    #[doc = "VkEvent"]
    pub const EVENT: Self = ObjectType(11);
    #[doc = "VkQueryPool"]
    pub const QUERY_POOL: Self = ObjectType(12);
    #[doc = "VkBufferView"]
    pub const BUFFER_VIEW: Self = ObjectType(13);
    #[doc = "VkImageView"]
    pub const IMAGE_VIEW: Self = ObjectType(14);
    #[doc = "VkShaderModule"]
    pub const SHADER_MODULE: Self = ObjectType(15);
    #[doc = "VkPipelineCache"]
    pub const PIPELINE_CACHE: Self = ObjectType(16);
    #[doc = "VkPipelineLayout"]
    pub const PIPELINE_LAYOUT: Self = ObjectType(17);
    #[doc = "VkRenderPass"]
    pub const RENDER_PASS: Self = ObjectType(18);
    #[doc = "VkPipeline"]
    pub const PIPELINE: Self = ObjectType(19);
    #[doc = "VkDescriptorSetLayout"]
    pub const DESCRIPTOR_SET_LAYOUT: Self = ObjectType(20);
    #[doc = "VkSampler"]
    pub const SAMPLER: Self = ObjectType(21);
    #[doc = "VkDescriptorPool"]
    pub const DESCRIPTOR_POOL: Self = ObjectType(22);
    #[doc = "VkDescriptorSet"]
    pub const DESCRIPTOR_SET: Self = ObjectType(23);
    #[doc = "VkFramebuffer"]
    pub const FRAMEBUFFER: Self = ObjectType(24);
    #[doc = "VkCommandPool"]
    pub const COMMAND_POOL: Self = ObjectType(25);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreType.html>"]
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
    pub const BINARY: Self = SemaphoreType(0);
    pub const TIMELINE: Self = SemaphoreType(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPresentModeKHR.html>"]
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
    pub const IMMEDIATE: Self = PresentModeKHR(0);
    pub const MAILBOX: Self = PresentModeKHR(1);
    pub const FIFO: Self = PresentModeKHR(2);
    pub const FIFO_RELAXED: Self = PresentModeKHR(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorSpaceKHR.html>"]
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
    pub const SRGB_NONLINEAR: Self = ColorSpaceKHR(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTimeDomainEXT.html>"]
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
    pub const DEVICE: Self = TimeDomainEXT(0);
    pub const CLOCK_MONOTONIC: Self = TimeDomainEXT(1);
    pub const CLOCK_MONOTONIC_RAW: Self = TimeDomainEXT(2);
    pub const QUERY_PERFORMANCE_COUNTER: Self = TimeDomainEXT(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportObjectTypeEXT.html>"]
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
    pub const UNKNOWN: Self = DebugReportObjectTypeEXT(0);
    pub const INSTANCE: Self = DebugReportObjectTypeEXT(1);
    pub const PHYSICAL_DEVICE: Self = DebugReportObjectTypeEXT(2);
    pub const DEVICE: Self = DebugReportObjectTypeEXT(3);
    pub const QUEUE: Self = DebugReportObjectTypeEXT(4);
    pub const SEMAPHORE: Self = DebugReportObjectTypeEXT(5);
    pub const COMMAND_BUFFER: Self = DebugReportObjectTypeEXT(6);
    pub const FENCE: Self = DebugReportObjectTypeEXT(7);
    pub const DEVICE_MEMORY: Self = DebugReportObjectTypeEXT(8);
    pub const BUFFER: Self = DebugReportObjectTypeEXT(9);
    pub const IMAGE: Self = DebugReportObjectTypeEXT(10);
    pub const EVENT: Self = DebugReportObjectTypeEXT(11);
    pub const QUERY_POOL: Self = DebugReportObjectTypeEXT(12);
    pub const BUFFER_VIEW: Self = DebugReportObjectTypeEXT(13);
    pub const IMAGE_VIEW: Self = DebugReportObjectTypeEXT(14);
    pub const SHADER_MODULE: Self = DebugReportObjectTypeEXT(15);
    pub const PIPELINE_CACHE: Self = DebugReportObjectTypeEXT(16);
    pub const PIPELINE_LAYOUT: Self = DebugReportObjectTypeEXT(17);
    pub const RENDER_PASS: Self = DebugReportObjectTypeEXT(18);
    pub const PIPELINE: Self = DebugReportObjectTypeEXT(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = DebugReportObjectTypeEXT(20);
    pub const SAMPLER: Self = DebugReportObjectTypeEXT(21);
    pub const DESCRIPTOR_POOL: Self = DebugReportObjectTypeEXT(22);
    pub const DESCRIPTOR_SET: Self = DebugReportObjectTypeEXT(23);
    pub const FRAMEBUFFER: Self = DebugReportObjectTypeEXT(24);
    pub const COMMAND_POOL: Self = DebugReportObjectTypeEXT(25);
    pub const SURFACE_KHR: Self = DebugReportObjectTypeEXT(26);
    pub const SWAPCHAIN_KHR: Self = DebugReportObjectTypeEXT(27);
    pub const DEBUG_REPORT_CALLBACK: Self = DebugReportObjectTypeEXT(28);
    pub const DISPLAY_KHR: Self = DebugReportObjectTypeEXT(29);
    pub const DISPLAY_MODE_KHR: Self = DebugReportObjectTypeEXT(30);
    pub const VALIDATION_CACHE: Self = DebugReportObjectTypeEXT(33);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRasterizationOrderAMD.html>"]
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
    pub const STRICT: Self = RasterizationOrderAMD(0);
    pub const RELAXED: Self = RasterizationOrderAMD(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCheckEXT.html>"]
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
    pub const ALL: Self = ValidationCheckEXT(0);
    pub const SHADERS: Self = ValidationCheckEXT(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeatureEnableEXT.html>"]
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
    pub const GPU_ASSISTED: Self = ValidationFeatureEnableEXT(0);
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = ValidationFeatureEnableEXT(1);
    pub const BEST_PRACTICES: Self = ValidationFeatureEnableEXT(2);
    pub const DEBUG_PRINTF: Self = ValidationFeatureEnableEXT(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeatureDisableEXT.html>"]
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
    pub const ALL: Self = ValidationFeatureDisableEXT(0);
    pub const SHADERS: Self = ValidationFeatureDisableEXT(1);
    pub const THREAD_SAFETY: Self = ValidationFeatureDisableEXT(2);
    pub const API_PARAMETERS: Self = ValidationFeatureDisableEXT(3);
    pub const OBJECT_LIFETIMES: Self = ValidationFeatureDisableEXT(4);
    pub const CORE_CHECKS: Self = ValidationFeatureDisableEXT(5);
    pub const UNIQUE_HANDLES: Self = ValidationFeatureDisableEXT(6);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsTokenTypeNV.html>"]
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
    pub const SHADER_GROUP: Self = IndirectCommandsTokenTypeNV(0);
    pub const STATE_FLAGS: Self = IndirectCommandsTokenTypeNV(1);
    pub const INDEX_BUFFER: Self = IndirectCommandsTokenTypeNV(2);
    pub const VERTEX_BUFFER: Self = IndirectCommandsTokenTypeNV(3);
    pub const PUSH_CONSTANT: Self = IndirectCommandsTokenTypeNV(4);
    pub const DRAW_INDEXED: Self = IndirectCommandsTokenTypeNV(5);
    pub const DRAW: Self = IndirectCommandsTokenTypeNV(6);
    pub const DRAW_TASKS: Self = IndirectCommandsTokenTypeNV(7);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayPowerStateEXT.html>"]
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
    pub const OFF: Self = DisplayPowerStateEXT(0);
    pub const SUSPEND: Self = DisplayPowerStateEXT(1);
    pub const ON: Self = DisplayPowerStateEXT(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceEventTypeEXT.html>"]
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
    pub const DISPLAY_HOTPLUG: Self = DeviceEventTypeEXT(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayEventTypeEXT.html>"]
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
    pub const FIRST_PIXEL_OUT: Self = DisplayEventTypeEXT(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewportCoordinateSwizzleNV.html>"]
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
    pub const POSITIVE_X: Self = ViewportCoordinateSwizzleNV(0);
    pub const NEGATIVE_X: Self = ViewportCoordinateSwizzleNV(1);
    pub const POSITIVE_Y: Self = ViewportCoordinateSwizzleNV(2);
    pub const NEGATIVE_Y: Self = ViewportCoordinateSwizzleNV(3);
    pub const POSITIVE_Z: Self = ViewportCoordinateSwizzleNV(4);
    pub const NEGATIVE_Z: Self = ViewportCoordinateSwizzleNV(5);
    pub const POSITIVE_W: Self = ViewportCoordinateSwizzleNV(6);
    pub const NEGATIVE_W: Self = ViewportCoordinateSwizzleNV(7);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDiscardRectangleModeEXT.html>"]
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
    pub const INCLUSIVE: Self = DiscardRectangleModeEXT(0);
    pub const EXCLUSIVE: Self = DiscardRectangleModeEXT(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPointClippingBehavior.html>"]
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
    pub const ALL_CLIP_PLANES: Self = PointClippingBehavior(0);
    pub const USER_CLIP_PLANES_ONLY: Self = PointClippingBehavior(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerReductionMode.html>"]
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
    pub const WEIGHTED_AVERAGE: Self = SamplerReductionMode(0);
    pub const MIN: Self = SamplerReductionMode(1);
    pub const MAX: Self = SamplerReductionMode(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTessellationDomainOrigin.html>"]
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
    pub const UPPER_LEFT: Self = TessellationDomainOrigin(0);
    pub const LOWER_LEFT: Self = TessellationDomainOrigin(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrModelConversion.html>"]
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
    pub const RGB_IDENTITY: Self = SamplerYcbcrModelConversion(0);
    #[doc = "just range expansion"]
    pub const YCBCR_IDENTITY: Self = SamplerYcbcrModelConversion(1);
    #[doc = "aka HD YUV"]
    pub const YCBCR_709: Self = SamplerYcbcrModelConversion(2);
    #[doc = "aka SD YUV"]
    pub const YCBCR_601: Self = SamplerYcbcrModelConversion(3);
    #[doc = "aka UHD YUV"]
    pub const YCBCR_2020: Self = SamplerYcbcrModelConversion(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrRange.html>"]
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
    #[doc = "Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)"]
    pub const ITU_FULL: Self = SamplerYcbcrRange(0);
    #[doc = "Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240"]
    pub const ITU_NARROW: Self = SamplerYcbcrRange(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkChromaLocation.html>"]
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
    pub const COSITED_EVEN: Self = ChromaLocation(0);
    pub const MIDPOINT: Self = ChromaLocation(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOverlapEXT.html>"]
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
    pub const UNCORRELATED: Self = BlendOverlapEXT(0);
    pub const DISJOINT: Self = BlendOverlapEXT(1);
    pub const CONJOINT: Self = BlendOverlapEXT(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageModulationModeNV.html>"]
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
    pub const NONE: Self = CoverageModulationModeNV(0);
    pub const RGB: Self = CoverageModulationModeNV(1);
    pub const ALPHA: Self = CoverageModulationModeNV(2);
    pub const RGBA: Self = CoverageModulationModeNV(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoverageReductionModeNV.html>"]
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
    pub const MERGE: Self = CoverageReductionModeNV(0);
    pub const TRUNCATE: Self = CoverageReductionModeNV(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheHeaderVersionEXT.html>"]
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
    pub const ONE: Self = ValidationCacheHeaderVersionEXT(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderInfoTypeAMD.html>"]
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
    pub const STATISTICS: Self = ShaderInfoTypeAMD(0);
    pub const BINARY: Self = ShaderInfoTypeAMD(1);
    pub const DISASSEMBLY: Self = ShaderInfoTypeAMD(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueGlobalPriorityEXT.html>"]
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
    pub const LOW: Self = QueueGlobalPriorityEXT(128);
    pub const MEDIUM: Self = QueueGlobalPriorityEXT(256);
    pub const HIGH: Self = QueueGlobalPriorityEXT(512);
    pub const REALTIME: Self = QueueGlobalPriorityEXT(1_024);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConservativeRasterizationModeEXT.html>"]
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
    pub const DISABLED: Self = ConservativeRasterizationModeEXT(0);
    pub const OVERESTIMATE: Self = ConservativeRasterizationModeEXT(1);
    pub const UNDERESTIMATE: Self = ConservativeRasterizationModeEXT(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVendorId.html>"]
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
    #[doc = "Vivante vendor ID"]
    pub const VIV: Self = VendorId(0x1_0001);
    #[doc = "VeriSilicon vendor ID"]
    pub const VSI: Self = VendorId(0x1_0002);
    #[doc = "Kazan Software Renderer"]
    pub const KAZAN: Self = VendorId(0x1_0003);
    #[doc = "Codeplay Software Ltd. vendor ID"]
    pub const CODEPLAY: Self = VendorId(0x1_0004);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDriverId.html>"]
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
    #[doc = "Advanced Micro Devices, Inc."]
    pub const AMD_PROPRIETARY: Self = DriverId(1);
    #[doc = "Advanced Micro Devices, Inc."]
    pub const AMD_OPEN_SOURCE: Self = DriverId(2);
    #[doc = "Mesa open source project"]
    pub const MESA_RADV: Self = DriverId(3);
    #[doc = "NVIDIA Corporation"]
    pub const NVIDIA_PROPRIETARY: Self = DriverId(4);
    #[doc = "Intel Corporation"]
    pub const INTEL_PROPRIETARY_WINDOWS: Self = DriverId(5);
    #[doc = "Intel Corporation"]
    pub const INTEL_OPEN_SOURCE_MESA: Self = DriverId(6);
    #[doc = "Imagination Technologies"]
    pub const IMAGINATION_PROPRIETARY: Self = DriverId(7);
    #[doc = "Qualcomm Technologies, Inc."]
    pub const QUALCOMM_PROPRIETARY: Self = DriverId(8);
    #[doc = "Arm Limited"]
    pub const ARM_PROPRIETARY: Self = DriverId(9);
    #[doc = "Google LLC"]
    pub const GOOGLE_SWIFTSHADER: Self = DriverId(10);
    #[doc = "Google LLC"]
    pub const GGP_PROPRIETARY: Self = DriverId(11);
    #[doc = "Broadcom Inc."]
    pub const BROADCOM_PROPRIETARY: Self = DriverId(12);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShadingRatePaletteEntryNV.html>"]
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
    pub const NO_INVOCATIONS: Self = ShadingRatePaletteEntryNV(0);
    pub const TYPE_16_INVOCATIONS_PER_PIXEL: Self = ShadingRatePaletteEntryNV(1);
    pub const TYPE_8_INVOCATIONS_PER_PIXEL: Self = ShadingRatePaletteEntryNV(2);
    pub const TYPE_4_INVOCATIONS_PER_PIXEL: Self = ShadingRatePaletteEntryNV(3);
    pub const TYPE_2_INVOCATIONS_PER_PIXEL: Self = ShadingRatePaletteEntryNV(4);
    pub const TYPE_1_INVOCATION_PER_PIXEL: Self = ShadingRatePaletteEntryNV(5);
    pub const TYPE_1_INVOCATION_PER_2X1_PIXELS: Self = ShadingRatePaletteEntryNV(6);
    pub const TYPE_1_INVOCATION_PER_1X2_PIXELS: Self = ShadingRatePaletteEntryNV(7);
    pub const TYPE_1_INVOCATION_PER_2X2_PIXELS: Self = ShadingRatePaletteEntryNV(8);
    pub const TYPE_1_INVOCATION_PER_4X2_PIXELS: Self = ShadingRatePaletteEntryNV(9);
    pub const TYPE_1_INVOCATION_PER_2X4_PIXELS: Self = ShadingRatePaletteEntryNV(10);
    pub const TYPE_1_INVOCATION_PER_4X4_PIXELS: Self = ShadingRatePaletteEntryNV(11);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCoarseSampleOrderTypeNV.html>"]
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
    pub const DEFAULT: Self = CoarseSampleOrderTypeNV(0);
    pub const CUSTOM: Self = CoarseSampleOrderTypeNV(1);
    pub const PIXEL_MAJOR: Self = CoarseSampleOrderTypeNV(2);
    pub const SAMPLE_MAJOR: Self = CoarseSampleOrderTypeNV(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeKHR.html>"]
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
    pub const CLONE: Self = CopyAccelerationStructureModeKHR(0);
    pub const COMPACT: Self = CopyAccelerationStructureModeKHR(1);
    pub const SERIALIZE: Self = CopyAccelerationStructureModeKHR(2);
    pub const DESERIALIZE: Self = CopyAccelerationStructureModeKHR(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html>"]
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
    pub const TOP_LEVEL: Self = AccelerationStructureTypeKHR(0);
    pub const BOTTOM_LEVEL: Self = AccelerationStructureTypeKHR(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeKHR.html>"]
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
    pub const TRIANGLES: Self = GeometryTypeKHR(0);
    pub const AABBS: Self = GeometryTypeKHR(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeKHR.html>"]
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
    pub const OBJECT: Self = AccelerationStructureMemoryRequirementsTypeKHR(0);
    pub const BUILD_SCRATCH: Self = AccelerationStructureMemoryRequirementsTypeKHR(1);
    pub const UPDATE_SCRATCH: Self = AccelerationStructureMemoryRequirementsTypeKHR(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html>"]
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
    pub const HOST: Self = AccelerationStructureBuildTypeKHR(0);
    pub const DEVICE: Self = AccelerationStructureBuildTypeKHR(1);
    pub const HOST_OR_DEVICE: Self = AccelerationStructureBuildTypeKHR(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html>"]
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
    pub const GENERAL: Self = RayTracingShaderGroupTypeKHR(0);
    pub const TRIANGLES_HIT_GROUP: Self = RayTracingShaderGroupTypeKHR(1);
    pub const PROCEDURAL_HIT_GROUP: Self = RayTracingShaderGroupTypeKHR(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html>"]
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
    pub const DEFAULT: Self = MemoryOverallocationBehaviorAMD(0);
    pub const ALLOWED: Self = MemoryOverallocationBehaviorAMD(1);
    pub const DISALLOWED: Self = MemoryOverallocationBehaviorAMD(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkScopeNV.html>"]
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
    pub const DEVICE: Self = ScopeNV(1);
    pub const WORKGROUP: Self = ScopeNV(2);
    pub const SUBGROUP: Self = ScopeNV(3);
    pub const QUEUE_FAMILY: Self = ScopeNV(5);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentTypeNV.html>"]
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
    pub const FLOAT16: Self = ComponentTypeNV(0);
    pub const FLOAT32: Self = ComponentTypeNV(1);
    pub const FLOAT64: Self = ComponentTypeNV(2);
    pub const SINT8: Self = ComponentTypeNV(3);
    pub const SINT16: Self = ComponentTypeNV(4);
    pub const SINT32: Self = ComponentTypeNV(5);
    pub const SINT64: Self = ComponentTypeNV(6);
    pub const UINT8: Self = ComponentTypeNV(7);
    pub const UINT16: Self = ComponentTypeNV(8);
    pub const UINT32: Self = ComponentTypeNV(9);
    pub const UINT64: Self = ComponentTypeNV(10);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFullScreenExclusiveEXT.html>"]
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
    pub const DEFAULT: Self = FullScreenExclusiveEXT(0);
    pub const ALLOWED: Self = FullScreenExclusiveEXT(1);
    pub const DISALLOWED: Self = FullScreenExclusiveEXT(2);
    pub const APPLICATION_CONTROLLED: Self = FullScreenExclusiveEXT(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterScopeKHR.html>"]
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
    pub const COMMAND_BUFFER: Self = PerformanceCounterScopeKHR(0);
    pub const RENDER_PASS: Self = PerformanceCounterScopeKHR(1);
    pub const COMMAND: Self = PerformanceCounterScopeKHR(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterUnitKHR.html>"]
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
    pub const GENERIC: Self = PerformanceCounterUnitKHR(0);
    pub const PERCENTAGE: Self = PerformanceCounterUnitKHR(1);
    pub const NANOSECONDS: Self = PerformanceCounterUnitKHR(2);
    pub const BYTES: Self = PerformanceCounterUnitKHR(3);
    pub const BYTES_PER_SECOND: Self = PerformanceCounterUnitKHR(4);
    pub const KELVIN: Self = PerformanceCounterUnitKHR(5);
    pub const WATTS: Self = PerformanceCounterUnitKHR(6);
    pub const VOLTS: Self = PerformanceCounterUnitKHR(7);
    pub const AMPS: Self = PerformanceCounterUnitKHR(8);
    pub const HERTZ: Self = PerformanceCounterUnitKHR(9);
    pub const CYCLES: Self = PerformanceCounterUnitKHR(10);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceCounterStorageKHR.html>"]
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
    pub const INT32: Self = PerformanceCounterStorageKHR(0);
    pub const INT64: Self = PerformanceCounterStorageKHR(1);
    pub const UINT32: Self = PerformanceCounterStorageKHR(2);
    pub const UINT64: Self = PerformanceCounterStorageKHR(3);
    pub const FLOAT32: Self = PerformanceCounterStorageKHR(4);
    pub const FLOAT64: Self = PerformanceCounterStorageKHR(5);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html>"]
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
    pub const PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self =
        PerformanceConfigurationTypeINTEL(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolSamplingModeINTEL.html>"]
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
    pub const QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: Self = QueryPoolSamplingModeINTEL(0);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceOverrideTypeINTEL.html>"]
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
    pub const PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: Self = PerformanceOverrideTypeINTEL(0);
    pub const PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: Self =
        PerformanceOverrideTypeINTEL(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceParameterTypeINTEL.html>"]
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
    pub const PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: Self =
        PerformanceParameterTypeINTEL(0);
    pub const PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALIDS_INTEL: Self =
        PerformanceParameterTypeINTEL(1);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceValueTypeINTEL.html>"]
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
    pub const PERFORMANCE_VALUE_TYPE_UINT32_INTEL: Self = PerformanceValueTypeINTEL(0);
    pub const PERFORMANCE_VALUE_TYPE_UINT64_INTEL: Self = PerformanceValueTypeINTEL(1);
    pub const PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: Self = PerformanceValueTypeINTEL(2);
    pub const PERFORMANCE_VALUE_TYPE_BOOL_INTEL: Self = PerformanceValueTypeINTEL(3);
    pub const PERFORMANCE_VALUE_TYPE_STRING_INTEL: Self = PerformanceValueTypeINTEL(4);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderFloatControlsIndependence.html>"]
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
    pub const TYPE_32_ONLY: Self = ShaderFloatControlsIndependence(0);
    pub const ALL: Self = ShaderFloatControlsIndependence(1);
    pub const NONE: Self = ShaderFloatControlsIndependence(2);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html>"]
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
    pub const BOOL32: Self = PipelineExecutableStatisticFormatKHR(0);
    pub const INT64: Self = PipelineExecutableStatisticFormatKHR(1);
    pub const UINT64: Self = PipelineExecutableStatisticFormatKHR(2);
    pub const FLOAT64: Self = PipelineExecutableStatisticFormatKHR(3);
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLineRasterizationModeEXT.html>"]
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
    pub const DEFAULT: Self = LineRasterizationModeEXT(0);
    pub const RECTANGULAR: Self = LineRasterizationModeEXT(1);
    pub const BRESENHAM: Self = LineRasterizationModeEXT(2);
    pub const RECTANGULAR_SMOOTH: Self = LineRasterizationModeEXT(3);
}
