
<h1 align="center">ash-window</h1>
<p align="center">
    <a href="https://crates.io/crates/ash-window">
      <img src="https://img.shields.io/crates/v/ash-window?style=flat-square" alt = "crates.io">
    </a>
    <a href="https://docs.rs/ash-window">
      <img src="https://docs.rs/ash-window/badge.svg?style=flat-square" alt="docs">
    </a>
    <a href="https://github.com/MaikKlein/ash/actions">
      <img src="https://github.com/MaikKlein/ash/workflows/CI/badge.svg?style=flat" alt="ci">
    </a>
    <br>
    <a href="LICENSE-MIT">
      <img src="https://img.shields.io/badge/license-MIT-green.svg?style=flat-square" alt="License - MIT">
    </a>
    <a href="LICENSE-APACHE">
      <img src="https://img.shields.io/badge/license-APACHE2-green.svg?style=flat-square" alt="License - Apache2">
    </a>
</p>

Interoperability between [`ash`](https://github.com/MaikKlein/ash) and [`raw-window-handle`](https://github.com/rust-windowing/raw-window-handle) for surface creation.

```toml
ash-window = "0.4"
```

## Usage

The library exposes two functions:

- `enumerate_required_extensions` returns the required instance extensions needed for surface creation from a specific window handle.

- `create_surface` allows to create a surface from a type implementing `HasRawWindowHandle`

```rust
ash_window::create_surface(&entry, &instance, &window, None)?;
```

## Versions
```toml
ash = "0.31"
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
