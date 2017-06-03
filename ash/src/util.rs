use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::size_of;

/// AlignByteSlice is the dynamic alternative to `Align`. Sometimes the user wants to
/// align slices at runtime. One example would be to align different images in one buffer.
/// There is usually no indicates of how big an image is at compile time and `AlignByteSlice`
/// will align those image byte slices at runtime.

#[derive(Debug, Clone)]
pub struct AlignByteSlice {
    ptr: *mut (),
    alignment: usize,
    size: usize,
}

impl AlignByteSlice {
    pub fn copy_from_slices(&mut self, slices: &[&[u8]]) {
        self.ptr as *mut u8;
        let mut current = 0;
        for slice in slices{
            unsafe {
                let ptr = (self.ptr as *mut u8).offset(current);
                let raw_slice = ::std::slice::from_raw_parts_mut(ptr, slice.len());
                raw_slice.copy_from_slice(slice);
                current += slice.len() as isize;
                let padding = current % self.alignment as isize;
                current += padding;
            }
        }
    }
}

impl AlignByteSlice {
    pub fn new(ptr: *mut (), alignment: usize, size: usize) -> Self {
        AlignByteSlice {
            ptr,
            size,
            alignment
        }
    }
}

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
    ptr: *mut (),
    offset: usize,
    size: usize,
    _m: PhantomData<T>,
}

#[derive(Debug)]
pub struct AlignIter<'a, T: 'a> {
    align: &'a mut Align<T>,
    current: usize,
}

impl<T: Copy> Align<T> {
    pub fn copy_from_slice(&mut self, slice: &[T]) {
        for (i, val) in self.iter_mut().enumerate() {
            *val = slice[i];
        }
    }
}

impl<T> Align<T> {
    pub fn new(ptr: *mut (), alignment: usize, size: usize) -> Self {
        let offset = size_of::<T>() + size_of::<T>() % alignment;
        Align {
            ptr,
            offset,
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
            self.current += self.align.offset;
            Some(&mut *ptr)
        }
    }
}
