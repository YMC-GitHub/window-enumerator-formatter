//! A powerful formatting library for window information with multiple output formats and templates.
//!
//! This crate provides flexible formatting capabilities for window information,
//! supporting various output formats and customizable templates.
//!
//! # Features
//!
//! - **Multiple Formats**: JSON, YAML, CSV, Table, Simple, and Detailed formats
//! - **Template System**: Field selection, key-value pairs, and custom templates
//! - **Highly Configurable**: Title truncation, header control, and output customization
//! - **Easy to Use**: Simple API with rich examples and builder pattern
//! - **Flexible**: Works standalone or with `window-enumerator` integration
//! - **Optional Dependencies**: Minimal required dependencies
//!
//! # Quick Start
//!
//! ## Standalone Usage
//!
//! ```
//! use window_enumerator_formatter::{
//!     WindowInfo, WindowPosition, OutputFormat, WindowListFormat
//! };
//! use std::path::PathBuf;
//!
//! // Create window data using builder pattern
//! let window = WindowInfo::builder()
//!     .hwnd(12345)
//!     .pid(1234)
//!     .title("Test Window".to_string())
//!     .class_name("TestClass".to_string())
//!     .process_name("test.exe".to_string())
//!     .process_file(PathBuf::from("test.exe"))
//!     .index(1)
//!     .position(WindowPosition::default())
//!     .build();
//!
//! let windows = vec![window];
//!
//! // Format with different output formats
//! println!("JSON: {}", windows.format_with(OutputFormat::Json));
//! println!("Table: {}", windows.format_with(OutputFormat::Table));
//! println!("CSV: {}", windows.format_with(OutputFormat::Csv));
//!
//! // Individual window formatting
//! println!("Single window: {}", windows[0].format_with(OutputFormat::Simple));
//! ```
//!
//! ## Using different output formats:
//! ```
//! use window_enumerator_formatter::{
//!     WindowInfo, WindowPosition, OutputFormat, WindowListFormat, FormatConfig
//! };
//! use std::path::PathBuf;
//!
//! let windows = vec![
//!     WindowInfo::builder()
//!         .hwnd(12345)
//!         .pid(1234)
//!         .title("Browser".to_string())
//!         .class_name("Chrome_WidgetWin_1".to_string())
//!         .process_name("chrome.exe".to_string())
//!         .process_file(PathBuf::from("C:\\Program Files\\Google\\Chrome\\chrome.exe"))
//!         .index(1)
//!         .position(WindowPosition { x: 100, y: 100, width: 1920, height: 1080 })
//!         .build(),
//!     WindowInfo::builder()
//!         .hwnd(67890)
//!         .pid(5678)
//!         .title("Text Editor".to_string())
//!         .class_name("Notepad".to_string())
//!         .process_name("notepad.exe".to_string())
//!         .process_file(PathBuf::from("C:\\Windows\\notepad.exe"))
//!         .index(2)
//!         .position(WindowPosition { x: 50, y: 50, width: 800, height: 600 })
//!         .build(),
//! ];
//!
//! // JSON format
//! println!("JSON:\n{}", windows.format_with(OutputFormat::Json));
//!
//! // Table format
//! println!("\nTable:\n{}", windows.format_with(OutputFormat::Table));
//!
//! // CSV format
//! println!("\nCSV:\n{}", windows.format_with(OutputFormat::Csv));
//!
//! // Simple format
//! println!("\nSimple:\n{}", windows.format_with(OutputFormat::Simple));
//! ```
//!
//! ## Using custom templates:
//! ```
//! use window_enumerator_formatter::{
//!     WindowInfo, WindowPosition, OutputFormat, TemplateFormat, FormatConfig
//! };
//! use std::path::PathBuf;
//!
//! let window = WindowInfo::builder()
//!     .hwnd(12345)
//!     .pid(1234)
//!     .title("My Application".to_string())
//!     .class_name("MyAppClass".to_string())
//!     .process_name("myapp.exe".to_string())
//!     .process_file(PathBuf::from("myapp.exe"))
//!     .index(1)
//!     .position(WindowPosition { x: 100, y: 200, width: 1024, height: 768 })
//!     .build();
//!
//! // Field values only template
//! let config = FormatConfig {
//!     format: OutputFormat::Custom,
//!     template: Some(TemplateFormat::Fields(vec![
//!         "index".into(),
//!         "pid".into(),
//!         "title".into()
//!     ])),
//!     ..Default::default()
//! };
//! println!("Fields: {}", window.format(&config));
//!
//! // Key-value template
//! let config = FormatConfig {
//!     format: OutputFormat::Custom,
//!     template: Some(TemplateFormat::KeyValue(vec![
//!         "index".into(),
//!         "hwnd".into(),
//!         "title".into()
//!     ])),
//!     ..Default::default()
//! };
//! println!("Key-Value: {}", window.format(&config));
//!
//! // Custom template string
//! let config = FormatConfig {
//!     format: OutputFormat::Custom,
//!     template: Some(TemplateFormat::Custom(
//!         "Window[{index}] | PID:{pid} | Title:{title}".into()
//!     )),
//!     ..Default::default()
//! };
//! println!("Custom: {}", window.format(&config));
//! ```
//!
//! ## Advanced configuration:
//! ```
//! use window_enumerator_formatter::{
//!     WindowInfo, WindowPosition, OutputFormat, FormatConfig, WindowListFormat
//! };
//! use std::path::PathBuf;
//!
//! let windows = vec![
//!     WindowInfo::builder()
//!         .hwnd(12345)
//!         .pid(1234)
//!         .title("This is a very long window title that might need truncation".to_string())
//!         .class_name("SomeClass".to_string())
//!         .process_name("app.exe".to_string())
//!         .process_file(PathBuf::from("app.exe"))
//!         .index(1)
//!         .position(WindowPosition::default())
//!         .build(),
//! ];
//!
//! let config = FormatConfig {
//!     format: OutputFormat::Table,
//!     show_headers: false,
//!     max_title_length: Some(20),
//!     ..Default::default()
//! };
//!
//! println!("{}", windows.format_output(&config));
//! ```
//!
//! ## With window-enumerator integration (requires feature):
//! ```ignore
//! // This example requires the `window-enumerator` feature
//! use window_enumerator::WindowEnumerator;
//! use window_enumerator_formatter::{OutputFormat, WindowListFormat};
//!
//! let mut enumerator = WindowEnumerator::new();
//! enumerator.enumerate_all_windows().unwrap();
//! let windows: Vec<_> = enumerator.get_windows().iter().map(|w| w.into()).collect();
//!
//! // Format as pretty JSON
//! println!("{}", windows.format_with(OutputFormat::JsonPretty));
//!
//! // Format as YAML
//! println!("{}", windows.format_with(OutputFormat::Yaml));
//!
//! // Format as detailed list
//! println!("{}", windows.format_with(OutputFormat::Detail));
//! ```

#![warn(missing_docs)]

mod error;
mod formatter;
mod models;

pub use error::FormatError;
pub use formatter::{
    FormatConfig, OutputFormat, TemplateFormat, WindowFormatter, WindowListFormat,
};
pub use models::{WindowInfo, WindowPosition};

// 为 WindowInfo 实现格式化方法，消除循环依赖
impl WindowInfo {
    /// Format this window according to the configuration.
    pub fn format(&self, config: &FormatConfig) -> String {
        WindowFormatter::format_window(self, config)
    }

    /// Format this window with a specific output format.
    pub fn format_with(&self, format: OutputFormat) -> String {
        let config = FormatConfig {
            format,
            ..Default::default()
        };
        self.format(&config)
    }
}

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        FormatConfig, OutputFormat, TemplateFormat, WindowInfo, WindowListFormat, WindowPosition,
    };
}
