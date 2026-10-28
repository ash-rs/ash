ash-swapchain
=

Synchronization for swapchain resize and presentation
-

[![Crates.io Version](https://img.shields.io/crates/v/ash-window.svg)](https://crates.io/crates/ash-swapchain)
[![Documentation](https://docs.rs/ash-window/badge.svg)](https://docs.rs/ash-swapchain)
[![Build Status](https://github.com/MaikKlein/ash/workflows/CI/badge.svg)](https://github.com/MaikKlein/ash/actions?workflow=CI)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)

## Usage

A `Swapchain` manages (re)creating a `vk::SwapchainKHR` associated with a particular
`vk::SurfaceKHR`, and tracks the availability of resources associated with a particular *frame in
flight*. `Swapchain::acquire` yields an `AcquiredFrame` which specifies which swapchain image to
render to, which frame's resources are available for reuse, how to synchronize submitted work with
frame and swapchain image availability, and whether previously exposed swapchain images have been
invalidated.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
