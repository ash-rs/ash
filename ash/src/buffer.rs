use vk_loader as vk;
use glfw::*;
use std::mem;
use instance::Instance;
use std::ptr;
use std::os::raw::c_void;

pub fn find_memorytype_index(memory_req: &vk::MemoryRequirements,
                             memory_prop: &vk::PhysicalDeviceMemoryProperties,
                             flags: vk::MemoryPropertyFlags)
                             -> Option<u32> {
    let mut memory_type_bits = memory_req.memoryTypeBits;
    for (index, ref memory_type) in memory_prop.memoryTypes.iter().enumerate() {
        if (memory_type.propertyFlags & flags) == flags {
            return Some(index as u32);
        }
        memory_type_bits = memory_type_bits >> 1;
    }
    None
}

struct HostBuffer {
    memory_type: vk::MemoryType,
}
