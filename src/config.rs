use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    path::Path,
};

use serde::de::DeserializeOwned;

mod config_impl;

/// Valid values for configs
#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConfigError {
    config_error_impl: config_impl::ConfigErrorImpl,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[CONFIG][ERROR] {}", self)
    }
}

impl Error for ConfigError {}

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
            config_impl: config_impl::ConfigImpl::new(),
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
        Ok(self.config_impl.build::<T>()?)
    }
}
