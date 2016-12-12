#Ash

A very lightweight wrapper around Vulkan

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Documentation](https://docs.rs/ash/badge.svg)](https://docs.rs/ash)
[![Build Status](https://travis-ci.org/MaikKlein/ash.svg?branch=master)](https://travis-ci.org/MaikKlein/ash)
[![Join the chat at https://gitter.im/MaikKlein/ash](https://badges.gitter.im/MaikKlein/ash.svg)](https://gitter.im/MaikKlein/ash?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Crates.io Version](https://img.shields.io/crates/v/ash.svg)](https://crates.io/crates/ash)

## Stable yet?
No.

## Why Ash?

Functions return a `type VkResult<T> = Result<T, vk::Result>` instead of an error code. No mutable references for the output are required.
```Rust
    pub fn create_swapchain_khr(&self,
                                create_info: &vk::SwapchainCreateInfoKHR)
                                -> VkResult<vk::SwapchainKHR>;
    let swapchain = device.create_swapchain_khr(&swapchain_create_info).unwrap();
```


Always returns a `Vec<T>` for functions that output multiple values.
```Rust
    pub fn get_swapchain_images_khr(&self,
                                    swapchain: vk::SwapchainKHR)
                                    -> VkResult<Vec<vk::Image>>;
    let present_images = device.get_swapchain_images_khr(swapchain).unwrap();
```
Ash always uses slices in functions.
```Rust
    // C
    void vkCmdPipelineBarrier(
        VkCommandBuffer                             commandBuffer,
        VkPipelineStageFlags                        srcStageMask,
        VkPipelineStageFlags                        dstStageMask,
        VkDependencyFlags                           dependencyFlags,
        uint32_t                                    memoryBarrierCount,
        const VkMemoryBarrier*                      pMemoryBarriers,
        uint32_t                                    bufferMemoryBarrierCount,
        const VkBufferMemoryBarrier*                pBufferMemoryBarriers,
        uint32_t                                    imageMemoryBarrierCount,
        const VkImageMemoryBarrier*                 pImageMemoryBarriers);

    // Rust
    pub fn cmd_pipeline_barrier(&self,
                                command_buffer: vk::CommandBuffer,
                                src_stage_mask: vk::PipelineStageFlags,
                                dst_stage_mask: vk::PipelineStageFlags,
                                dependency_flags: vk::DependencyFlags,
                                memory_barriers: &[vk::MemoryBarrier],
                                buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                                image_memory_barriers: &[vk::ImageMemoryBarrier]);

    device.cmd_pipeline_barrier(setup_command_buffer,
                                vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                vk::DependencyFlags::empty(),
                                &[],
                                &[],
                                &[layout_transition_barrier]);

    // or

    let slice = device.map_memory::<Vertex>(vertex_input_buffer_memory,
                              0,
                              vertex_input_buffer_info.size,
                              vk::MemoryMapFlags::empty())
        .unwrap();
    slice.copy_from_slice(&vertices);
```
Ash still uses raw Vulkan structs. The only difference is type safety. Everything that can be an enum is an enum like `vk::StructureType`, flags are implemented similar to the `Bitflags` crate. Ash also follows the Rust style guide. The reason that Ash uses raw Vulkan structs is to be extensible, just like the Vulkan spec.
```Rust
    let pool_create_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::CommandPoolCreateInfo,
        p_next: ptr::null(),
        flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        queue_family_index: queue_family_index,
    };
    let pool = device.create_command_pool(&pool_create_info).unwrap();
```
Ash also takes care of loading the function pointers. Function pointers are split into 3 categories. Entry, Instance and Device. The reason for not loading it into a global is that in Vulkan you can have multiple devices and each device must load its own function pointers.
```Rust
    // Looks for the vulkan lib in your path, alternatively you can supply the path explicitly.
    let entry = Entry::load_vulkan().unwrap();
    let instance: Instance = entry.create_instance(&create_info).expect("Instance creation error");
    let device: Device = instance.create_device(pdevice, &device_create_info)
        .unwrap();
```
You don't have to pass an Instance or Device handle anymore, this is done implicitly for you.
```Rust
    // C
    VkResult vkCreateCommandPool(
        VkDevice                                    device,
        const VkCommandPoolCreateInfo*              pCreateInfo,
        const VkAllocationCallbacks*                pAllocator,
        VkCommandPool*                              pCommandPool);

    // Rust
    pub fn create_command_pool(&self,
                               create_info: &vk::CommandPoolCreateInfo)
                               -> VkResult<vk::CommandPool>;

    let pool = device.create_command_pool(&pool_create_info).unwrap();
```
## Example
You can find the examples [here](https://github.com/MaikKlein/ash/tree/master/examples).
### [Triangle](https://github.com/MaikKlein/ash/blob/master/examples/triangle/src/main.rs)
Currently only runs under Linux (x11) and requires GLFW, the LunarG Validation layers, a Vulkan library. Ports for other operating systems are in progress. (Currently the GLFW wrapper only wraps the low level x11 bindings)

The triangle example is written from top to bottom without many helper functions or external dependencies. It renders a colored triangle. The shaders a written in GLSL and compiled into SPIR-V with [glslang](https://github.com/KhronosGroup/glslang)

```
cd examples
cargo run
```

![screenshot](http://i.imgur.com/PQZcL6w.jpg)

### [Texture](https://github.com/MaikKlein/ash/blob/master/examples/texture/src/main.rs)
Display a texture on a quad. *Needs a cleanup*.
![texture](http://i.imgur.com/trow00H.png)

## Open questions

### Unsafe?
Currently ash can be used without any unsafe keyword. I have looked at a few other c wrappers and it seems this is common practice. But Ash is not particular safe and I am thinking of marking every function `unsafe`.

### Optional extension loading
Currently extensions are loaded like normal vulkan functions. This means some extenstions can be loaded, for example the win32 surface on linux. Accessing a unloaded function currently triggers a `debug_assert`. I am thinking of seperating extensions into their own struct.

```Rust
impl Device{
    pub fn load_swapchain(&self) -> Result<SwapchainExtension, Err>;
}
// Instead of
//let swapchain = device.create_swapchain_khr(&swapchain_create_info).unwrap();

let swapchain_ext = device.load_swapchain().unwrap();
let swapchain = swapchain_ext.create_swapchain_khr(&swapchain_create_info).unwrap();
```


## Roadmap

### Complete

### In progress

- Wrapping the complete spec
- Optional extension loading

### Not started
- Custom allocators

## A thanks to

* [Api with no secrets](https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-1)
* [Vulkan tutorial](http://av.dfki.de/~jhenriques/development.html)
* [Vulkan examples](https://github.com/SaschaWillems/Vulkan)
* [Vulkan tutorial](https://vulkan-tutorial.com/)
* [Vulkano](https://github.com/tomaka/vulkano/)
* [vk-rs](https://github.com/Osspial/vk-rs)
