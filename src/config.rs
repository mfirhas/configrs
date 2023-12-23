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
    pub fn with_value(mut self, key: &str, value: Value) -> Self {
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
    pub fn build<T>(self) -> T
    where
        T: DeserializeOwned + Debug,
    {
        todo!()
    }
}
