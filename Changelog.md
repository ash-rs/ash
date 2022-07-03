### Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

### Changed

- Inlined builder setters (partial backport from #602)
- Inlined `Default` impls and trivial `Instance`/`Device`/`Entry` wrapper methods (#606, #632)

### Added

- Added `VK_EXT_image_drm_format_modifier` device extension (#603)
- Update Vulkan-Headers to 1.3.219 (#605, #608, #619)
- Added `VK_EXT_sample_locations` device extension (#616)
- Added `VK_NV_coverage_reduction_mode` device extension (#617)
- Added new functions to `VK_KHR_swapchain`, available since Vulkan 1.1 (#629)
- Added `VK_KHR_device_group_creation` instance extension (#630)
- Added `VK_KHR_device_group` device extension (#631)

## [0.37.0] - 2022-03-23

### Changed

- Dropped auto-generated wrapper methods from function pointer structs
  in favor of direct invocation of function pointers (#599)
- Constified extension names (#590)
- `VK_NV_device_diagnostic_checkpoints`: Enable passing `pNext`-initialized structs to `get_queue_checkpoint_data` (#588)

### Added

- Update Vulkan-Headers to 1.3.209 (#597, #601)
- Added `VK_EXT_headless_surface` instance extension (#589)

## [0.36.0] - 2022-02-21

### Changed

- extensions/khr: Drop `_khr` suffix from `get_memory_fd_properties_khr` in `ExternalMemoryFd` (#580)
- entry: Allow querying `enumerate_instance_extension_properties()` by layer name (#574)

### Added

- Add helper wrappers for Vulkan core 1.3 `Instance` and `Device` functions (#568)
- Update Vulkan-Headers to 1.3.206 (#563)

## [0.35.2] - 2022-02-19

### Changed

- Replace `1.2-extensions` documentation links with `1.3-extensions` (#569)
- Fixed broken (intradoc) links in the prelude and `VK_KHR_get_surface_capabilities2` extension (#559)

### Added

- Added `VK_KHR_external_fence_win32` device extension (#582)
- Added `VK_KHR_external_semaphore_win32` device extension (#581)
- Added `VK_KHR_external_memory_win32` device extension (#579)
- Added `VK_EXT_extended_dynamic_state2` device extension (#572)
- Added `VK_KHR_copy_commands2` device extension (#571)
- Added `VK_EXT_private_data` device extension (#570)
- Added conversions from `Extent2D` to `Extent3D` and `Rect2D` (#557)

## [0.35.1] - 2022-01-18

### Added

- Added `VK_EXT_calibrated_timestamps` device extension (#556)
- Added `VK_KHR_get_surface_capabilities2` device extension (#530)

### Changed

- Convert `vk_bitflags_wrapped!` methods to `const fn` (#549)
- examples: Update winit to 0.26 and image to 0.23 (#551)
- ash-window: Require at least `raw-window-handle 0.3.4` for 0.4 interop (#553)
- Assert that Vulkan array-getters return the same length (#534)
- README: Correct documentation for `Entry` functions and related crate features (#545)
- example: Refactor event loop handling for continuous redraw (#542)
- Generate `RGBA=R|G|B|A` helper constant for `ColorComponentFlags` (#537)
- Remove remaining `CString` allocations on string literals in examples and hand-written AMD extension (#533)

## [0.35.0] - 2021-12-27

### Changed

- `loaded` feature enabled by default in place of `linked` to relax default constraints on the build environment
- `Entry::new` renamed to `Entry::linked`

## [0.34.0] - 2021-12-22

### Added

- Update Vulkan-Headers to 1.2.203 (#477, #497, #504, #509, #514)
- Add missing documentation to bitflag extension variants (#501)
- Added `VK_KHR_present_wait` device extension (#493)
- Added `VK_KHR_maintenance4` device extension (#489, #498)
- Link `_len()` functions to their array-getter using intradoc-links (#490)
- Added `VK_KHR_dynamic_rendering` device extension (#488)

### Changed

- Extension names from `fn name()` will not be checked for interior nuls anymore at runtime (#522)
- examples: Use `c_char` for pointer to raw string (#521)
- Group enum extension variants together per `impl T` block (#519)
- examples: Use `slice::from_ref` to not loose lifetime on nested slices (#513)
- Simplify triangle example's vertex input state (#512)
- Device extension `khr::PipelineExecutableProperties` and `khr::TimelineSemaphore` now expose `fn device()` instead of `fn instance()` (#499)
- Changed `khr::PipelineExecutableProperties::new()` and `khr::TimelineSemaphore::new()` to take `instance` and `device` as arguments (#499)
- Fix broken vulkan-tutorial link in README.md (#492)
- Make `enumerate_physical_device_groups` unsafe (#491)
- Added `Packed24_8` helper-type for constructing AS Instance bitfields, used in `AccelerationStructureInstanceKHR`, `AccelerationStructureSRTMotionInstanceNV` and `AccelerationStructureMatrixMotionInstanceNV` (#476)s (#490)
- examples: Upgrade to winit 0.25 (#487)
- To allow faster builds, Vulkan structures only implement `Debug` if the `debug` feature is enabled, which is the default (#482)
- Use `Self` in macros (instead of `$name`) and `impl` blocks (#479)
- Link Vulkan directly under the default `linked` feature. Disable default features and enable the `loaded` feature for the old `libloading` behaviour (#457)

### Removed

- Deprecated aliases for wrongly-named enum constants (containing `"Backwards-compatible"` in the Vulkan spec comment) are removed: switch to the version that it aliased (#502)
- Removed `device()` function from `khr::Synchronization2` device extension (#494)
- Removed `instance()` function from `ext::ExtendedDynamicState`, `khr::PushDescriptor`, `ext::ToolingInfo` and `khr::GetPhysicalDeviceProperties2` instance extensions (#494)
- Removed `device` argument from `ext::DebugMarkers::debug_marker_set_object_name` function, `khr::PipelineExecutableProperties` and `khr::TimelineSemaphore` functions (#494, #499)
- Removed `From<vk::Result>` trait for `VkResult` (#495)
- Removed `instance` argument from `ext::DebugUtils::submit_debug_utils_message` function (#499)
- Removed misleading `all()`/`-`/`-=` function/ops from bitflags (#478)

## [0.33.3] - 2021-09-08

### Added

- Regenerated with Vulkan-Headers 1.2.191 (#463)

## [0.33.2] - 2021-08-26

### Fixed

- `tooling_info`: Initialize `sType`/`pNext` in `get_physical_device_tool_properties` (#465)
- Repeatedly call enumeration functions when `VK_INCOMPLETE` is returned (#465)

## [0.33.1] - 2021-08-23

### Fixed

- util: Zero-initialize result to prevent possible uninit memory read (#470)

## [0.33.0] - 2021-07-30

### Added

- Regenerated with Vulkan-Headers 1.2.186 (#456, #454, #446, #429)
- Generate `push_next` function for all extended structs (#305)
- Vulkan 1.2.175: Provisional Video Extensions (#417)
- Globally remove all `allow(dead_code)` exceptions and make extensions public (#430)
- Add extension wrapper for `VK_KHR_synchronization2` (#403)
- added missing functions to `V1_0` traits (#416)
- extensions/khr: Add `VK_KHR_external_fence_fd` wrapper (#413)
- Add `VK_EXT_extended_dynamic_state` extension (#421)
- Add `VK_KHR_get_physical_device_properties2` extension (#400)
- extensions/khr: Add `ExternalSemaphoreFd` safe wrapper (#395)
- Add `VK_KHR_get_memory_requirements2` extension (#401)
- Add `VK_EXT_full_screen_exclusive` extension (#399)
- Add `VK_NN_vi_surface` extension (#398)
- Add `merge_pipeline_caches`, `queue_bind_sparse`, `get_render_area_granularity` to `DeviceV1_0` (#397)
- Add `VK_KHR`/`EXT_buffer_device_address` extension (#405)
- Add `VK_KHR_maintenance` extensions (#406)

### Changed

- Use `PFN_` types for struct members instead of repeating function signature (#438)
- Untangle mismatched parameter/return `fn` signatures in types (#437)
- Mark all function pointer types as `unsafe` (#436)
- `pSampleMask` setter should write `NULL` if slice is empty (#432)
- vk/platform_types: Mark `SECURITY_ATTRIBUTES` as a true `ffi::c_void` (#433)
- Impl trait functions directly on `EntryCustom`/`Instance`/`Device` (#412)
- Improve `Result`'s `Display` impl for extension values (#424)
- Use lifetime borrows instead of raw pointers in `pp_geometries` (#420)
- `Entry::new` returns `Err` when entry point isn't found (#390)
- Remove the `_mvk` suffix from the two extensions' functions (#407)

### Fixed

- ash/extensions: Fix missing and broken autolinks to Vulkan docs (#459)
- Adds `LICENSE-*` files to crate subdirectories (#452)
- external_memory_fd: Initialize output struct with proper `sType` (#394)
- entry_libloading: Do not pass `AsRef` implementation by reference (#389)

## [0.32.1] - 2021-03-29

### Added

- Add high-level extension wrapper for the 1.1 extension `VK_KHR_create_renderpass2` (#414)

## [0.32.0] - 2021-03-07

### Added

- Final KHR RayTracing support (`VK_KHR_ray_tracing_pipeline`, `VK_KHR_ray_query` and `VK_KHR_acceleration_structure` device extensions)
- `VK_NV_device_diagnostics_config` device extension
- `VK_KHR_deferred_host_operations` device extension
- Expose header version and extension spec version constants
- Provide Vulkan library loader from custom path

### Changed

- Removed deprecated experimental `VK_KHR_ray_tracing` extension
- Update Vulkan-Headers to 1.2.168
- Update libloading from 0.6 to 0.7

### Fixed

- Turn `c_void`-returning functions into Rust `()`
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
- -Breaking-  Renamed `.next(..)` to `push_next`. `push_next` is only available on structs that are passed directly. Additionally `push_next` only accepts structs that can be inserted into the pointer chain. Read the readme for more information.
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


[Unreleased]: https://github.com/MaikKlein/ash/compare/0.37.0...HEAD
[0.37.0]: https://github.com/MaikKlein/ash/releases/tag/0.37.0
[0.36.0]: https://github.com/MaikKlein/ash/releases/tag/0.36.0
[0.35.2]: https://github.com/MaikKlein/ash/releases/tag/0.35.2
[0.35.1]: https://github.com/MaikKlein/ash/releases/tag/0.35.1
[0.35.0]: https://github.com/MaikKlein/ash/releases/tag/0.35.0
[0.34.0]: https://github.com/MaikKlein/ash/releases/tag/0.34.0
[0.33.3]: https://github.com/MaikKlein/ash/releases/tag/0.33.3
[0.33.2]: https://github.com/MaikKlein/ash/releases/tag/0.33.2
[0.33.1]: https://github.com/MaikKlein/ash/releases/tag/0.33.1
[0.33.0]: https://github.com/MaikKlein/ash/releases/tag/0.33.0
[0.32.1]: https://github.com/MaikKlein/ash/releases/tag/0.32.1
[0.32.0]: https://github.com/MaikKlein/ash/releases/tag/0.32.0
[0.31.0]: https://github.com/MaikKlein/ash/releases/tag/0.31.0
[0.30.0]: https://github.com/MaikKlein/ash/releases/tag/0.30.0
