- 0.18.0: Fixes arm build => uses libc everywhere. Remove `AlignByteSlice`.

- 0.17.0: Refactor Align to use vk::DeviceSize.

- 0.16.0: `map_memory` now returns a void ptr. `ash::util::Align` is a helper struct that
can write to aligned memory.
