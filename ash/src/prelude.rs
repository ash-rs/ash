use alloc::vec::Vec;
use core::convert::TryInto;
use core::ffi;
use core::mem;
use core::ptr;

use crate::vk;
pub type VkResult<T> = Result<T, vk::Result>;

impl vk::Result {
    #[inline]
    pub fn result(self) -> VkResult<()> {
        self.result_with_success(())
    }

    #[inline]
    pub fn result_with_success<T>(self, v: T) -> VkResult<T> {
        match self {
            Self::SUCCESS => Ok(v),
            _ => Err(self),
        }
    }

    #[inline]
    pub unsafe fn assume_init_on_success<T>(self, v: mem::MaybeUninit<T>) -> VkResult<T> {
        self.result().map(move |()| v.assume_init())
    }

    #[inline]
    pub unsafe fn set_vec_len_on_success<T>(self, mut v: Vec<T>, len: usize) -> VkResult<Vec<T>> {
        self.result().map(move |()| {
            v.set_len(len);
            v
        })
    }
}

/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore, ensuring all
/// available data has been read into the vector.
///
/// See for example [`vkEnumerateInstanceExtensionProperties()`]: the number of available items may
/// change between calls; [`vk::Result::INCOMPLETE`] is returned when the count increased (and the
/// vector is not large enough after querying the initial size), requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties()`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
    mut f: impl FnMut(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: core::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

/// Calls `f` twice until it does not return [`vk::Result::ERROR_NOT_ENOUGH_SPACE_KHR`], ensuring all
/// available binary data has been read into the vector.
///
/// The first call happens with a [`Vec`] of size `4096`.  If this is not adequate, `f` is supposed
/// to return [`vk::Result::ERROR_NOT_ENOUGH_SPACE_KHR`] while also updating `count` to the desired
/// number of elements, allowing us to try again.
///
/// This function is _not_ designed to be used with [`vk::Result::INCOMPLETE`], see
/// [`read_into_uninitialized_vector()`] instead.
///
/// See for example [`vkGetPipelineBinaryDataKHR()`], where the new return code was first introduced.
///
/// [`vkGetPipelineBinaryDataKHR()`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineBinaryDataKHR.html
pub(crate) unsafe fn read_into_uninitialized_binary_vector(
    mut f: impl FnMut(&mut usize, *mut ffi::c_void) -> vk::Result,
) -> VkResult<Vec<u8>> {
    let mut count = 4096;
    let mut data = Vec::<u8>::with_capacity(count);
    let mut err_code = f(&mut count, data.as_mut_ptr().cast());
    if err_code == vk::Result::ERROR_NOT_ENOUGH_SPACE_KHR {
        debug_assert!(
            count > 4096,
            "Implementation should have updated the value to be higher than the initial request"
        );
        err_code = f(&mut count, data.as_mut_ptr().cast());
        debug_assert_ne!(
            err_code,
            vk::Result::ERROR_NOT_ENOUGH_SPACE_KHR,
            "Updated count was still not adequate"
        );
    }
    err_code.set_vec_len_on_success(data, count)
}

#[cfg(feature = "debug")]
pub(crate) fn debug_flags<Value: Into<u64> + Copy>(
    f: &mut core::fmt::Formatter<'_>,
    known: &[(Value, &'static str)],
    value: Value,
) -> core::fmt::Result {
    let mut first = true;
    let mut accum = value.into();
    for &(bit, name) in known {
        let bit = bit.into();
        if bit != 0 && accum & bit == bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            accum &= !bit;
        }
    }
    if accum != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{accum:b}")?;
    }
    Ok(())
}
