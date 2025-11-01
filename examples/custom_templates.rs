//! 展示所有支持的输出格式

use std::path::PathBuf;
use window_enumerator_formatter::{OutputFormat, WindowInfo, WindowListFormat, WindowPosition};

fn main() {
    let windows = vec![WindowInfo::builder()
        .hwnd(1001)
        .pid(1234)
        .title("示例窗口".to_string())
        .class_name("ExampleClass".to_string())
        .process_name("example.exe".to_string())
        .process_file(PathBuf::from("example.exe"))
        .index(1)
        .position(WindowPosition {
            x: 100,
            y: 100,
            width: 800,
            height: 600,
        })
        .build()];

    let formats = [
        (OutputFormat::Json, "JSON"),
        (OutputFormat::JsonPretty, "美化 JSON"),
        (OutputFormat::Yaml, "YAML"),
        (OutputFormat::Csv, "CSV"),
        (OutputFormat::Table, "表格"),
        (OutputFormat::Simple, "简单"),
        (OutputFormat::Detail, "详细"),
    ];

    for (format, name) in formats {
        println!("=== {}格式 ===", name);
        println!("{}", windows.format_with(format));
        println!();
    }
}
