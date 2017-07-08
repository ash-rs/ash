use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::size_of;
use vk;

/// `Align` handles dynamic alignment. x86 aligns on 4 byte boundries but GPUs
/// sometimes have different requirements. For example if the user wants to create
/// a Uniform buffer of a Vec3<f32>, `get_buffer_memory_requirements` might return
/// an alignment of 16 bytes. A Vec3<f32> has a size of 12 bytes. The memory layout
/// for a `Vec<Vec3<f32>>` will be contigous, because 4 is a multiple of 12. But Vulkan
/// expects a 4 byte padding in between each Vec3<f32>, if the alignment is 16 bytes.
/// `Vec3<f32>, 4bytes, Vec3<f32>, 4bytes, Vec3<f32>...`. Align is able to take a slice
/// that is allocated on 4 bytes boundries, and insert the correct amount of paddings.
#[derive(Debug, Clone)]
pub struct Align<T> {
    ptr: *mut vk::c_void,
    elem_size: vk::DeviceSize,
    size: vk::DeviceSize,
    _m: PhantomData<T>,
}

#[derive(Debug)]
pub struct AlignIter<'a, T: 'a> {
    align: &'a mut Align<T>,
    current: vk::DeviceSize,
}

impl<T: Copy> Align<T> {
    pub fn copy_from_slice(&mut self, slice: &[T]) {
        use std::slice::from_raw_parts_mut;
        if self.elem_size == size_of::<T>() as u64 {
            unsafe {
                let mapped_slice = from_raw_parts_mut(self.ptr as *mut T, slice.len());
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
    pub unsafe fn new(
        ptr: *mut vk::c_void,
        alignment: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> Self {
        let padding = calc_padding(size_of::<T>() as vk::DeviceSize, alignment);
        let elem_size = size_of::<T>() as vk::DeviceSize + padding;
        assert!(calc_padding(size, alignment) == 0, "size must be aligned");
        Align {
            ptr,
            elem_size,
            size,
            _m: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> AlignIter<T> {
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
            let ptr = (self.align.ptr as *mut u8).offset(self.current as isize) as *mut T;
            self.current += self.align.elem_size;
            Some(&mut *ptr)
        }
    }
}
