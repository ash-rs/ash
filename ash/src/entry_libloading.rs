use crate::entry::EntryCustom;
use libloading::Library;
use std::error::Error;
use std::fmt;
use std::ptr;
use std::sync::Arc;

#[cfg(windows)]
const LIB_PATH: &str = "vulkan-1.dll";

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
const LIB_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &str = "libvulkan.dylib";

/// Function loader
pub type Entry = EntryCustom<Arc<Library>>;

#[derive(Debug)]
pub struct LoadingError(libloading::Error);

impl fmt::Display for LoadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for LoadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&self.0)
    }
}

impl EntryCustom<Arc<Library>> {
    /// ```rust,no_run
    /// use ash::{vk, Entry, version::EntryV1_0};
    /// # fn main() -> Result<(), Box<std::error::Error>> {
    /// let entry = Entry::new()?;
    /// let app_info = vk::ApplicationInfo {
    ///     api_version: vk::make_version(1, 0, 0),
    ///     ..Default::default()
    /// };
    /// let create_info = vk::InstanceCreateInfo {
    ///     p_application_info: &app_info,
    ///     ..Default::default()
    /// };
    /// let instance = unsafe { entry.create_instance(&create_info, None)? };
    /// # Ok(()) }
    /// ```
    pub fn new() -> Result<Entry, LoadingError> {
        let lib = Library::new(&LIB_PATH)
            .map_err(LoadingError)
            .map(Arc::new)?;

        Ok(Self::new_custom(lib, |vk_lib, name| unsafe {
            vk_lib
                .get(name.to_bytes_with_nul())
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null_mut())
        }))
    }
}
