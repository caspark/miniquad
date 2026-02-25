#![allow(dead_code)]

use std::sync::mpsc;

/// Raw window handle data for each platform, stored after window creation.
/// This is the miniquad-internal representation; when the `rwh-06` feature is
/// enabled, these are converted to `raw_window_handle` types.
#[derive(Debug, Clone, Copy)]
pub enum RawWindowHandleData {
    #[cfg(target_os = "linux")]
    Xlib {
        window: std::ffi::c_ulong,
        visual_id: std::ffi::c_ulong,
    },
    #[cfg(target_os = "linux")]
    Wayland {
        surface: *mut std::ffi::c_void,
    },
    #[cfg(target_os = "windows")]
    Win32 {
        hwnd: isize,
        hinstance: isize,
    },
    #[cfg(target_vendor = "apple")]
    AppKit {
        ns_view: *mut std::ffi::c_void,
    },
    #[cfg(target_arch = "wasm32")]
    Web {
        /// Index into JS object table (for raw-window-handle WebCanvasWindowHandle)
        canvas_id: u32,
    },
}

// SAFETY: The raw pointers stored here are OS window handles that remain valid
// for the lifetime of the window. They are created and used on the main thread.
unsafe impl Send for RawWindowHandleData {}
unsafe impl Sync for RawWindowHandleData {}

/// Raw display handle data for each platform.
#[derive(Debug, Clone, Copy)]
pub enum RawDisplayHandleData {
    #[cfg(target_os = "linux")]
    Xlib {
        display: *mut std::ffi::c_void,
        screen: i32,
    },
    #[cfg(target_os = "linux")]
    Wayland {
        display: *mut std::ffi::c_void,
    },
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_vendor = "apple")]
    AppKit,
    #[cfg(target_arch = "wasm32")]
    Web,
}

unsafe impl Send for RawDisplayHandleData {}
unsafe impl Sync for RawDisplayHandleData {}

#[derive(Default)]
pub(crate) struct DroppedFiles {
    pub paths: Vec<std::path::PathBuf>,
    pub bytes: Vec<Vec<u8>>,
}
pub(crate) struct NativeDisplayData {
    pub screen_width: i32,
    pub screen_height: i32,
    pub screen_position: (u32, u32),
    pub dpi_scale: f32,
    pub high_dpi: bool,
    pub quit_requested: bool,
    pub quit_ordered: bool,
    pub native_requests: mpsc::Sender<Request>,
    pub clipboard: Box<dyn Clipboard>,
    pub dropped_files: DroppedFiles,
    pub blocking_event_loop: bool,

    /// Raw window handle for external rendering libraries (e.g. wgpu).
    pub raw_window_handle: Option<RawWindowHandleData>,
    /// Raw display handle for external rendering libraries (e.g. wgpu).
    pub raw_display_handle: Option<RawDisplayHandleData>,

    #[cfg(target_vendor = "apple")]
    pub view: crate::native::apple::frameworks::ObjcId,
    #[cfg(target_os = "ios")]
    pub view_ctrl: crate::native::apple::frameworks::ObjcId,
    #[cfg(target_vendor = "apple")]
    pub gfx_api: crate::conf::AppleGfxApi,
}
#[cfg(target_vendor = "apple")]
unsafe impl Send for NativeDisplayData {}
#[cfg(target_vendor = "apple")]
unsafe impl Sync for NativeDisplayData {}

impl NativeDisplayData {
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        native_requests: mpsc::Sender<Request>,
        clipboard: Box<dyn Clipboard>,
    ) -> NativeDisplayData {
        NativeDisplayData {
            screen_width,
            screen_height,
            screen_position: (0, 0),
            dpi_scale: 1.,
            high_dpi: false,
            quit_requested: false,
            quit_ordered: false,
            native_requests,
            clipboard,
            dropped_files: Default::default(),
            blocking_event_loop: false,
            raw_window_handle: None,
            raw_display_handle: None,
            #[cfg(target_vendor = "apple")]
            gfx_api: crate::conf::AppleGfxApi::OpenGl,
            #[cfg(target_vendor = "apple")]
            view: std::ptr::null_mut(),
            #[cfg(target_os = "ios")]
            view_ctrl: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Request {
    ScheduleUpdate,
    SetCursorGrab(bool),
    ShowMouse(bool),
    SetMouseCursor(crate::CursorIcon),
    SetWindowSize { new_width: u32, new_height: u32 },
    SetWindowPosition { new_x: u32, new_y: u32 },
    SetFullscreen(bool),
    ShowKeyboard(bool),
}

pub trait Clipboard: Send + Sync {
    fn get(&mut self) -> Option<String>;
    fn set(&mut self, string: &str);
}

pub mod module;

#[cfg(target_os = "linux")]
pub mod linux_x11;

#[cfg(target_os = "linux")]
pub mod linux_wayland;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod apple;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod egl;

// there is no glGetProcAddr on webgl, so its impossible to make "gl" module work
// on macos.. well, there is, but way easier to just statically link to gl
#[cfg(not(target_arch = "wasm32"))]
pub mod gl;

#[cfg(target_arch = "wasm32")]
pub use wasm::webgl as gl;

pub mod query_stab;
