use std::{error::Error, fmt::Display};

use super::ConfigError;

// ConfigError traits implementations
impl Display for super::ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            super::ConfigError::CONFIG_ERROR_PREFIX,
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
    DuplicateKey(String),
    FileError(String),
    JsonError(String),
    YamlError(String),
    TomlError(String),
    BuildError(String),

    // serde
    SerdeError(String),
}

impl ConfigErrorImpl {
    pub const CONFIG_ERROR_IMPL_SERDE_PREFIX: &'static str = "[CONFIG][ERROR][SERDE]";
}

impl Display for ConfigErrorImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigErrorImpl::DuplicateKey(v) => {
                write!(f, "Overwrite is false, found key: {} is duplicated", v)
            }
            ConfigErrorImpl::FileError(v) => {
                write!(f, "File error: {}", v)
            }
            ConfigErrorImpl::JsonError(v) => {
                write!(f, "Json parsing error: {}", v)
            }
            ConfigErrorImpl::YamlError(v) => {
                write!(f, "Yaml parsing error: {}", v)
            }
            ConfigErrorImpl::TomlError(v) => {
                write!(f, "Toml parsing error: {}", v)
            }
            ConfigErrorImpl::BuildError(v) => {
                write!(f, "Failed building config: {}", v)
            }
            ConfigErrorImpl::SerdeError(v) => {
                write!(f, "Failed parsing error into serde: {}", v)
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

impl serde::de::Error for ConfigErrorImpl {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(msg.to_string())
    }
}

impl serde::de::Error for ConfigError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            config_error_impl: ConfigErrorImpl::SerdeError(msg.to_string()),
        }
    }
}
