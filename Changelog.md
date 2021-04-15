### Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

## [0.32.1] - 2021-03-29

### Added

- `VK_KHR_create_renderpass2` device extension

## [0.32.0] - 2021-03-07

### Added

- Khr raytracing support
- `VK_NV_device_diagnostics_config` device extension
- Add `VK_KHR_deferred_host_operations` extension support
- Expose header version and extension spec version constants
- Provide Vulkan library loader from custom path

### Changed

- Removed deprecated experimental raytracing extension
- Update Vulkan-Headers to 1.2.168
- Update libloading from 0.6 to 0.7

### Fixed

- Turn c_void-returning functions into Rust ()
- Use best guess for ggp extension types
- Use raw pointers to static-sized arrays in FFI signatures
- rename parameter of `get_physical_device_surface_support`


## [0.31.0] - 2020-05-10

### Added

- `libloading` is now an optional dependency, but still used by default
- Add metal surface extension
- Implement `VK_KHR_draw_indirect_count`
- Added const qualifier to `as_raw` and `from_raw` fns on enums

### Changed

- The `vk::Device` parameter in 1.2 functions is now implicit
- Moved library creation out of `Entry::new_custom`

### Fixed

- Initialize MemoryRequirements with `Default` instead of `zeroed`

## [0.30.0] - 2020-03-22

### Added

- Support for Vulkan 1.2 (Generated from spec 1.2.135)
- Add `VK_KHR_timeline_semaphore` extension support
- Add `VK_KHR_ray_tracing` extension support
- Add `VK_KHR_external_memory_fd` extension support
- More safety docs
- Expose raw function pointers in extensions

### Changed

- Switch to [libloading](https://github.com/nagisa/rust_libloading)
- Vulkan version macros are now const functions
- Switched to a new [changelog](https://keepachangelog.com/en/1.0.0/) format

### Fixed

- Fix XCB types
- Fix OSX build errors of the examples


## Before 0.30.0

### 0.29.0
- -Breaking-: Removed Display impl for flags. The Debug impl now reports flags by name.
- Functions now have a doc comment that links to the Vulkan spec
- Entry has a new method called `try_enumerate_instance_version` which can be used in a 1.0 context.
- The generator now uses `BTreeMap` for better diffs.
### 0.28.0
- Switched to a new [changelog](https://keepachangelog.com/en/1.0.0/) format
- Fixed a build issue on ARM.
- -Breaking- Arrays are now passed by reference.
- Builders are now marked as `#[transparent]`.
- -Breaking-  Renamed `.next(..)` to `push_next`. `push_next` is only available on structs that are passed directly. Addtionally `push_next` only accepts structs that can be inserted into the pointer chain. Read the readme for more information.
- New -experimental- extensions. Those do not follow the semver rules and can be removed at any time.
- Added `AmdGpaInterface` extension.

# 0.27.0/1

- Extensions are now namespaced. `ash::extensions::khr::Swapchain`
- Removed vendor tags from extension methods
- Added missing functions for VkEvent
- The examples were updated to use the new builder pattern
- A SPIR-V parsing function `ash::util::read_spv`
- Added `get_pipeline_cache_data`

### 0.26.0

- Fix loader on MacOS.

- Expose function pointers for easier interop with external libraries.

- Builder now uses bool instead of Bool32.
### 0.25.0

- Adds support for Vulkan 1.1

- Constants are not represented as an `enum` anymore. Constants and flags are both represented as associated constants.

```Rust
flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
//to
flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER_BIT,

```

- Pretty printing for flags

- Handles can be loaded from outside of ash. See `SomeHandle::from_raw`. This is useful if you need to interact with a C library.

- Removing versioning from ash. `V1_X` are now gone. Versioning had very little benefit in practice, `Entry`, `Instance` and `Device` are now version free. A custom loader can still be implemented. The various traits still remain `DeviceV1_0`.

- `vk.rs` is now generated from `vk.xml`

- Ash now includes all docs inside the `vk.xml`, and are visible in rustdoc.

- `Default` is now implemented for all structs

- There is now a builder pattern

- Handles are now `#[repr(transparent)]`

- Various bug fixes


### 0.18.0
- Fixes arm build => uses libc everywhere. Remove `AlignByteSlice`.

### 0.17.0

- Refactor Align to use vk::DeviceSize.

### 0.16.0

- `map_memory` now returns a void ptr

- `ash::util::Align` is a helper struct that
can write to aligned memory.


[Unreleased]: https://github.com/MaikKlein/ash/compare/0.32.0...HEAD
[0.32.1]: https://github.com/MaikKlein/ash/releases/tag/0.32.1
[0.32.0]: https://github.com/MaikKlein/ash/releases/tag/0.32.0
[0.31.0]: https://github.com/MaikKlein/ash/releases/tag/0.31.0
[0.30.0]: https://github.com/MaikKlein/ash/releases/tag/0.30.0
