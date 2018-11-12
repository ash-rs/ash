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
