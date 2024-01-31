use std::{collections::HashMap, fmt::Debug, path::Path};

use serde::{de::DeserializeOwned, Serialize};

mod config_error_impl;
mod config_impl;
mod value_impl;
mod value_serde_impl;

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

/// Error for related to configs build.
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigError {
    config_error_impl: config_error_impl::ConfigErrorImpl,
}
impl ConfigError {
    pub(crate) const CONFIG_ERROR_PREFIX: &'static str = "[CONFIG][ERROR]";
}

/// Configuration builder to build your configs.
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
    /// - `i32`/`i64`,
    /// - `f32`/`f64`,
    /// - `bool`,
    /// - `Vector<Value>`,
    /// - `HashMap<String, Value>`
    ///
    pub fn with_value<V>(self, key: &str, value: V) -> Self
    where
        V: Into<Value> + Debug,
    {
        Self {
            config_impl: self.config_impl.with_value(key, value),
        }
    }

    /// Filter configs from environment variables and .env file with prefix
    ///
    ///
    /// It doesn't filter configs from json, toml and yaml.
    pub fn with_env_prefix(self, prefix: &'static str) -> Self {
        Self {
            config_impl: self.config_impl.with_env_prefix(prefix),
        }
    }

    /// Overwrite previous already existing configs keys
    ///
    /// Once called, it activated overwriting for next configs callings.
    ///
    /// Example:
    /// ```rust
    /// use configrs::config::Config;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct Cfg {}
    /// // `.with_env("env_path.env")` will not overwrite what's in env(.new()), but `.with_toml("toml.toml")` will, because `.with_overwrite()` declared before.
    /// let cfg = Config::new().with_env("env_path.env").with_overwrite().with_toml("toml.toml").build::<Cfg>();
    /// ```
    ///
    /// Anything declared after `.with_overwrite` will overwrite anything before it in order of declarations.
    pub fn with_overwrite(self) -> Self {
        Self {
            config_impl: self.config_impl.with_overwrite(),
        }
    }

    /// Add configs from .env file
    ///
    pub fn with_env(self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_env(file_path),
        }
    }

    /// Add configs from .json file
    ///
    pub fn with_json(self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_json(file_path),
        }
    }

    /// Add configs from .toml file
    ///
    pub fn with_toml(self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_toml(file_path),
        }
    }

    /// Add configs from .yaml file
    ///
    pub fn with_yaml(self, file_path: impl AsRef<Path>) -> Self {
        Self {
            config_impl: self.config_impl.with_yaml(file_path),
        }
    }

    /// Build configs into T
    ///
    /// This utilize serde DeserializeOwned type, so T must be implemented/derived the Deserialize and/or Serialize trait from serde.
    ///
    /// Use serde to alias or rename config fields as well as for default values or flattening the structure.
    pub fn build<T>(self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned + Debug,
    {
        Ok(self.config_impl.build::<T>()?)
    }
}
