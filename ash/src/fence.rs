use vk_loader as vk;

use std::mem;
use device::Device;
use std::ops::Drop;
use std::ptr;

pub struct Fence {
    pub device: Device,
    pub handle: vk::Fence,
}
impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            self.device.dp().DestroyFence(self.device.handle(), self.handle, ptr::null());
        }
    }
}

impl Fence {}
