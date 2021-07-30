Ash-window
=

Interoperability between [`ash`](https://github.com/MaikKlein/ash) and [`raw-window-handle`](https://github.com/rust-windowing/raw-window-handle) for surface creation.
-

[![Crates.io Version](https://img.shields.io/crates/v/ash-window.svg)](https://crates.io/crates/ash-window)
[![Documentation](https://docs.rs/ash-window/badge.svg)](https://docs.rs/ash-window)
[![Build Status](https://github.com/MaikKlein/ash/workflows/CI/badge.svg)](https://github.com/MaikKlein/ash/actions?workflow=CI)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Join the chat at https://gitter.im/MaikKlein/ash-window](https://badges.gitter.im/MaikKlein/ash-window.svg)](https://gitter.im/MaikKlein/ash-window?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

## Usage

```toml
ash-window = "0.6"
```

The library exposes two functions:

- [`enumerate_required_extensions`] returns the required instance extensions needed for surface creation from a specific window handle.

- [`create_surface`] allows to create a surface from a type implementing [`HasRawWindowHandle`]:

  ```rust
  ash_window::create_surface(&entry, &instance, &window, None)?;
  ```

[`enumerate_required_extensions`]: https://docs.rs/ash-window/latest/ash_window/fn.enumerate_required_extensions.html
[`create_surface`]: https://docs.rs/ash-window/latest/ash_window/fn.create_surface.html
[`HasRawWindowHandle`]: https://docs.rs/raw-window-handle/latest/raw_window_handle/trait.HasRawWindowHandle.html

## Versions
```toml
ash = "0.33"
raw-window-handle = "0.3"
```

## Support

- [x] Windows (`VK_KHR_win32_surface`)
- [x] Unix (`VK_KHR_xlib_surface`/`VK_KHR_xcb_surface`/`VK_KHR_wayland_surface`)
- [x] MacOS/IOS (`VK_EXT_metal_surface`)
- [x] Android (`VK_KHR_android_surface`)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
