### Changelog

## [Unreleased]

### Changed
- `impl HasRawWindowHandle` to `dyn HasRawWindowHandle`

## Version 0.4.1

### Changed
- Use `raw-window-metal` to automatically allocate a `CAMetalLayer` if there is none

## Version 0.4.0

### Changed
- Update `ash` version to 0.31

## Version 0.3.0

### Changed
- Update `ash` version to 0.30

## Version 0.2.0

### Changed
- `enumerate_required_extension` renamed to `enumerate_required_extensions`
- `enumerate_required_extensions` will return an error if the window handle is not supported instead of panic.
- `enumerate_required_extensions` may return multiple extension names. Includes all dependent extensions.
- `create_surface` will return an error if the window handle is not supported instead of panic.

## Version 0.1.0
Initial release for `raw-window-handle = "0.3"` with Windows, Linux, Android, MacOS/iOS support.