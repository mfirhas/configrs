use std::fs::File;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::Path;

impl From<bool> for super::Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for super::Value {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}

impl From<f64> for super::Value {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl<'a> From<&'a str> for super::Value {
    fn from(value: &'a str) -> Self {
        Self::String(value.into())
    }
}

impl<V> From<Vec<V>> for super::Value
where
    V: Into<super::Value>,
{
    fn from(value: Vec<V>) -> Self {
        Self::Array(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<V> From<HashMap<String, V>> for super::Value
where
    V: Into<super::Value>,
{
    fn from(value: HashMap<String, V>) -> Self {
        Self::Map(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

#[derive(Debug)]
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
                writeln!(f, "Parsing File error: {}", v)
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
                writeln!(f, "Ini/Env parsing error: {}", v)
            }
            ConfigErrorImpl::BuildError(v) => {
                writeln!(f, "Failed building config: {}", v)
            }
        }
    }
}

impl Error for ConfigErrorImpl {}

impl From<serde_json::Error> for ConfigErrorImpl {
    fn from(value: serde_json::Error) -> Self {
        Self::BuildError(value.to_string())
    }
}

#[derive(Debug, Default)]
pub(super) struct ConfigImpl {
    env: serde_json::Value,
    prefix: &'static str,
    overwrite: bool,
    err: Option<ConfigErrorImpl>,
}

impl ConfigImpl {
    pub fn new() -> Self {
        let mut json_map: HashMap<String, serde_json::Value> = HashMap::new();
        for (k, v) in env::vars() {
            json_map.insert(k, Self::parse_str(&v));
        }
        let env = serde_json::json!(json_map);
        if !env.is_object() {
            return Self {
                err: Some(ConfigErrorImpl::ParseError(String::from(
                    "env not passed as key=val format.",
                ))),
                ..Default::default()
            };
        }
        Self {
            env,
            ..Default::default()
        }
    }

    /// Build configs into T
    pub fn build<T>(self) -> Result<T, ConfigErrorImpl>
    where
        T: DeserializeOwned + Debug,
    {
        if self.err.is_some() {
            return Err(self.err.unwrap());
        }

        let ret = serde_json::from_value::<T>(self.env)?;
        Ok(ret)
    }

    fn parse_str(v: &str) -> serde_json::Value {
        if let Ok(parsed) = v.parse::<bool>() {
            return serde_json::Value::Bool(parsed);
        }
        if let Ok(parsed) = v.parse::<i64>() {
            return json!(parsed);
        }
        if let Ok(parsed) = v.parse::<f64>() {
            return json!(parsed);
        }
        serde_json::Value::String(v.to_string())
    }
}
