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
- [x] Convenience features without limiting functionality
- [x] Additional type safety
- [x] Device local function pointer loading
- [x] No validation, everything is **unsafe**
- [x] Generated from `vk.xml`
- [x] Support for Vulkan 1.1

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


### `Vec<T>` instead of mutable slices

```Rust
pub fn get_swapchain_images(&self,
                            swapchain: vk::SwapchainKHR)
                            -> VkResult<Vec<vk::Image>>;
let present_images = swapchain_loader.get_swapchain_images_khr(swapchain).unwrap();
```
*Note*: Functions don't return `Vec<T>` if this would limit the functionality. See `p_next`.

### Slices
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

### Strongly typed handles

Each Vulkan handle type is exposed as a newtyped struct for improved type safety. Null handles can be constructed with
`T::null()`, and handles may be freely converted to and from `u64` with `Handle::from_raw` and `Handle::as_raw` for
interop with non-Ash Vulkan code.

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

Builders implement `Deref` targeting their corresponding Vulkan struct, so references to builders can be passed directly
to Vulkan functions. This is encouraged as doing so allows Rust to check the lifetimes of captured objects are valid,
whereas calling `build` discards lifetime information, making inadvertent use-after-free errors more likely.

### Flags and constants as associated constants

```Rust
// Bitflag
vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE
```

```Rust
// Constant
vk::PipelineBindPoint::GRAPHICS,
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

### Function pointer loading
Ash also takes care of loading the function pointers. Function pointers are split into 3 categories.

* Entry: Loads the Vulkan library. Needs to outlive `Instance` and `Device`.
* Instance: Loads instance level functions. Needs to outlive the `Device`s it has created.
* Device: Loads device **local** functions.

The loader is just one possible implementation:

* Device level functions are retrieved on a per device basis.
* Everything is loaded by default, functions that failed to load are initialized to a function that always panics.
* Do not call Vulkan 1.1 functions if you have created a 1.0 instance. Doing so will result in a panic.

Custom loaders can be implemented.

### Extension loading
Additionally, every Vulkan extension has to be loaded explicitly. You can find all extensions under [ash::extensions](https://github.com/MaikKlein/ash/tree/master/ash/src/extensions).
```Rust
use ash::extensions::khr::Swapchain;
let swapchain_loader = Swapchain::new(&instance, &device);
let swapchain = swapchain_loader.create_swapchain(&swapchain_create_info).unwrap();
```

### Raw function pointers

Raw function pointers are available, if something hasn't been exposed yet in the higher level API. Please open an issue if anything is missing.

```Rust
device.fp_v1_0().destroy_device(...);
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
#### macOS
Install the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/). This basically entails extracting the downloaded tarball to any location you choose and then setting a few environment variables. Specifically, if `SDK_PATH` is set to the root extracted SDK directory,

* `DYLD_LIBRARY_PATH = $SDK_PATH/macOS/lib`
* `VK_ICD_FILENAMES = $SDK_PATH/macOS/etc/vulkan/icd.d/MoltenVK_icd.json`
* `VK_LAYER_PATH = $SDK_PATH/macOS/etc/vulkan/explicit_layer.d`

![screenshot](http://i.imgur.com/PQZcL6w.jpg)

### [Texture](https://github.com/MaikKlein/ash/blob/master/examples/src/bin/texture.rs)
Displays a texture on a quad.
```
cd examples
cargo run --bin texture
```
![texture](http://i.imgur.com/trow00H.png)

## Useful resources

### Examples

* [vulkan-tutorial-rust](https://github.com/Usami-Renko/vulkan-tutorial-rust) - A port of [vulkan-tutorial.com](vulkan-tutorial.com).
* [ash-sample-progression](https://github.com/bzm3r/ash-sample-progression) - A port of the LunarG examples.
* [ash-nv-rt](https://github.com/gwihlidal/ash-nv-rt) A raytracing example for ash.

### Utility libraries
* [vk-sync](https://github.com/gwihlidal/vk-sync-rs) - Simplified Vulkan synchronization logic, written in rust.
* [vk-mem-rs](https://github.com/gwihlidal/vk-mem-rs) - This crate provides an FFI layer and idiomatic rust wrappers for the excellent AMD Vulkan Memory Allocator (VMA) C/C++ library.
* [lahar](https://github.com/Ralith/lahar) - Tools for asynchronously uploading data to a Vulkan device.

### Libraries that use ash
* [gfx-rs](https://github.com/gfx-rs/gfx) - gfx-rs is a low-level, cross-platform graphics abstraction library in Rust.

## A thanks to

* [Api with no secrets](https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-1)
* [Vulkan tutorial](http://av.dfki.de/~jhenriques/development.html)
* [Vulkan examples](https://github.com/SaschaWillems/Vulkan)
* [Vulkan tutorial](https://vulkan-tutorial.com/)
* [Vulkano](https://github.com/tomaka/vulkano/)
* [vk-rs](https://github.com/Osspial/vk-rs)
