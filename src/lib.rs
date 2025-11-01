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
//! - **Easy to Use**: Simple API with rich examples
//! - **Optional Integration**: Optional feature for `window-enumerator` integration
//!
//! # Quick Start
//!
//! ## Basic usage with custom data:
//! ```
//! use window_enumerator_formatter::{
//!     WindowInfo, WindowPosition,
//!     OutputFormat, WindowListFormat
//! };
//! use std::path::PathBuf;
//!
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
//! println!("{}", windows.format_with(OutputFormat::Json));
//! ```
//!
//! ## With window-enumerator integration (requires feature):
//! ```ignore
//! use window_enumerator::WindowEnumerator;
//! use window_enumerator_formatter::{OutputFormat, WindowListFormat};
//!
//! let mut enumerator = WindowEnumerator::new();
//! enumerator.enumerate_all_windows().unwrap();
//! let windows: Vec<_> = enumerator.get_windows().iter().map(|w| w.into()).collect();
//!
//! println!("{}", windows.format_with(OutputFormat::Table));
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

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        FormatConfig, OutputFormat, TemplateFormat, WindowInfo, WindowListFormat, WindowPosition,
    };
}
