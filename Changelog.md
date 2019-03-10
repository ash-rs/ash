# 0.28.0
* Fixed a build issue on ARM.
* *Breaking* Arrays are now passed by reference.
* Builders are now marked as `#[transparent]`.
* *Breaking*  Renamed `.next(..)` to `push_next`. `push_next` is only available on structs that are passed directly. Addtionally `push_next` only accepts structs that can be inserted into the pointer chain. Read the readme for more information.
* New *experimental* extensions. Those do not follow the semver rules and can be removed at any time.
* Added `AmdGpaInterface` extension.

# 0.27.0/1

* Extensions are now namespaced. `ash::extensions::khr::Swapchain`
* Removed vendor tags from extension methods
* Added missing functions for VkEvent
* The examples were updated to use the new builder pattern
* A SPIR-V parsing function `ash::util::read_spv`
* Added `get_pipeline_cache_data`

# 0.26.0

* Fix loader on MacOS.

* Expose function pointers for easier interop with external libraries.

* Builder now uses bool instead of Bool32.
# 0.25.0

* Adds support for Vulkan 1.1

* Constants are not represented as an `enum` anymore. Constants and flags are both represented as associated constants.

```Rust
flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
//to
flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER_BIT,

```

* Pretty printing for flags

* Handles can be loaded from outside of ash. See `SomeHandle::from_raw`. This is useful if you need to interact with a C library.

* Removing versioning from ash. `V1_X` are now gone. Versioning had very little benefit in practice, `Entry`, `Instance` and `Device` are now version free. A custom loader can still be implemented. The various traits still remain `DeviceV1_0`.

* `vk.rs` is now generated from `vk.xml`

* Ash now includes all docs inside the `vk.xml`, and are visible in rustdoc.

* `Default` is now implemented for all structs

* There is now a builder pattern

* Handles are now `#[repr(transparent)]`

* Various bug fixes


# 0.18.0
* Fixes arm build => uses libc everywhere. Remove `AlignByteSlice`.

# 0.17.0

* Refactor Align to use vk::DeviceSize.

# 0.16.0

* `map_memory` now returns a void ptr

* `ash::util::Align` is a helper struct that
can write to aligned memory.
