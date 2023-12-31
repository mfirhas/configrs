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

impl Display for super::Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            super::Value::Bool(v) => {
                write!(f, "{}", v)
            }
            super::Value::Int64(v) => {
                write!(f, "{}", v)
            }
            super::Value::Float64(v) => {
                write!(f, "{:?}", v)
            }
            super::Value::String(ref v) => {
                write!(f, "{}", v)
            }
            // TODO: need to adjust trailing extras chars
            super::Value::Array(ref v) => write!(f, "{:?}", {
                v.iter().map(|e| format!("{}, ", e)).collect::<String>()
            }),
            // TODO: need to adjust trailing extras chars
            super::Value::Map(ref v) => write!(f, "{{ {} }}", {
                v.iter()
                    .map(|(k, v)| format!("{} => {}, ", k, v))
                    .collect::<String>()
            }),
        }
    }
}

#[cfg(test)]
mod enum_value_display_tests {
    use super::super::Value;
    use std::collections::HashMap;

    #[test]
    fn test_value_bool() {
        let input = Value::Bool(true);
        let expected = "true";
        assert_eq!(input.to_string(), expected);
    }

    #[test]
    fn test_value_int64() {
        let input = Value::Int64(123);
        let expected = "123";
        assert_eq!(input.to_string(), expected);
    }

    #[test]
    fn test_value_float64() {
        let input = Value::Float64(123.028);
        let expected = "123.028";
        assert_eq!(input.to_string(), expected);
    }

    #[test]
    fn test_value_string() {
        let input = Value::String("string".to_string());
        let expected = "string";
        assert_eq!(input.to_string(), expected);
    }

    #[test]
    fn test_value_array() {
        let input = Value::Array(vec![
            Value::Int64(1),
            Value::Int64(2),
            Value::Int64(3),
            Value::Int64(4),
        ]);
        let expected = "\"1, 2, 3, 4, \"";
        assert_eq!(input.to_string(), expected);
    }

    #[test]
    fn test_value_map() {
        let input = Value::Map(HashMap::from([("1".to_string(), Value::Float64(123.457))]));
        let expected = "{ 1 => 123.457,  }";
        assert_eq!(input.to_string(), expected);
    }
}

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

#[derive(Debug, PartialEq, Eq)]
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

impl From<ConfigErrorImpl> for super::ConfigError {
    fn from(value: ConfigErrorImpl) -> Self {
        Self {
            config_error_impl: value,
        }
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
