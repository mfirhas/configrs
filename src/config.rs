// TODO: Document codes.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

mod config_error_impl;
mod config_impl;
mod value_impl;

/// Valid values for configs
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Value {
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    None,
}

static CONFIG_ERROR_PREFIX: &str = "[CONFIG][ERROR]";
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigError {
    config_error_impl: config_error_impl::ConfigErrorImpl,
}

/// Starting point to build your configs.
#[derive(Clone)]
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
    /// Accepted `Value`:
    /// - `&str`/`String`,
    /// - `i64`,
    /// - `f64`,
    /// - `bool`,
    /// - `Vector<Value>`,
    /// - `HashMap<String, Value>`
    pub fn with_value<V>(mut self, key: &str, value: V) -> Self
    where
        V: Into<Value> + Debug,
    {
        Self {
            config_impl: self.config_impl.with_value(key, value),
        }
    }

    /// Take configs only with this prefix from all sources.
    pub fn with_env_prefix(mut self, prefix: &str) -> Self {
        todo!()
    }

    /// Overwrite previous already existing configs keys
    pub fn with_overwrite(mut self) -> Self {
        todo!()
    }

    /// Add configs from .env file
    pub fn with_env(mut self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_env(file_path),
        }
    }

    /// Add configs from .json file
    pub fn with_json(mut self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_json(file_path),
        }
    }

    /// Add configs from .toml file
    pub fn with_toml(mut self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_toml(file_path),
        }
    }

    /// Add configs from .yaml file
    pub fn with_yaml(mut self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_yaml(file_path),
        }
    }

    /// Build configs into T
    pub fn build<T>(self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned + Debug,
    {
        Ok(self.config_impl.build::<T>()?)
    }
}
