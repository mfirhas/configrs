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

use crate::config::{ConfigError, Value};

impl From<serde_json::Error> for ConfigError {
    fn from(value: serde_json::Error) -> Self {
        Self::BuildError(value.to_string())
    }
}

#[derive(Debug, Default)]
pub struct ConfigImpl {
    env: serde_json::Value,
    prefix: &'static str,
    overwrite: bool,
    err: Option<ConfigError>,
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
                err: Some(ConfigError::ParseError(String::from(
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
    pub fn build<T>(self) -> Result<T, ConfigError>
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
