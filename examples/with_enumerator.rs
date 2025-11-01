//! 与 window-enumerator 集成的示例
//! 运行: cargo run --example with_enumerator --features window-enumerator

#[cfg(feature = "window-enumerator")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use window_enumerator::WindowEnumerator;
    use window_enumerator_formatter::{OutputFormat, WindowListFormat};

    println!("=== 与 window-enumerator 集成示例 ===\n");

    // 枚举所有窗口
    let mut enumerator = WindowEnumerator::new();
    enumerator.enumerate_all_windows()?;
    let windows: Vec<_> = enumerator.get_windows().iter().map(|w| w.into()).collect();

    println!("找到 {} 个窗口\n", windows.len());

    // 显示前5个窗口的表格
    if windows.len() > 5 {
        println!("前5个窗口 (表格格式):");
        println!("{}", windows[..5].format_with(OutputFormat::Table));
    } else {
        println!("所有窗口 (表格格式):");
        println!("{}", windows.format_with(OutputFormat::Table));
    }

    println!("\nJSON 格式示例:");
    if let Some(window) = windows.first() {
        println!("{}", window.format_with(OutputFormat::JsonPretty));
    }

    Ok(())
}

#[cfg(not(feature = "window-enumerator"))]
fn main() {
    println!("此示例需要启用 'window-enumerator' 特性");
    println!("请使用: cargo run --example with_enumerator --features window-enumerator");
}
