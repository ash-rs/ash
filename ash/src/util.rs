use crate::{vk, vk::TaggedStructure};
use core::ffi::c_void;
use core::iter::Iterator;
use core::marker::PhantomData;
use core::mem::size_of; // TODO: Remove when bumping MSRV to 1.80
use core::slice;
#[cfg(feature = "std")]
use std::io;

/// [`Align`] handles dynamic alignment. The is useful for dynamic uniform buffers where
/// the alignment might be different. For example a 4x4 f32 matrix has a size of 64 bytes
/// but the min alignment for a dynamic uniform buffer might be 256 bytes. A slice of `&[Mat4x4<f32>]`
/// has a memory layout of `[[64 bytes], [64 bytes], [64 bytes]]`, but it might need to have a memory
/// layout of `[[256 bytes], [256 bytes], [256 bytes]]`.
/// [`Align::copy_from_slice`] will copy a slice of `&[T]` directly into the host memory without
/// an additional allocation and with the correct alignment.
#[derive(Debug, Clone)]
pub struct Align<T> {
    ptr: *mut c_void,
    elem_size: vk::DeviceSize,
    size: vk::DeviceSize,
    _m: PhantomData<T>,
}

#[derive(Debug)]
pub struct AlignIter<'a, T> {
    align: &'a mut Align<T>,
    current: vk::DeviceSize,
}

impl<T: Copy> Align<T> {
    pub fn copy_from_slice(&mut self, slice: &[T]) {
        if self.elem_size == size_of::<T>() as u64 {
            unsafe {
                let mapped_slice = slice::from_raw_parts_mut(self.ptr.cast(), slice.len());
                mapped_slice.copy_from_slice(slice);
            }
        } else {
            for (i, val) in self.iter_mut().enumerate().take(slice.len()) {
                *val = slice[i];
            }
        }
    }
}

fn calc_padding(adr: vk::DeviceSize, align: vk::DeviceSize) -> vk::DeviceSize {
    (align - adr % align) % align
}

impl<T> Align<T> {
    pub unsafe fn new(ptr: *mut c_void, alignment: vk::DeviceSize, size: vk::DeviceSize) -> Self {
        let padding = calc_padding(size_of::<T>() as vk::DeviceSize, alignment);
        let elem_size = size_of::<T>() as vk::DeviceSize + padding;
        assert!(calc_padding(size, alignment) == 0, "size must be aligned");
        Self {
            ptr,
            elem_size,
            size,
            _m: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> AlignIter<'_, T> {
        AlignIter {
            current: 0,
            align: self,
        }
    }
}

impl<'a, T: Copy + 'a> Iterator for AlignIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.align.size {
            return None;
        }
        unsafe {
            // Need to cast to *mut u8 because () has size 0
            let ptr = (self.align.ptr.cast::<u8>())
                .offset(self.current as isize)
                .cast();
            self.current += self.align.elem_size;
            Some(&mut *ptr)
        }
    }
}

/// Decode SPIR-V from bytes.
///
/// This function handles SPIR-V of arbitrary endianness gracefully, and returns correctly aligned
/// storage.
///
/// # Examples
/// ```no_run
/// // Decode SPIR-V from a file
/// let mut file = std::fs::File::open("/path/to/shader.spv").unwrap();
/// let words = ash::util::read_spv(&mut file).unwrap();
/// ```
/// ```
/// // Decode SPIR-V from memory
/// const SPIRV: &[u8] = &[
///     // ...
/// #   0x03, 0x02, 0x23, 0x07,
/// ];
/// let words = ash::util::read_spv(&mut std::io::Cursor::new(&SPIRV[..])).unwrap();
/// ```
#[cfg(feature = "std")]
pub fn read_spv<R: io::Read + io::Seek>(x: &mut R) -> io::Result<Vec<u32>> {
    // TODO use stream_len() once it is stabilized and remove the subsequent rewind() call
    let size = x.seek(io::SeekFrom::End(0))?;
    x.rewind()?;
    if size % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input length not divisible by 4",
        ));
    }
    if size > usize::MAX as u64 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too long"));
    }
    let words = (size / 4) as usize;
    // https://github.com/ash-rs/ash/issues/354:
    // Zero-initialize the result to prevent read_exact from possibly
    // reading uninitialized memory.
    let mut result = vec![0u32; words];
    x.read_exact(unsafe {
        slice::from_raw_parts_mut(result.as_mut_ptr().cast::<u8>(), words * 4)
    })?;
    const MAGIC_NUMBER: u32 = 0x0723_0203;
    if !result.is_empty() && result[0] == MAGIC_NUMBER.swap_bytes() {
        for word in &mut result {
            *word = word.swap_bytes();
        }
    }
    if result.is_empty() || result[0] != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input missing SPIR-V magic number",
        ));
    }
    Ok(result)
}

pub trait NextChainExt<'a>: TaggedStructure<'a> {
    /// Prepends the given extension struct between the root and the first pointer. This
    /// method only exists on structs that can be passed to a function directly. Only
    /// valid extension structs can be pushed into the chain.
    /// If the chain looks like `A -> B -> C`, and you call `x.push_next(&mut D)`, then the
    /// chain will look like `A -> D -> B -> C`.
    ///
    /// For inline construction of extension structs using the builder pattern, use `with_next` instead.
    fn push_next<T: vk::Extends<'a, Self>>(&mut self, next: &'a mut T) {
        let next: &mut vk::BaseOutStructure<'a> = next.as_base_mut();
        assert!(next.p_next.is_null());
        let base: &mut vk::BaseOutStructure<'a> = self.as_base_mut();
        next.p_next = base.p_next;
        base.p_next = next;
    }

    /// Builder method to prepends the given extension struct between the root and the first pointer.
    /// This  method only exists on structs that can be passed to a function directly. Only
    /// valid extension structs can be pushed into the chain.
    ///
    /// ```rust
    /// use ash::prelude::*;
    /// use ash::vk;
    /// let mut a = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
    /// let mut b = vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
    /// let mut c = vk::PhysicalDeviceMultiDrawFeaturesEXT::default();
    /// let base = vk::PhysicalDeviceFeatures2::default()
    ///    .with_next(&mut a)
    ///     .with_next(&mut b)
    ///     .with_next(&mut c);
    /// let mut iter = base.iter_next_chain();
    /// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT); // c.s_type
    /// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR); // b.s_type
    /// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR); // a.s_type
    /// ```
    /// For inline construction of extension structs, use `with_next` instead.
    fn with_next<T: vk::Extends<'a, Self>>(mut self, next: &'a mut T) -> Self
    where
        Self: Sized,
    {
        self.push_next(next);
        self
    }
    /// Returns a mutable iterator over the entire extension chain attached to `Self`
    fn iter_next_chain_mut(&'a mut self) -> impl Iterator<Item = &'a mut TaggedObject<'a>> + 'a {
        (0..).scan(self.as_base_mut().p_next, |p_ptr, _| unsafe {
            if p_ptr.is_null() {
                return None;
            }
            let n_ptr = (**p_ptr).p_next;
            let old = *p_ptr;
            *p_ptr = n_ptr;
            Some(TaggedObject::from_raw_mut(old))
        })
    }
    /// Returns an iterator over the entire extension chain attached to `Self`
    fn iter_next_chain(&'a self) -> impl Iterator<Item = &'a TaggedObject<'a>> + 'a {
        (0..).scan(self.as_base().p_next, |p_ptr, _| unsafe {
            if p_ptr.is_null() {
                return None;
            }
            let n_ptr = (**p_ptr).p_next;
            let old = *p_ptr;
            *p_ptr = n_ptr;
            Some(TaggedObject::from_raw(old))
        })
    }
    /// Extend the next chain of the current object with multiple tagged objects
    /// ```rust
    /// use ash::prelude::*;
    /// use ash::vk;
    /// use ash::util::TaggedObject;
    /// let mut a = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
    /// let mut b = vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
    /// let mut c = vk::PhysicalDeviceMultiDrawFeaturesEXT::default();
    /// let mut base = vk::PhysicalDeviceFeatures2::default();
    /// base.extend([
    ///     TaggedObject::from_mut(&mut a),
    ///     TaggedObject::from_mut(&mut b),
    ///     TaggedObject::from_mut(&mut c)
    /// ]);
    /// let mut iter = base.iter_next_chain();
    /// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT); // c.s_type
    /// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR); // b.s_type
    /// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR); // a.s_type
    /// ```
    fn extend(&mut self, nexts: impl IntoIterator<Item = &'a mut TaggedObject<'a>>) {
        for next in nexts.into_iter() {
            assert!(next.as_base_mut().p_next.is_null());
            // Safety: self implements TaggedStructure
            let base: &mut vk::BaseOutStructure<'a> = self.as_base_mut();
            next.output.p_next = base.p_next;
            base.p_next = <*mut TaggedObject<'a>>::cast(next);
        }
    }
}

/// Blanket implementation of next chain utility methods on all base types
impl<'a, T> NextChainExt<'a> for T where T: vk::BaseTaggedStructure<'a> {}

/// Type-erased object representing a tagged Vulkan structure.
/// It is basically a [`Box<dyn Any>`], but for types implementing [`TaggedStructure`].
#[repr(C)]
pub union TaggedObject<'a> {
    output: vk::BaseOutStructure<'a>,
    input: vk::BaseInStructure<'a>,
}
impl vk::StructureType {
    const ASH_DYNAMIC: Self = Self(-1);
}

/// [`TaggedObject`]s are layout-compatible with [`vk::BaseInStructure`] and
/// [`vk::BaseOutStructure`].
unsafe impl<'a> TaggedStructure<'a> for TaggedObject<'a> {
    /// Querying the tag of a [`TaggedObject`] statically using [`TaggedStructure::STRUCTURE_TYPE`]
    /// returns [`vk::StructureType::ASH_DYNAMIC`], since the actual tag is dynamic.
    /// To query the tag of a [`TaggedObject`] dynamically, use [`TaggedObject::tag`].
    const STRUCTURE_TYPE: vk::StructureType = vk::StructureType::ASH_DYNAMIC;
}

impl<'a> NextChainExt<'a> for TaggedObject<'a> {}

/// [`TaggedObject`]s can be extended with ANY tagged objects. It is up to the user to ensure that they're
/// calling it correctly.
/// ```rust
/// use ash::prelude::*;
/// use ash::vk;
/// use ash::util::TaggedObject;
/// let mut a = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
/// let mut b = vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
/// let mut c = vk::PhysicalDeviceMultiDrawFeaturesEXT::default();
/// let mut d = vk::ApplicationInfo::default();
/// let mut base = vk::PhysicalDeviceFeatures2::default();
/// let base = TaggedObject::from_mut(&mut base);
/// base.push_next(&mut a);
/// base.push_next(&mut b);
/// base.push_next(&mut c);
/// base.push_next(&mut d);
///
///
/// let mut iter = base.iter_next_chain();
/// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::APPLICATION_INFO); // d.s_type
/// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT); // c.s_type
/// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR); // b.s_type
/// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR); // a.s_type
/// ```
unsafe impl<'a, T: TaggedStructure<'a> + ?Sized> vk::Extends<'a, TaggedObject<'a>> for T {}
/// [`TaggedObject`]s can extend ANY other tagged objects.
/// ```rust
/// use ash::prelude::*;
/// use ash::vk;
/// use ash::util::TaggedObject;
/// let mut a = vk::ApplicationInfo::default();
/// let mut a = TaggedObject::from_mut(&mut a);
/// let mut base = vk::PhysicalDeviceFeatures2::default();
/// base.push_next(a);
///
/// let mut iter = base.iter_next_chain();
/// assert_eq!(iter.next().unwrap().tag(), vk::StructureType::APPLICATION_INFO); // a.s_type
/// ```
unsafe impl<'a, T: vk::BaseTaggedStructure<'a> + ?Sized> vk::Extends<'a, T> for TaggedObject<'a> {}

impl<'a> TaggedObject<'a> {
    pub unsafe fn from_raw(obj: *const vk::BaseInStructure<'a>) -> &'a Self {
        &*(obj as *const Self)
    }

    pub unsafe fn from_raw_mut(obj: *mut vk::BaseOutStructure<'a>) -> &'a mut Self {
        &mut *(obj as *mut Self)
    }

    pub fn from_ref<T: TaggedStructure<'a> + ?Sized>(obj: &T) -> &Self {
        unsafe { &*(<*const T>::cast(obj)) }
    }

    pub fn from_mut<T: TaggedStructure<'a> + ?Sized>(obj: &mut T) -> &mut Self {
        unsafe { &mut *(<*mut T>::cast(obj)) }
    }
    pub fn tag(&self) -> vk::StructureType {
        self.as_base().s_type
    }
    pub fn downcast_ref<T: TaggedStructure<'a>>(&self) -> Option<&T> {
        unsafe {
            if self.tag() == T::STRUCTURE_TYPE {
                Some(&*<*const vk::BaseInStructure<'_>>::cast(&self.input))
            } else {
                None
            }
        }
    }
    pub fn downcast_mut<T: TaggedStructure<'a>>(&mut self) -> Option<&mut T> {
        unsafe {
            if self.tag() == T::STRUCTURE_TYPE {
                Some(&mut *<*mut vk::BaseOutStructure<'_>>::cast(
                    &mut self.output,
                ))
            } else {
                None
            }
        }
    }
    pub fn is<T: TaggedStructure<'a>>(&self) -> bool {
        self.tag() == T::STRUCTURE_TYPE
    }
}
