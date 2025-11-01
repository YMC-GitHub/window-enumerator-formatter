use serde::Serialize;
use std::path::PathBuf;

/// Window position and size information.
#[derive(Debug, Clone, Serialize, Default)]
pub struct WindowPosition {
    /// X coordinate of the window.
    pub x: i32,
    /// Y coordinate of the window.
    pub y: i32,
    /// Width of the window.
    pub width: i32,
    /// Height of the window.
    pub height: i32,
}

/// Information about a window.
#[derive(Debug, Clone, Serialize)]
pub struct WindowInfo {
    /// Window handle.
    pub hwnd: isize,
    /// Process ID.
    pub pid: u32,
    /// Window title.
    pub title: String,
    /// Window class name.
    pub class_name: String,
    /// Process name.
    pub process_name: String,
    /// Process file path.
    pub process_file: PathBuf,
    /// Index in the enumeration.
    pub index: usize,
    /// Window position and size.
    pub position: WindowPosition,
}

impl WindowInfo {
    /// Create a new WindowInfo instance using builder pattern.
    pub fn builder() -> WindowInfoBuilder {
        WindowInfoBuilder::default()
    }

    // format 和 format_with 方法已移到 lib.rs 中
}

/// Builder for WindowInfo to avoid too many arguments.
#[derive(Default)]
pub struct WindowInfoBuilder {
    hwnd: Option<isize>,
    pid: Option<u32>,
    title: Option<String>,
    class_name: Option<String>,
    process_name: Option<String>,
    process_file: Option<PathBuf>,
    index: Option<usize>,
    position: Option<WindowPosition>,
}

impl WindowInfoBuilder {
    /// Set the window handle.
    pub fn hwnd(mut self, hwnd: isize) -> Self {
        self.hwnd = Some(hwnd);
        self
    }

    /// Set the process ID.
    pub fn pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }

    /// Set the window title.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the window class name.
    pub fn class_name(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    /// Set the process name.
    pub fn process_name(mut self, process_name: String) -> Self {
        self.process_name = Some(process_name);
        self
    }

    /// Set the process file path.
    pub fn process_file(mut self, process_file: PathBuf) -> Self {
        self.process_file = Some(process_file);
        self
    }

    /// Set the index.
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Set the window position.
    pub fn position(mut self, position: WindowPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Build the WindowInfo instance.
    pub fn build(self) -> WindowInfo {
        WindowInfo {
            hwnd: self.hwnd.unwrap_or(0),
            pid: self.pid.unwrap_or(0),
            title: self.title.unwrap_or_default(),
            class_name: self.class_name.unwrap_or_default(),
            process_name: self.process_name.unwrap_or_default(),
            process_file: self.process_file.unwrap_or_default(),
            index: self.index.unwrap_or(0),
            position: self.position.unwrap_or_default(),
        }
    }
}

/// Conversion from window-enumerator types
#[cfg(feature = "window-enumerator")]
impl From<&window_enumerator::WindowInfo> for WindowInfo {
    fn from(window: &window_enumerator::WindowInfo) -> Self {
        Self::builder()
            .hwnd(window.hwnd)
            .pid(window.pid)
            .title(window.title.clone())
            .class_name(window.class_name.clone())
            .process_name(window.process_name.clone())
            .process_file(window.process_file.clone())
            .index(window.index)
            .position(WindowPosition {
                x: window.position.x,
                y: window.position.y,
                width: window.position.width,
                height: window.position.height,
            })
            .build()
    }
}

#[cfg(feature = "window-enumerator")]
impl From<window_enumerator::WindowInfo> for WindowInfo {
    fn from(window: window_enumerator::WindowInfo) -> Self {
        Self::from(&window)
    }
}
