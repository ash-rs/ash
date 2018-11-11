# Ash

A very lightweight wrapper around Vulkan

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Documentation](https://docs.rs/ash/badge.svg)](https://docs.rs/ash)
[![Build Status](https://travis-ci.org/MaikKlein/ash.svg?branch=master)](https://travis-ci.org/MaikKlein/ash)
[![Build status](https://ci.appveyor.com/api/projects/status/ed7757as3a4ebexn/branch/master?svg=true)](https://ci.appveyor.com/project/MaikKlein/ash/branch/master)
[![Join the chat at https://gitter.im/MaikKlein/ash](https://badges.gitter.im/MaikKlein/ash.svg)](https://gitter.im/MaikKlein/ash?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Crates.io Version](https://img.shields.io/crates/v/ash.svg)](https://crates.io/crates/ash)
[![](https://tokei.rs/b1/github/maikklein/ash)](https://github.com/MaikKlein/ash)

## Overview
- [x] A true Vulkan API without compromises
- [x] Convenience features that don't limit the functionality
- [x] Function pointer loading
- [x] No validation, everything is **unsafe**
- [x] Generated from `vk.xml`

## Features
### Explicit returns with `Result`
```Rust
// function signature
pub fn create_instance(&self,
                       create_info: &vk::InstanceCreateInfo,
                       allocation_callbacks: Option<&vk::AllocationCallbacks>)
                       -> Result<Instance, InstanceError> { .. }
let instance = entry.create_instance(&create_info, None)
    .expect("Instance creation error");
```


### Returns a `Vec<T>` (*when possible*) for functions that output multiple values.

```Rust
pub fn get_swapchain_images_khr(&self,
                                swapchain: vk::SwapchainKHR)
                                -> VkResult<Vec<vk::Image>>;
let present_images = swapchain_loader.get_swapchain_images_khr(swapchain).unwrap();
```

### Slices in functions
```Rust
pub fn cmd_pipeline_barrier(&self,
                            command_buffer: vk::CommandBuffer,
                            src_stage_mask: vk::PipelineStageFlags,
                            dst_stage_mask: vk::PipelineStageFlags,
                            dependency_flags: vk::DependencyFlags,
                            memory_barriers: &[vk::MemoryBarrier],
                            buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                            image_memory_barriers: &[vk::ImageMemoryBarrier]);
```

### Default implementation for all types
```Rust
// No need to manually set the structure type
let desc_alloc_info = vk::DescriptorSetAllocateInfo {
    descriptor_pool: self.pool,
    descriptor_set_count: self.layouts.len() as u32,
    p_set_layouts: self.layouts.as_ptr(),
    ..Default::default()
};
```
### Builder pattern
```Rust
let pipeline_vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::builder()
    .vertex_binding_descriptions(&Vertex::binding_descriptions())
    .vertex_attribute_descriptions(&Vertex::attribute_descriptions()).build();
```
*Note*: No validation is done, the lifetimes only have to live as long as the builder object. It is the responsibility of the user to make sure that the pointers are valid.
### Flags and constants as associated constants

```Rust
dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
    | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
```

```Rust
pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
```
### Debug/Display for Flags

```Rust
let flag = vk::AccessFlags::COLOR_ATTACHMENT_READ
        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE;
println!("Debug: {:?}", flag);
println!("Display: {}", flag);
// Prints:
// Debug: AccessFlags(110000000)
// Display: COLOR_ATTACHMENT_READ | COLOR_ATTACHMENT_WRITE
```

### Interop

```Rust
PipelineBindPoint::from_raw(bindpoint);
```

### Function pointer loading
Ash also takes care of loading the function pointers. Function pointers are split into 3 categories, Entry, Instance and Device. The reason for not loading it into a global is that in Vulkan you can have multiple devices and each device will load its own function pointers to achieve better performance. Click [here](https://github.com/KhronosGroup/Vulkan-LoaderAndValidationLayers/blob/master/loader/LoaderAndLayerInterface.md) for more information.

### Extension loading
Additionally, every Vulkan extension has to be loaded explicitly. You can find all extensions under [ash::extensions](https://github.com/MaikKlein/ash/tree/master/ash/src/extensions).
```Rust
use ash::extensions::Swapchain;
let swapchain_loader = Swapchain::new(&instance, &device);
let swapchain = swapchain_loader.create_swapchain_khr(&swapchain_create_info).unwrap();
```

### Support for extension names
```Rust
use ash::extensions::{Swapchain, XlibSurface, Surface, DebugReport};
#[cfg(all(unix, not(target_os = "android")))]
fn extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr()
    ]
}
```

### Implicit handles
Handles from Instance or Device are passed implicitly.
```Rust
pub fn create_command_pool(&self,
                           create_info: &vk::CommandPoolCreateInfo)
                           -> VkResult<vk::CommandPool>;

let pool = device.create_command_pool(&pool_create_info).unwrap();
```

## Example
You can find the examples [here](https://github.com/MaikKlein/ash/tree/master/examples).
All examples currently require: the LunarG Validation layers and a Vulkan library that is visible in your `PATH`. An easy way to get started is to use the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/)
#### Windows
Make sure that you have a Vulkan ready driver and install the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/).
#### Linux
Make sure that you have a Vulkan ready driver and install the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/). You also have to add the library and layers to your path. Have a look at my [post](http://askubuntu.com/a/803110/77183) if you are unsure how to do that.
### [Triangle](https://github.com/MaikKlein/ash/blob/master/examples/src/bin/triangle.rs)
Displays a triangle with vertex colors.
```
cd examples
cargo run --bin triangle
```

![screenshot](http://i.imgur.com/PQZcL6w.jpg)

### [Texture](https://github.com/MaikKlein/ash/blob/master/examples/src/bin/texture.rs)
Displays a texture on a quad.
```
cd examples
cargo run --bin texture
```
![texture](http://i.imgur.com/trow00H.png)

## A thanks to

* [Api with no secrets](https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-1)
* [Vulkan tutorial](http://av.dfki.de/~jhenriques/development.html)
* [Vulkan examples](https://github.com/SaschaWillems/Vulkan)
* [Vulkan tutorial](https://vulkan-tutorial.com/)
* [Vulkano](https://github.com/tomaka/vulkano/)
* [vk-rs](https://github.com/Osspial/vk-rs)
