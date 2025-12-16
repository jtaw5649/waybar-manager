use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("IO error: {context} - {source}")]
    Io {
        context: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse JSON: {context} - {source}")]
    JsonParse {
        context: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("Network error: {context} - {source}")]
    Network {
        context: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Archive extraction failed: {0}")]
    Archive(String),

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Module not found: {0}")]
    NotFound(String),

    #[error("Config error: {0}")]
    Config(String),
}

impl ServiceError {
    pub fn io(context: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }

    pub fn json_parse(context: impl Into<String>, source: serde_json::Error) -> Self {
        Self::JsonParse {
            context: context.into(),
            source,
        }
    }

    pub fn network(context: impl Into<String>, source: reqwest::Error) -> Self {
        Self::Network {
            context: context.into(),
            source,
        }
    }

    pub fn archive(message: impl Into<String>) -> Self {
        Self::Archive(message.into())
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn not_found(uuid: impl Into<String>) -> Self {
        Self::NotFound(uuid.into())
    }

    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = ServiceError::io("reading config", io_err);
        assert!(err.to_string().contains("reading config"));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = ServiceError::validation("invalid UUID format");
        assert_eq!(err.to_string(), "Invalid input: invalid UUID format");
    }

    #[test]
    fn test_not_found_error_display() {
        let err = ServiceError::not_found("test-module@namespace");
        assert_eq!(err.to_string(), "Module not found: test-module@namespace");
    }

    #[test]
    fn test_archive_error_display() {
        let err = ServiceError::archive("corrupted tarball");
        assert_eq!(err.to_string(), "Archive extraction failed: corrupted tarball");
    }

    #[test]
    fn test_config_error_display() {
        let err = ServiceError::config("invalid waybar config syntax");
        assert_eq!(err.to_string(), "Config error: invalid waybar config syntax");
    }
}
