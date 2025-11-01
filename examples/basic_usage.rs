//! 基本用法示例：创建窗口信息并使用不同格式输出

use std::path::PathBuf;
use window_enumerator_formatter::{OutputFormat, WindowInfo, WindowListFormat, WindowPosition};

fn main() {
    // 创建一些示例窗口数据
    let windows = vec![
        WindowInfo::builder()
            .hwnd(1001)
            .pid(1234)
            .title("Google Chrome".to_string())
            .class_name("Chrome_WidgetWin_1".to_string())
            .process_name("chrome.exe".to_string())
            .process_file(PathBuf::from(
                "C:\\Program Files\\Google\\Chrome\\chrome.exe",
            ))
            .index(1)
            .position(WindowPosition {
                x: 100,
                y: 50,
                width: 1920,
                height: 1040,
            })
            .build(),
        WindowInfo::builder()
            .hwnd(1002)
            .pid(5678)
            .title("Notepad".to_string())
            .class_name("Notepad".to_string())
            .process_name("notepad.exe".to_string())
            .process_file(PathBuf::from("C:\\Windows\\notepad.exe"))
            .index(2)
            .position(WindowPosition {
                x: 50,
                y: 100,
                width: 800,
                height: 600,
            })
            .build(),
        WindowInfo::builder()
            .hwnd(1003)
            .pid(9012)
            .title("Calculator".to_string())
            .class_name("ApplicationFrameWindow".to_string())
            .process_name("calculator.exe".to_string())
            .process_file(PathBuf::from("C:\\Windows\\System32\\calculator.exe"))
            .index(3)
            .position(WindowPosition {
                x: 200,
                y: 150,
                width: 400,
                height: 500,
            })
            .build(),
    ];

    println!("=== 基本格式示例 ===\n");

    // 表格格式
    println!("1. 表格格式:");
    println!("{}", windows.format_with(OutputFormat::Table));

    // 简单格式
    println!("\n2. 简单格式:");
    println!("{}", windows.format_with(OutputFormat::Simple));

    // JSON 格式
    println!("\n3. JSON 格式:");
    println!("{}", windows.format_with(OutputFormat::Json));
}
