//! 自定义模板使用示例

use std::path::PathBuf;
use window_enumerator_formatter::{
    FormatConfig, OutputFormat, TemplateFormat, WindowInfo, WindowPosition,
};

fn main() {
    let window = WindowInfo::builder()
        .hwnd(1001)
        .pid(1234)
        .title("我的应用程序".to_string())
        .class_name("MyAppClass".to_string())
        .process_name("myapp.exe".to_string())
        .process_file(PathBuf::from("C:\\Program Files\\MyApp\\myapp.exe"))
        .index(1)
        .position(WindowPosition {
            x: 100,
            y: 200,
            width: 1024,
            height: 768,
        })
        .build();

    println!("=== 自定义模板示例 ===\n");

    // 字段值模板
    let config = FormatConfig {
        format: OutputFormat::Custom,
        template: Some(TemplateFormat::Fields(vec![
            "index".into(),
            "pid".into(),
            "title".into(),
            "process".into(),
        ])),
        ..Default::default()
    };
    println!("1. 字段值模板 (制表符分隔):");
    println!("{}", window.format(&config));
    println!();

    // 键值对模板
    let config = FormatConfig {
        format: OutputFormat::Custom,
        template: Some(TemplateFormat::KeyValue(vec![
            "index".into(),
            "hwnd".into(),
            "pid".into(),
            "title".into(),
        ])),
        ..Default::default()
    };
    println!("2. 键值对模板:");
    println!("{}", window.format(&config));
    println!();

    // 自定义字符串模板
    let templates = [
        "窗口[{index}] | 进程: {process} (PID: {pid})",
        "标题: {title} | 位置: ({x}, {y}) | 大小: {width}x{height}",
        "句柄: {hwnd} | 类名: {class} | 文件: {file}",
    ];

    for (i, template) in templates.iter().enumerate() {
        let config = FormatConfig {
            format: OutputFormat::Custom,
            template: Some(TemplateFormat::Custom(template.to_string())),
            ..Default::default()
        };
        println!("3.{}. 自定义字符串模板:", i + 1);
        println!("模板: {}", template);
        println!("输出: {}", window.format(&config));
        println!();
    }
}
