use crate::models::WindowInfo;

/// Supported output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Compact JSON format.
    Json,
    /// Pretty-printed JSON format.
    JsonPretty,
    /// CSV format.
    Csv,
    /// YAML format.
    Yaml,
    /// Simple one-line format.
    Simple,
    /// Detailed multi-line format.
    Detail,
    /// Formatted table.
    Table,
    /// Custom template format.
    Custom,
}

/// Template formats for custom output.
#[derive(Debug, Clone)]
pub enum TemplateFormat {
    /// Output only the values of specified fields (tab-separated).
    Fields(Vec<String>),
    /// Output field names and values.
    KeyValue(Vec<String>),
    /// Custom template string with placeholders.
    Custom(String),
}

/// Configuration for formatting output.
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// The output format to use.
    pub format: OutputFormat,
    /// Template configuration for custom formats.
    pub template: Option<TemplateFormat>,
    /// Whether to show headers in CSV/Table formats.
    pub show_headers: bool,
    /// Maximum title length before truncation.
    pub max_title_length: Option<usize>,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Table,
            template: None,
            show_headers: true,
            max_title_length: Some(50),
        }
    }
}

/// Main formatter for window information.
pub struct WindowFormatter;

impl WindowFormatter {
    /// Format a single window according to the configuration.
    pub fn format_window(window: &WindowInfo, config: &FormatConfig) -> String {
        match config.format {
            OutputFormat::Json => {
                serde_json::to_string(window).unwrap_or_else(|_| "{}".to_string())
            }
            OutputFormat::JsonPretty => {
                serde_json::to_string_pretty(window).unwrap_or_else(|_| "{}".to_string())
            }
            OutputFormat::Yaml => {
                serde_yaml::to_string(window).unwrap_or_else(|_| "---".to_string())
            }
            OutputFormat::Simple => Self::format_simple(window, config),
            OutputFormat::Detail => Self::format_detail(window, config),
            OutputFormat::Table => Self::format_table_single(window),
            OutputFormat::Custom => Self::format_custom(window, config),
            OutputFormat::Csv => Self::format_csv_single(window),
        }
    }

    /// Format a list of windows according to the configuration.
    pub fn format_windows(windows: &[WindowInfo], config: &FormatConfig) -> String {
        if windows.is_empty() {
            return "No windows found".to_string();
        }

        match config.format {
            OutputFormat::Json => {
                serde_json::to_string(windows).unwrap_or_else(|_| "[]".to_string())
            }
            OutputFormat::JsonPretty => {
                serde_json::to_string_pretty(windows).unwrap_or_else(|_| "[]".to_string())
            }
            OutputFormat::Yaml => {
                serde_yaml::to_string(windows).unwrap_or_else(|_| "---".to_string())
            }
            OutputFormat::Simple => Self::format_simple_list(windows, config),
            OutputFormat::Detail => Self::format_detail_list(windows, config),
            OutputFormat::Table => Self::format_table(windows, config),
            OutputFormat::Custom => Self::format_custom_list(windows, config),
            OutputFormat::Csv => Self::format_csv(windows, config),
        }
    }

    // Simple format - single window
    fn format_simple(window: &WindowInfo, config: &FormatConfig) -> String {
        if let Some(template) = &config.template {
            return Self::apply_template(window, template);
        }

        let title = Self::truncate_title(&window.title, config.max_title_length);
        format!(
            "[{}] {} (PID: {}) @ ({},{})",
            window.index, title, window.pid, window.position.x, window.position.y
        )
    }

    // Detailed format - single window
    fn format_detail(window: &WindowInfo, _config: &FormatConfig) -> String {
        format!(
            "Index: {}\n\
             Handle: 0x{:x}\n\
             PID: {}\n\
             Title: {}\n\
             Class: {}\n\
             Process: {}\n\
             File: {}\n\
             Position: ({}, {}) Size: {}x{}\n\
             {}",
            window.index,
            window.hwnd,
            window.pid,
            window.title,
            window.class_name,
            window.process_name,
            window.process_file.display(),
            window.position.x,
            window.position.y,
            window.position.width,
            window.position.height,
            "-".repeat(40)
        )
    }

    // Table format - list
    fn format_table(windows: &[WindowInfo], config: &FormatConfig) -> String {
        let mut output = String::new();

        // Header
        if config.show_headers {
            output.push_str(&format!(
                "{:<6} {:<12} {:<8} {:<12} {}\n",
                "Index", "Handle", "PID", "Position", "Title"
            ));
            output.push_str(&format!(
                "{:-<6} {:-<12} {:-<8} {:-<12} {:-<30}\n",
                "", "", "", "", ""
            ));
        }

        // Rows
        for window in windows {
            let title = Self::truncate_title(&window.title, config.max_title_length);
            output.push_str(&format!(
                "{:<6} 0x{:<10x} {:<8} {:4},{:<7} {}\n",
                window.index, window.hwnd, window.pid, window.position.x, window.position.y, title
            ));
        }

        output
    }

    // Table format - single window
    fn format_table_single(window: &WindowInfo) -> String {
        Self::format_table(std::slice::from_ref(window), &FormatConfig::default())
    }

    // CSV format - list
    fn format_csv(windows: &[WindowInfo], config: &FormatConfig) -> String {
        let mut output = String::new();

        if config.show_headers {
            output.push_str("Index,Handle,PID,Title,Class,Process,File,X,Y,Width,Height\n");
        }

        for window in windows {
            let title = Self::escape_csv_field(&window.title);
            let class_name = Self::escape_csv_field(&window.class_name);
            let process_name = Self::escape_csv_field(&window.process_name);
            let file_path = Self::escape_csv_field(&window.process_file.to_string_lossy());

            output.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                window.index,
                window.hwnd,
                window.pid,
                title,
                class_name,
                process_name,
                file_path,
                window.position.x,
                window.position.y,
                window.position.width,
                window.position.height
            ));
        }

        output
    }

    // CSV format - single window
    fn format_csv_single(window: &WindowInfo) -> String {
        Self::format_csv(std::slice::from_ref(window), &FormatConfig::default())
    }

    // Simple format list
    fn format_simple_list(windows: &[WindowInfo], config: &FormatConfig) -> String {
        windows
            .iter()
            .map(|w| Self::format_simple(w, config))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Detailed format list
    fn format_detail_list(windows: &[WindowInfo], config: &FormatConfig) -> String {
        windows
            .iter()
            .map(|w| Self::format_detail(w, config))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Custom template formatting
    fn format_custom(window: &WindowInfo, config: &FormatConfig) -> String {
        if let Some(template) = &config.template {
            Self::apply_template(window, template)
        } else {
            Self::format_simple(window, config)
        }
    }

    // Custom template list
    fn format_custom_list(windows: &[WindowInfo], config: &FormatConfig) -> String {
        windows
            .iter()
            .map(|w| Self::format_custom(w, config))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Apply template
    fn apply_template(window: &WindowInfo, template: &TemplateFormat) -> String {
        match template {
            TemplateFormat::Fields(fields) => Self::format_fields(window, fields),
            TemplateFormat::KeyValue(fields) => Self::format_key_value(window, fields),
            TemplateFormat::Custom(template_str) => {
                Self::format_custom_template(window, template_str)
            }
        }
    }

    // Output only field values
    fn format_fields(window: &WindowInfo, fields: &[String]) -> String {
        let values: Vec<String> = fields
            .iter()
            .map(|field| Self::get_field_value(window, field))
            .collect();

        values.join("\t")
    }

    // Output field names and values
    fn format_key_value(window: &WindowInfo, fields: &[String]) -> String {
        fields
            .iter()
            .map(|field| {
                let value = Self::get_field_value(window, field);
                format!("{}: {}", field, value)
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }

    // Custom template string
    fn format_custom_template(window: &WindowInfo, template: &str) -> String {
        let mut result = template.to_string();

        // Replace template variables
        let replacements = [
            ("{index}", &window.index.to_string()),
            ("{hwnd}", &format!("0x{:x}", window.hwnd)),
            ("{pid}", &window.pid.to_string()),
            ("{title}", &window.title),
            ("{class}", &window.class_name),
            ("{process}", &window.process_name),
            (
                "{file}",
                &window.process_file.to_string_lossy().into_owned(),
            ),
            ("{x}", &window.position.x.to_string()),
            ("{y}", &window.position.y.to_string()),
            ("{width}", &window.position.width.to_string()),
            ("{height}", &window.position.height.to_string()),
        ];

        for (pattern, replacement) in replacements {
            result = result.replace(pattern, replacement);
        }

        result
    }

    // Get field value
    fn get_field_value(window: &WindowInfo, field: &str) -> String {
        match field.to_lowercase().as_str() {
            "index" => window.index.to_string(),
            "hwnd" => format!("0x{:x}", window.hwnd),
            "pid" => window.pid.to_string(),
            "title" => window.title.clone(),
            "class" => window.class_name.clone(),
            "process" => window.process_name.clone(),
            "file" => window.process_file.to_string_lossy().to_string(),
            "x" => window.position.x.to_string(),
            "y" => window.position.y.to_string(),
            "width" => window.position.width.to_string(),
            "height" => window.position.height.to_string(),
            _ => format!("[unknown field: {}]", field),
        }
    }

    // Utility functions
    fn truncate_title(title: &str, max_length: Option<usize>) -> String {
        if let Some(max) = max_length {
            if title.len() > max {
                format!("{}...", &title[..max - 3])
            } else {
                title.to_string()
            }
        } else {
            title.to_string()
        }
    }

    fn escape_csv_field(field: &str) -> String {
        if field.contains(',') || field.contains('"') || field.contains('\n') {
            format!("\"{}\"", field.replace('"', "\"\""))
        } else {
            field.to_string()
        }
    }
}

/// Extension trait for formatting window information.
pub trait WindowListFormat {
    /// Format windows according to the configuration.
    fn format_output(&self, config: &FormatConfig) -> String;

    /// Format windows with a specific output format.
    fn format_with(&self, format: OutputFormat) -> String;
}

impl WindowListFormat for [WindowInfo] {
    fn format_output(&self, config: &FormatConfig) -> String {
        WindowFormatter::format_windows(self, config)
    }

    fn format_with(&self, format: OutputFormat) -> String {
        let config = FormatConfig {
            format,
            ..Default::default()
        };
        self.format_output(&config)
    }
}

impl WindowListFormat for Vec<WindowInfo> {
    fn format_output(&self, config: &FormatConfig) -> String {
        WindowFormatter::format_windows(self, config)
    }

    fn format_with(&self, format: OutputFormat) -> String {
        let config = FormatConfig {
            format,
            ..Default::default()
        };
        self.format_output(&config)
    }
}
