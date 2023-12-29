// Public APIs

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    path::Path,
};

use serde::de::DeserializeOwned;

use crate::config_impl::{self, ConfigImpl};

/// Contains config errors constants/statics.
pub mod errors {}

#[derive(Debug)]
pub enum ConfigError {
    ParseError(String),
    FileError(String),
    JsonError(String),
    YamlError(String),
    TomlError(String),
    EnvError(String),
    BuildError(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ParseError(v) => writeln!(f, "[CONFIG][ERROR] Parsing error: {}", v),
            ConfigError::FileError(v) => writeln!(f, "[CONFIG][ERROR] Parsing File error: {}", v),
            ConfigError::JsonError(v) => writeln!(f, "[CONFIG][ERROR] Json parsing error: {}", v),
            ConfigError::YamlError(v) => writeln!(f, "[CONFIG][ERROR] Yaml parsing error: {}", v),
            ConfigError::TomlError(v) => writeln!(f, "[CONFIG][ERROR] Toml parsing error: {}", v),
            ConfigError::EnvError(v) => writeln!(f, "[CONFIG][ERROR] Ini/Env parsing error: {}", v),
            ConfigError::BuildError(v) => {
                writeln!(f, "[CONFIG][ERROR] Failed building config: {}", v)
            }
        }
    }
}

impl Error for ConfigError {}

/// Valid values for configs
#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Self::String(value.into())
    }
}

impl<V> From<Vec<V>> for Value
where
    V: Into<Value>,
{
    fn from(value: Vec<V>) -> Self {
        Self::Array(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<V> From<HashMap<String, V>> for Value
where
    V: Into<Value>,
{
    fn from(value: HashMap<String, V>) -> Self {
        Self::Map(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::Bool(v) => {
                write!(f, "{}", v)
            }
            Value::Int64(v) => {
                write!(f, "{}", v)
            }
            Value::Float64(v) => {
                write!(f, "{}", v)
            }
            Value::String(ref v) => {
                write!(f, "{}", v)
            }
            Value::Array(ref v) => write!(f, "{:?}", {
                v.iter().map(|e| format!("{}, ", e)).collect::<String>()
            }),
            Value::Map(ref v) => write!(f, "{{ {} }}", {
                v.iter()
                    .map(|(k, v)| format!("{} => {}, ", k, v))
                    .collect::<String>()
            }),
        }
    }
}

/// Starting point to build your configs.
#[derive(Debug)]
pub struct Config {
    config_impl: config_impl::ConfigImpl,
}

impl Config {
    /// Initialized configs from environment variables.
    ///
    pub fn new() -> Self {
        Self {
            config_impl: ConfigImpl::new(),
        }
    }

    /// Set key and value programmatically.
    ///
    /// Accepted `Value`: `&str`, `i64`, `f64`, `bool`, `Vector<Value>`, `HashMap<String, Value>`
    pub fn with_value<V>(mut self, key: &str, value: V) -> Self
    where
        V: Into<Value> + Debug,
    {
        todo!()
    }

    /// Take configs only with this prefix from all sources.
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        todo!()
    }

    /// Overwrite previous already existing configs keys
    pub fn with_overwrite(mut self) -> Self {
        todo!()
    }

    /// Add configs from .env file
    pub fn with_env(mut self, file_path: impl AsRef<Path>) -> Self {
        todo!()
    }

    /// Add configs from .json file
    pub fn with_json(mut self, file_path: impl AsRef<Path>) -> Self {
        todo!()
    }

    /// Add configs from .toml file
    pub fn with_toml(mut self, file_path: impl AsRef<Path>) -> Self {
        todo!()
    }

    /// Add configs from .yaml file
    pub fn with_yaml(mut self, file_path: impl AsRef<Path>) -> Self {
        todo!()
    }

    /// Build configs into T
    pub fn build<T>(self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned + Debug,
    {
        self.config_impl.build::<T>()
    }
}
