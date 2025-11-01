//! 高级配置选项示例

use std::path::PathBuf;
use window_enumerator_formatter::{
    FormatConfig, OutputFormat, WindowInfo, WindowListFormat, WindowPosition,
};

fn main() {
    let windows = vec![
        WindowInfo::builder()
            .hwnd(1001)
            .pid(1234)
            .title("这是一个非常长的窗口标题，需要被截断显示".to_string())
            .class_name("VeryLongClassNameExample".to_string())
            .process_name("long_title_app.exe".to_string())
            .process_file(PathBuf::from(
                "C:\\Program Files\\LongTitleApp\\long_title_app.exe",
            ))
            .index(1)
            .position(WindowPosition {
                x: 100,
                y: 100,
                width: 800,
                height: 600,
            })
            .build(),
        WindowInfo::builder()
            .hwnd(1002)
            .pid(5678)
            .title("短标题".to_string())
            .class_name("ShortClass".to_string())
            .process_name("short.exe".to_string())
            .process_file(PathBuf::from("short.exe"))
            .index(2)
            .position(WindowPosition {
                x: 200,
                y: 200,
                width: 400,
                height: 300,
            })
            .build(),
    ];

    println!("=== 高级配置示例 ===\n");

    // 配置1: 无表头，标题截断
    let config1 = FormatConfig {
        format: OutputFormat::Table,
        show_headers: false,
        max_title_length: Some(15),
        ..Default::default()
    };
    println!("1. 无表头 + 标题截断 (15字符):");
    println!("{}", windows.format_output(&config1));
    println!();

    // 配置2: 无标题截断
    let config2 = FormatConfig {
        format: OutputFormat::Table,
        show_headers: true,
        max_title_length: None, // 不截断标题
        ..Default::default()
    };
    println!("2. 有表头 + 无标题截断:");
    println!("{}", windows.format_output(&config2));
    println!();

    // 配置3: CSV 无表头
    let config3 = FormatConfig {
        format: OutputFormat::Csv,
        show_headers: false,
        max_title_length: Some(20),
        ..Default::default()
    };
    println!("3. CSV 无表头 + 标题截断:");
    println!("{}", windows.format_output(&config3));
}
