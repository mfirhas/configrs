// Public APIs

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    path::Path,
};

use serde::de::DeserializeOwned;

/// Contains config errors constants/statics.
pub mod errors {}

/// Error type
///
#[derive(Debug)]
pub enum ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
        Self::Array(value.into_iter().map(|v| v.into()).collect::<Vec<Value>>())
    }
}

impl<V> From<HashMap<String, V>> for Value
where
    V: Into<Value>,
{
    fn from(value: HashMap<String, V>) -> Self {
        Self::Map(
            value
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<HashMap<String, Value>>(),
        )
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
pub struct Config();

impl Config {
    /// Initialized configs from environment variables.
    ///
    pub fn new() -> Self {
        todo!()
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
        todo!()
    }
}
