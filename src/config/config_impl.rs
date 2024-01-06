use serde::de::DeserializeOwned;
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
            // TODO: refactor
            super::Value::None => write!(f, "<NULL>"),
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
                writeln!(f, "File error: {}", v)
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

    pub fn with_env(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let env_map = env_file_reader::read_file(file_path);
        if env_map.is_err() {
            self.err = Some(ConfigErrorImpl::EnvError(env_map.unwrap_err().to_string()));
            return self;
        }
        let env_map = env_map.unwrap();

        for (key, val) in env_map {
            let value = Self::parse_str(&val);
            self.env
                .as_object_mut()
                .unwrap_or(&mut serde_json::Map::new())
                .insert(key, value);
        }

        self
    }

    pub fn with_json(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let file = std::fs::File::open(file_path);
        if file.is_err() {
            self.err = Some(ConfigErrorImpl::FileError(file.unwrap_err().to_string()));
            return self;
        }
        let reader = std::io::BufReader::new(file.unwrap());
        let json_data = serde_json::from_reader::<_, serde_json::Value>(reader);
        if json_data.is_err() {
            self.err = Some(ConfigErrorImpl::JsonError(
                json_data.unwrap_err().to_string(),
            ));
            return self;
        }
        let mut json_data = json_data.unwrap();
        self.env
            .as_object_mut()
            .unwrap_or(&mut serde_json::Map::new())
            .append(
                json_data
                    .as_object_mut()
                    .unwrap_or(&mut serde_json::Map::new()),
            );
        self
    }

    pub fn with_toml(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let ret = Self::load_file_to_string(file_path).map_or_else(
            move |a| Self {
                err: Some(a),
                ..Default::default()
            },
            move |s| {
                let ret = toml::from_str::<toml::Value>(&s).map_or_else(
                    move |err| Self {
                        err: Some(ConfigErrorImpl::TomlError(err.to_string())),
                        ..Default::default()
                    },
                    move |d| {
                        let jsoned = json!(d);
                        let default_json_map = serde_json::Map::new();
                        let map_json = jsoned.as_object().unwrap_or(&default_json_map);
                        self.env
                            .as_object_mut()
                            .unwrap_or(&mut serde_json::Map::new())
                            .extend(map_json.to_owned().into_iter());
                        self
                    },
                );
                ret
            },
        );

        ret
    }

    // support for .ini later.
    // pub fn with_ini(mut self, file_path: impl AsRef<Path>) -> Self {
    //     // check error
    //     if self.err.is_some() {
    //         return self;
    //     }

    //     // read file
    //     let env_file_string = Self::load_file_to_string(file_path);
    //     if env_file_string.is_err() {
    //         self.err = Some(env_file_string.unwrap_err());
    //         return self;
    //     }
    //     let env_file_string = env_file_string.unwrap();

    //     // parse into Ini
    //     let env_values = match ini::Ini::load_from_str(&env_file_string) {
    //         Ok(v) => v,
    //         Err(e) => {
    //             self.err = Some(ConfigErrorImpl::EnvError(e.to_string()));
    //             return self;
    //         }
    //     };

    //     // read
    //     for (sec, prop) in env_values {
    //         if let Some(sec) = sec {
    //             let mut sec_map = serde_json::Map::<String, serde_json::Value>::new();
    //             for (k, v) in prop.iter() {
    //                 if let Err(e) = self.is_key_exist(k) {
    //                     self.err = Some(e);
    //                     return self;
    //                 }
    //                 sec_map.insert(k.to_string(), Self::parse_str(&v));
    //             }
    //             self.env
    //                 .as_object_mut()
    //                 .unwrap_or(&mut serde_json::Map::new())
    //                 .insert(sec, serde_json::Value::Object(sec_map));
    //         } else {
    //             for (k, v) in prop.iter() {
    //                 if let Err(e) = self.is_key_exist(k) {
    //                     self.err = Some(e);
    //                     return self;
    //                 }
    //                 self.env
    //                     .as_object_mut()
    //                     .unwrap_or(&mut serde_json::Map::new())
    //                     .insert(k.to_string(), Self::parse_str(&v));
    //             }
    //         }
    //     }
    //
    //     self
    // }

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

    fn load_file_to_string(file_path: impl AsRef<Path>) -> Result<String, ConfigErrorImpl> {
        match std::fs::read_to_string(file_path) {
            Ok(v) => Ok(v),
            Err(e) => Err(ConfigErrorImpl::FileError(e.to_string())),
        }
    }

    fn is_env_map(&self) -> Result<bool, ConfigErrorImpl> {
        if !self.env.is_object() {
            return Err(ConfigErrorImpl::ParseError(
                "env vars parsing is not in form of key-val object".to_string(),
            ));
        }
        Ok(true)
    }

    fn is_key_exist(&self, key: &str) -> Result<(), ConfigErrorImpl> {
        self.is_env_map()?;
        if let Some(obj) = self.env.as_object() {
            if !self.overwrite && obj.get(key).is_some() {
                return Err(ConfigErrorImpl::JsonError(format!(
                    "config with key: `{}` already exist",
                    key
                )));
            }
        }
        Ok(())
    }

    fn parse_str(v: &str) -> serde_json::Value {
        if let Ok(parsed) = v.parse::<bool>() {
            return json!(parsed);
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
