以下是更新后的英文文档，反映库的最新架构和特性：

## README.md

```markdown
# Window Enumerator Formatter

[![Crates.io](https://img.shields.io/crates/v/window-enumerator-formatter)](https://crates.io/crates/window-enumerator-formatter)
[![Documentation](https://docs.rs/window-enumerator-formatter/badge.svg)](https://docs.rs/window-enumerator-formatter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful formatting library for window information with multiple output formats and template support. Works standalone or integrates with `window-enumerator`.

## Features ✨

- 🎨 **Multiple Formats**: JSON, YAML, CSV, Table, Simple, and Detailed formats
- 🎯 **Template System**: Field selection, key-value pairs, and custom templates
- 🔧 **Highly Configurable**: Title truncation, header control, and output customization
- 🚀 **Easy to Use**: Simple API with rich examples and builder pattern
- 📦 **Flexible**: Works standalone or with `window-enumerator` integration
- 🔌 **Optional Dependencies**: Minimal required dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
window-enumerator-formatter = "0.1"
```

For `window-enumerator` integration, enable the feature:

```toml
[dependencies]
window-enumerator-formatter = { version = "0.1", features = ["window-enumerator"] }
window-enumerator = "0.4"
```

## Quick Start

### Standalone Usage

```rust
use window_enumerator_formatter::{
    WindowInfo, WindowPosition, OutputFormat, WindowListFormat
};
use std::path::PathBuf;

// Create window data using builder pattern
let window = WindowInfo::builder()
    .hwnd(12345)
    .pid(1234)
    .title("Test Window".to_string())
    .class_name("TestClass".to_string())
    .process_name("test.exe".to_string())
    .process_file(PathBuf::from("test.exe"))
    .index(1)
    .position(WindowPosition::default())
    .build();

let windows = vec![window];

// Format with different output formats
println!("JSON: {}", windows.format_with(OutputFormat::Json));
println!("Table: {}", windows.format_with(OutputFormat::Table));
println!("CSV: {}", windows.format_with(OutputFormat::Csv));
```

### With window-enumerator Integration

```rust
use window_enumerator::WindowEnumerator;
use window_enumerator_formatter::{OutputFormat, WindowListFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut enumerator = WindowEnumerator::new();
    enumerator.enumerate_all_windows()?;
    let windows: Vec<_> = enumerator.get_windows().iter().map(|w| w.into()).collect();

    // Multiple output formats
    println!("JSON: {}", windows.format_with(OutputFormat::Json));
    println!("Pretty JSON: {}", windows.format_with(OutputFormat::JsonPretty));
    println!("Table: {}", windows.format_with(OutputFormat::Table));
    println!("YAML: {}", windows.format_with(OutputFormat::Yaml));

    Ok(())
}
```

## Usage Examples

### Multiple Output Formats

```rust
use window_enumerator_formatter::{OutputFormat, WindowListFormat};

let windows = get_windows(); // Your window data

// All available formats
println!("JSON: {}", windows.format_with(OutputFormat::Json));
println!("Pretty JSON: {}", windows.format_with(OutputFormat::JsonPretty));
println!("YAML: {}", windows.format_with(OutputFormat::Yaml));
println!("CSV: {}", windows.format_with(OutputFormat::Csv));
println!("Table: {}", windows.format_with(OutputFormat::Table));
println!("Simple: {}", windows.format_with(OutputFormat::Simple));
println!("Detail: {}", windows.format_with(OutputFormat::Detail));
```

### Template System

```rust
use window_enumerator_formatter::{
    OutputFormat, TemplateFormat, FormatConfig, WindowInfo
};

let window = WindowInfo::builder()
    .hwnd(0x12345)
    .pid(1234)
    .title("My Application".to_string())
    .build();

// Field values only (tab-separated)
let config = FormatConfig {
    format: OutputFormat::Custom,
    template: Some(TemplateFormat::Fields(vec![
        "index".into(),
        "pid".into(),
        "title".into()
    ])),
    ..Default::default()
};
println!("Fields: {}", window.format(&config));
// Output: "1    1234    My Application"

// Key-value pairs
let config = FormatConfig {
    format: OutputFormat::Custom,
    template: Some(TemplateFormat::KeyValue(vec![
        "index".into(),
        "hwnd".into(),
        "title".into()
    ])),
    ..Default::default()
};
println!("Key-Value: {}", window.format(&config));
// Output: "index: 1 | hwnd: 0x12345 | title: My Application"

// Custom template string
let config = FormatConfig {
    format: OutputFormat::Custom,
    template: Some(TemplateFormat::Custom(
        "Window[{index}] | PID:{pid} | Title:{title}".into()
    )),
    ..Default::default()
};
println!("Custom: {}", window.format(&config));
// Output: "Window[1] | PID:1234 | Title:My Application"
```

### Advanced Configuration

```rust
use window_enumerator_formatter::{OutputFormat, FormatConfig, WindowListFormat};

let windows = get_windows();

// Custom configuration
let config = FormatConfig {
    format: OutputFormat::Table,
    show_headers: false,           // Hide table headers
    max_title_length: Some(20),    // Truncate long titles
    ..Default::default()
};

println!("{}", windows.format_output(&config));
```

## Available Template Fields

Use these field names in custom templates:

- `{index}` - Window index
- `{hwnd}` - Window handle (hex format)
- `{pid}` - Process ID
- `{title}` - Window title
- `{class}` - Window class name
- `{process}` - Process name
- `{file}` - Process file path
- `{x}`, `{y}` - Window position
- `{width}`, `{height}` - Window size

## Examples

Run the provided examples to see the library in action:

```bash
# Basic usage
cargo run --example basic_usage

# All output formats
cargo run --example multiple_formats

# Custom templates
cargo run --example custom_templates

# Advanced configuration
cargo run --example advanced_config

# With window-enumerator integration
cargo run --example with_enumerator --features window-enumerator
```

## Features

- **default**: No additional dependencies
- **window-enumerator**: Enables integration with `window-enumerator` crate
- **all**: Enables all features

## Supported Formats

| Format | Description | Use Case |
|--------|-------------|----------|
| **JSON** | Compact JSON format | APIs, data exchange |
| **JSON Pretty** | Formatted JSON with indentation | Debugging, configuration |
| **YAML** | YAML format | Configuration files |
| **CSV** | Comma-separated values | Spreadsheets, data analysis |
| **Table** | Formatted table | Command-line display |
| **Simple** | One-line format | Logging, quick overview |
| **Detail** | Multi-line detailed format | Debugging, full information |
| **Custom** | Template-based format | Custom output requirements |

## Documentation

- [Full API Documentation](https://docs.rs/window-enumerator-formatter)
- [Examples Directory](./examples/)

## License

MIT License - see [LICENSE](LICENSE) file for details.

This project is licensed under either of

[MIT License](./LICENSE-MIT)

[Apache License, Version 2.0](./LICENSE-APACHE)
