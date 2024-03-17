// TODO:
// - Refactor: make illegal states unrepresentable

use serde::de::DeserializeOwned;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;

use super::config_error_impl::ConfigErrorImpl;

#[derive(Clone, Default)]
pub(super) struct ConfigImpl {
    env: serde_json::Map<String, serde_json::Value>, // for env vars, .env, values
    files_env: serde_json::Map<String, serde_json::Value>, // for json, yaml, toml
    prefix: &'static str,
    overwrite: bool,
    err: Option<ConfigErrorImpl>,
}

impl ConfigImpl {
    pub fn new() -> Self {
        let env: serde_json::Map<String, serde_json::Value> = env::vars()
            .into_iter()
            .map(|(key, val)| (key, Self::parse_str(&val)))
            .collect();
        let files_env = serde_json::Map::new();
        Self {
            env,
            files_env,
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
        if !self.overwrite && Self::is_exist(&self.env, key) {
            return Self::make_err(ConfigErrorImpl::DuplicateKey(key.to_string()));
        }

        self.env
            .insert(key.into(), serde_json::Value::from(value.into()));

        self
    }

    pub fn with_overwrite(mut self) -> Self {
        self.overwrite = true;
        self
    }

    pub fn with_env_prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn with_env(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let env_map = env_file_reader::read_file(&file_path).map_or_else(
            |err| {
                return Self::make_err(Self::make_err_open_file(&file_path, err));
            },
            |env_map_iter| {
                for (key, val) in env_map_iter {
                    // check duplicate if not overwrite
                    if !self.overwrite && Self::is_exist(&self.env, &key) {
                        return Self::make_err(ConfigErrorImpl::DuplicateKey(key));
                    }
                    let value = Self::parse_str(&val);
                    self.env.insert(key, value);
                }

                return self;
            },
        );

        env_map
    }

    pub fn with_json(mut self, file_path: impl AsRef<Path>) -> Self {
        // check error
        if self.err.is_some() {
            return self;
        }

        let ret = std::fs::File::open(file_path).map_or_else(
            |err| {
                return Self::make_err(ConfigErrorImpl::FileError(err.to_string()));
            },
            |file| {
                let reader = std::io::BufReader::new(file);
                serde_json::from_reader::<_, serde_json::Value>(reader).map_or_else(
                    |err| {
                        return Self::make_err(ConfigErrorImpl::JsonError(err.to_string()));
                    },
                    |json_value| {
                        if let serde_json::Value::Object(v) = json_value {
                            for (key, val) in v {
                                // check duplicate if not overwrite
                                if !self.overwrite && Self::is_exist(&self.files_env, &key) {
                                    return Self::make_err(ConfigErrorImpl::DuplicateKey(key));
                                }
                                self.files_env.insert(key, val);
                            }
                        }
                        return self;
                    },
                )
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
            move |err| Self::make_err(err),
            move |s| {
                let ret = toml::from_str::<toml::Value>(&s).map_or_else(
                    move |err| Self::make_err(ConfigErrorImpl::TomlError(err.to_string())),
                    move |d| {
                        let jsoned = json!(d);
                        if let serde_json::Value::Object(v) = jsoned {
                            for (key, val) in v {
                                // check duplicate if not overwrite
                                if !self.overwrite && Self::is_exist(&self.files_env, &key) {
                                    return Self::make_err(ConfigErrorImpl::DuplicateKey(key));
                                }
                                self.files_env.insert(key, val);
                            }
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
            |err| Self::make_err(err),
            |val| {
                let ret = serde_yaml::from_str::<serde_json::Value>(&val).map_or_else(
                    |err| Self::make_err(ConfigErrorImpl::YamlError(err.to_string())),
                    |val| {
                        if let serde_json::Value::Object(v) = val {
                            for (key, val) in v {
                                // check duplicate if not overwrite
                                if !self.overwrite && Self::is_exist(&self.files_env, &key) {
                                    return Self::make_err(ConfigErrorImpl::DuplicateKey(key));
                                }
                                self.files_env.insert(key, val);
                            }
                        }
                        self
                    },
                );
                ret
            },
        );

        ret
    }

    /// Build configs into T
    pub fn build<T>(self) -> Result<T, super::config_error_impl::ConfigErrorImpl>
    where
        T: DeserializeOwned + Debug,
    {
        if let Some(err) = self.err {
            return Err(err);
        }

        let config_vals = if !self.prefix.is_empty() {
            self.filter_env_prefix()
        } else {
            self
        };

        if !&config_vals.overwrite {
            let is_env_file_env_duplicate = config_vals
                .env
                .keys()
                .any(|k| config_vals.files_env.contains_key(k));
            if is_env_file_env_duplicate {
                return Err(ConfigErrorImpl::DuplicateKey(String::from(
                    "found duplicate key between env and values with json, toml and yaml ",
                )));
            }
        }

        let config_vals = config_vals.merge_env_files_env();

        let ret = serde_json::from_value::<T>(serde_json::Value::Object(config_vals))?;

        Ok(ret)
    }

    fn is_exist(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> bool {
        if map.contains_key(key) {
            return true;
        }
        false
    }

    fn filter_env_prefix(mut self) -> Self {
        let filtered_env = self
            .env
            .into_iter()
            .filter(|(key, _)| key.starts_with(self.prefix))
            .collect::<serde_json::Map<String, serde_json::Value>>();
        self.env = filtered_env;
        self
    }

    fn merge_env_files_env(mut self) -> serde_json::Map<String, serde_json::Value> {
        self.env.extend(self.files_env.into_iter());
        self.env
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

    // create ConfigImpl containing error, ignoring other fields. Used for discarding existing ConfigImpl in case of error.
    fn make_err(err: ConfigErrorImpl) -> Self {
        Self {
            err: Some(err),
            ..Default::default()
        }
    }

    fn make_err_open_file(file_path: &impl AsRef<Path>, err: impl Error) -> ConfigErrorImpl {
        ConfigErrorImpl::FileError(format!(
            "error opening file {}: {}",
            file_path.as_ref().display(),
            err.to_string()
        ))
    }
}
