use std::{error::Error, fmt::Display};

// ConfigError traits implementations
impl Display for super::ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            super::CONFIG_ERROR_PREFIX,
            self.config_error_impl
        )
    }
}
impl Error for super::ConfigError {}

// ConfigError factory from ConfigErrorImpl
impl From<ConfigErrorImpl> for super::ConfigError {
    fn from(value: ConfigErrorImpl) -> Self {
        Self {
            config_error_impl: value,
        }
    }
}

// ConfigErrorImpl
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ConfigErrorImpl {
    ParseError(String),
    FileError(String),
    JsonError(String),
    YamlError(String),
    TomlError(String),
    EnvError(String),
    BuildError(String),
}

impl Display for ConfigErrorImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigErrorImpl::ParseError(v) => writeln!(f, "Parsing error: {}", v),
            ConfigErrorImpl::FileError(v) => {
                writeln!(f, "File error: {}", v)
            }
            ConfigErrorImpl::JsonError(v) => {
                writeln!(f, "Json parsing error: {}", v)
            }
            ConfigErrorImpl::YamlError(v) => {
                writeln!(f, "Yaml parsing error: {}", v)
            }
            ConfigErrorImpl::TomlError(v) => {
                writeln!(f, "Toml parsing error: {}", v)
            }
            ConfigErrorImpl::EnvError(v) => {
                writeln!(f, "Env parsing error: {}", v)
            }
            ConfigErrorImpl::BuildError(v) => {
                writeln!(f, "Failed building config: {}", v)
            }
        }
    }
}
impl Error for ConfigErrorImpl {}

// from serde_json::Error to ConfigErrorImpl
impl From<serde_json::Error> for ConfigErrorImpl {
    fn from(value: serde_json::Error) -> Self {
        Self::BuildError(value.to_string())
    }
}
