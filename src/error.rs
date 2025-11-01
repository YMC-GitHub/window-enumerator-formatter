use thiserror::Error;

/// Errors that can occur during formatting operations.
#[derive(Error, Debug)]
pub enum FormatError {
    /// The input window list is empty.
    #[error("Cannot format empty window list")]
    EmptyInput,

    /// Invalid field name in template.
    #[error("Invalid field name: {field}")]
    InvalidField {
        /// The invalid field name.
        field: String,
    },

    /// Template parsing error.
    #[error("Template parsing error: {message}")]
    TemplateError {
        /// Error message.
        message: String,
    },

    /// Serialization error.
    #[error("Serialization error: {source}")]
    SerializationError {
        /// The underlying error.
        #[from]
        source: serde_json::Error,
    },

    /// YAML serialization error.
    #[error("YAML serialization error: {source}")]
    YamlError {
        /// The underlying error.
        #[from]
        source: serde_yaml::Error,
    },

    /// Other unspecified errors.
    #[error("Formatting error: {message}")]
    Other {
        /// Error message.
        message: String,
    },
}

// 移除未使用的 Result 类型别名
// 因为在实际代码中没有使用这个类型别名
