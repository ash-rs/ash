#Ash

A very lightweight wrapper around Vulkan

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Documentation](https://docs.rs/ash/badge.svg)](https://docs.rs/ash)
[![Build Status](https://travis-ci.org/MaikKlein/ash.svg?branch=master)](https://travis-ci.org/MaikKlein/ash)
[![Join the chat at https://gitter.im/MaikKlein/ash](https://badges.gitter.im/MaikKlein/ash.svg)](https://gitter.im/MaikKlein/ash?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Crates.io Version](https://img.shields.io/crates/v/ash.svg)](https://crates.io/crates/ash)
[![](https://tokei.rs/b1/github/maikklein/ash)](https://github.com/MaikKlein/ash)

## Stable yet?
No.

## Why Ash?
- [x] Lightweight Vulkan wrapper
- [x] Low overhead
- [x] Added type safety
- [x] More convenient to use than raw Vulkan
- [x] Includes a loader
- [x] Extensions have their own loader

## What does it do?
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
Additionally, every Vulkan extension has to be loaded explicity. You can find all extensions under [ash::extensions](https://github.com/MaikKlein/ash/tree/master/src/extensions). You still have to tell Vulkan which instance or device extensions you want to load.
```Rust
use ash::extensions::Swapchain;
let swapchain_loader = Swapchain::new(&instance, &device).expect("Unable to load swapchain");
let swapchain = swapchain_loader.create_swapchain_khr(&swapchain_create_info).unwrap();
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

The triangle example is written from top to bottom without many helper functions or external dependencies. It renders a colored triangle. The shaders are written in GLSL and compiled into SPIR-V with [glslang](https://github.com/KhronosGroup/glslang)

```
cd examples/triangle
cargo run
```

![screenshot](http://i.imgur.com/PQZcL6w.jpg)

### [Texture](https://github.com/MaikKlein/ash/blob/master/examples/texture/src/main.rs)
Displays a texture on a quad. *Needs a cleanup*.

```
cd examples/texture
cargo run
```
![texture](http://i.imgur.com/trow00H.png)

## Roadmap

### Extensions
- [x] Swapchain
- [x] Surface
- [x] XlibSurface
- [x] DebugReport
- [ ] Win32Surface
- [ ] MirSurface
- [ ] XcbSurface
- [ ] AndroidSurface
- [ ] WaylandSurface
- [ ] Display

### In progress
- Wrapping the complete spec

### Not started
- [Custom allocators](https://github.com/MaikKlein/ash/issues/2.static_fn)

## A thanks to

* [Api with no secrets](https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-1)
* [Vulkan tutorial](http://av.dfki.de/~jhenriques/development.html)
* [Vulkan examples](https://github.com/SaschaWillems/Vulkan)
* [Vulkan tutorial](https://vulkan-tutorial.com/)
* [Vulkano](https://github.com/tomaka/vulkano/)
* [vk-rs](https://github.com/Osspial/vk-rs)
