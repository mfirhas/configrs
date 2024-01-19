// TODO:
// - Refactor codes that use `.unwrap`
// - Refactor procedural error handling to be more "FP"

use serde::de::DeserializeOwned;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::Path;

use super::config_error_impl::ConfigErrorImpl;

#[derive(Clone, Default)]
pub(super) struct ConfigImpl {
    env: serde_json::Map<String, serde_json::Value>,
    prefix: &'static str,
    overwrite: bool,
    err: Option<super::config_error_impl::ConfigErrorImpl>,
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
                err: Some(super::config_error_impl::ConfigErrorImpl::ParseError(
                    String::from("env not passed as key=val format."),
                )),
                ..Default::default()
            };
        }
        let env = env
            .as_object()
            .unwrap_or(&serde_json::Map::new())
            .to_owned();
        Self {
            env,
            ..Default::default()
        }
    }

    pub fn with_value<V>(mut self, key: &str, value: V) -> Self
    where
        V: Into<super::Value> + Debug,
    {
        // check error
        if self.err.is_some() {
            return self;
        }

        // check duplicate if not overwrite
        if !self.overwrite {
            let exist = Self::is_exist(&self.env, key);
            if exist.is_err() {
                self.err = Some(exist.unwrap_err());
                return self;
            }
        }

        self.env
            .insert(key.into(), serde_json::Value::from(value.into()));

        self
    }

    pub fn with_env(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let env_map = env_file_reader::read_file(file_path);
        if env_map.is_err() {
            self.err = Some(super::config_error_impl::ConfigErrorImpl::EnvError(
                env_map.unwrap_err().to_string(),
            ));
            return self;
        }
        let env_map = env_map.unwrap();

        for (key, val) in env_map {
            // check duplicate if not overwrite
            if !self.overwrite {
                let exist = Self::is_exist(&self.env, &key);
                if exist.is_err() {
                    self.err = Some(exist.unwrap_err());
                    return self;
                }
            }
            let value = Self::parse_str(&val);
            self.env.insert(key, value);
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
            self.err = Some(super::config_error_impl::ConfigErrorImpl::FileError(
                file.unwrap_err().to_string(),
            ));
            return self;
        }
        let reader = std::io::BufReader::new(file.unwrap());
        let json_data = serde_json::from_reader::<_, serde_json::Value>(reader);
        if json_data.is_err() {
            self.err = Some(super::config_error_impl::ConfigErrorImpl::JsonError(
                json_data.unwrap_err().to_string(),
            ));
            return self;
        }
        let ret = json_data.map_or_else(
            |err| {
                return Self {
                    err: Some(ConfigErrorImpl::JsonError(err.to_string())),
                    ..Default::default()
                };
            },
            |json_val| {
                let obj = json_val
                    .as_object()
                    .unwrap_or(&serde_json::Map::default())
                    .to_owned();
                for (key, val) in obj {
                    // check duplicate if not overwrite
                    if !self.overwrite {
                        let exist = Self::is_exist(&self.env, &key);
                        if exist.is_err() {
                            self.err = Some(exist.unwrap_err());
                            return self;
                        }
                    }
                    self.env.insert(key, val);
                }
                self
            },
        );
        ret
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
                        err: Some(super::config_error_impl::ConfigErrorImpl::TomlError(
                            err.to_string(),
                        )),
                        ..Default::default()
                    },
                    move |d| {
                        let jsoned = json!(d);
                        let default_json_map = serde_json::Map::new();
                        let map_json = jsoned.as_object().unwrap_or(&default_json_map).to_owned();
                        for (key, val) in map_json {
                            // check duplicate if not overwrite
                            if !self.overwrite {
                                let exist = Self::is_exist(&self.env, &key);
                                if exist.is_err() {
                                    self.err = Some(exist.unwrap_err());
                                    return self;
                                }
                            }
                            self.env.insert(key, val);
                        }
                        self
                    },
                );
                ret
            },
        );

        ret
    }

    pub fn with_yaml(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let ret = Self::load_file_to_string(file_path).map_or_else(
            |err| Self {
                err: Some(err),
                ..Default::default()
            },
            |val| {
                let ret = serde_yaml::from_str::<serde_json::Value>(&val).map_or_else(
                    |err| Self {
                        err: Some(super::config_error_impl::ConfigErrorImpl::YamlError(
                            err.to_string(),
                        )),
                        ..Default::default()
                    },
                    |val| {
                        let default_json_map = serde_json::Map::new();
                        let map_json = val.as_object().unwrap_or(&default_json_map).to_owned();
                        for (k, v) in map_json {
                            // check duplicate if not overwrite
                            if !self.overwrite {
                                let exist = Self::is_exist(&self.env, &k);
                                if exist.is_err() {
                                    self.err = Some(exist.unwrap_err());
                                    return self;
                                }
                            }
                            self.env.insert(k, v);
                        }
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
    //
    //
    //                 .insert(sec, serde_json::Value::Object(sec_map));
    //         } else {
    //             for (k, v) in prop.iter() {
    //                 if let Err(e) = self.is_key_exist(k) {
    //                     self.err = Some(e);
    //                     return self;
    //                 }
    //                 self.env
    //
    //
    //                     .insert(k.to_string(), Self::parse_str(&v));
    //             }
    //         }
    //     }
    //
    //     self
    // }

    /// Build configs into T
    pub fn build<T>(self) -> Result<T, super::config_error_impl::ConfigErrorImpl>
    where
        T: DeserializeOwned + Debug,
    {
        if self.err.is_some() {
            return Err(self.err.unwrap());
        }

        let ret = serde_json::from_value::<T>(serde_json::Value::Object(self.env))?;
        Ok(ret)
    }

    fn is_exist(
        map: &serde_json::Map<String, serde_json::Value>,
        key: &str,
    ) -> Result<(), ConfigErrorImpl> {
        if map.contains_key(key) {
            return Err(ConfigErrorImpl::DuplicateKey);
        }
        Ok(())
    }

    fn load_file_to_string(
        file_path: impl AsRef<Path>,
    ) -> Result<String, super::config_error_impl::ConfigErrorImpl> {
        match std::fs::read_to_string(file_path) {
            Ok(v) => Ok(v),
            Err(e) => Err(super::config_error_impl::ConfigErrorImpl::FileError(
                e.to_string(),
            )),
        }
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
