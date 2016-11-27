use device;
use vk_loader as vk;
use std::mem;
use std::ptr;
use std::convert::From;

pub enum CommandPoolFlags {
    Transient,
    Reset,
}

enum CommandBufferLevel {
    Primary,
    Secondary,
}

pub struct CommandPool {
    pub device: device::Device,
    pub pool: vk::CommandPool,
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            self.device
                .inner
                .dp
                .DestroyCommandPool(self.device.inner.device, self.pool, ptr::null());
        }
    }
}

pub struct CommandBuffer<'r> {
    pub pool: &'r CommandPool,
    pub buffer: vk::CommandBuffer,
}

impl<'r> CommandBuffer<'r> {
    pub fn begin(&self, cu: CommandBufferUsage) {
        let create_info = vk::CommandBufferBeginInfo {
            sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            flags: cu.bits(),
            pInheritanceInfo: ptr::null(),
        };
        unsafe {
            self.pool.device.inner.dp.BeginCommandBuffer(self.buffer, &create_info);
        }
    }

    pub fn end(&self){
        unsafe{
            self.pool.device.inner.dp.EndCommandBuffer(self.buffer);
        }
    }
}

impl CommandPool {
    pub fn allocate_commandbuffer(&self) -> CommandBuffer {
        let create_info = vk::CommandBufferAllocateInfo {
            sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            commandPool: self.pool,
            level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
            pNext: ptr::null(),
        };

        unsafe {
            let mut command_buffer = mem::uninitialized();
            self.device.inner.dp.AllocateCommandBuffers(self.device.inner.device,
                                                        &create_info,
                                                        &mut command_buffer);
            let cb = CommandBuffer {
                pool: self,
                buffer: command_buffer,
            };

            cb
        }
    }
}

bitflags! {
    pub flags CommandBufferUsage: u32{
        const ONE_TIME_SUBMIT      = vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        const RENDER_PASS_CONTINUE = vk::COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT,
        const SIMULTANEOUS_USE     = vk::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT
    }
}
