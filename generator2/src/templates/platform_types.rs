pub type RROutput = c_ulong;
pub type VisualID = c_uint;
pub type Display = *const c_void;
pub type Window = c_ulong;
#[allow(non_camel_case_types)]
pub type xcb_connection_t = c_void;
#[allow(non_camel_case_types)]
pub type xcb_window_t = u32;
#[allow(non_camel_case_types)]
pub type xcb_visualid_t = u32;
pub type MirConnection = *const c_void;
pub type MirSurface = *const c_void;
pub type HINSTANCE = *const c_void;
pub type HWND = *const c_void;
#[allow(non_camel_case_types)]
pub type wl_display = c_void;
#[allow(non_camel_case_types)]
pub type wl_surface = c_void;
pub type HANDLE = *mut c_void;
pub type HMONITOR = HANDLE;
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;
#[allow(non_camel_case_types)]
pub type zx_handle_t = u32;

// FIXME: Platform specific types that should come from a library id:0
// typedefs are only here so that the code compiles for now
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = ();
// Opage types
pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;
/// This definition is experimental and won't adhere to semver rules.
pub type GgpStreamDescriptor = u32;
/// This definition is experimental and won't adhere to semver rules.
pub type GgpFrameToken = u32;
pub type CAMetalLayer = c_void;
