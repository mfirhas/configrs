// test file for loading configuration from toml file

use serde::{Deserialize, Serialize};

use configrs::config::*;

// load all and success
#[test]
fn test_json_success() {
    let file_path = "../tests/data/toml/test.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_string")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_json(file_path).build::<Env>();
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_sub_env.sub_sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
    assert_eq!(
        cfg.sub_env_2.sub_env_2_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
}

// there is missing env key in env file
#[test]
fn test_json_missing_key() {
    let file_path = "../tests/data/toml/test_missing_field.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_string")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_json(file_path).build::<Env>();
    assert!(cfg.is_err());
    let cfg = cfg.unwrap_err();
}

// load with default(trait) value for non-existing env in env file and success
#[test]
fn test_json_default_success() {
    let file_path = "../tests/data/toml/test_missing_field.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_string")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean", default)]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_json(file_path).build::<Env>();
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.arr, vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_sub_env.sub_sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
    assert_eq!(
        cfg.sub_env_2.sub_env_2_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
}

// load with custom default value for non-existing env in env file
#[test]
fn test_json_custom_default_success() {
    let file_path = "../tests/data/toml/test_missing_field.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_string")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean", default = "default_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }
    const fn default_boolean() -> bool {
        true
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_json(file_path).build::<Env>();
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_sub_env.sub_sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
    assert_eq!(
        cfg.sub_env_2.sub_env_2_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
}

// load with different letter case between env file and structs's fields
#[test]
fn test_json_field_case_sensitive_failed() {
    let file_path = "../tests/data/toml/test.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        // #[serde(alias = "env_string")]
        pub env_strIng: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_json(file_path).build::<Env>();
    assert!(cfg.is_err());
}

// load with different letter case between env file and structs's serde alias
#[test]
fn test_json_alias_case_sensitive_failed() {
    let file_path = "../tests/data/toml/test.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_strinG")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_json(file_path).build::<Env>();
    assert!(cfg.is_err());
}

// Load config from env vars and overwrite with file from toml, with overwrite. It should success.
fn test_with_overwrite() {
    std::env::set_var("env_string", "it's string"); // should be overwritten by "string"

    let file_path = "../tests/data/toml/test.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_string")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new()
        .with_overwrite()
        .with_json(file_path)
        .build::<Env>();
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_sub_env.sub_sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
    assert_eq!(
        cfg.sub_env_2.sub_env_2_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );

    std::env::remove_var("env_string");
}

// Load config from env vars and overwrite with file from toml, with overwrite. It should failed.
fn test_without_overwrite() {
    std::env::set_var("env_string", "it's string");

    let file_path = "../tests/data/toml/test.toml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "env_string")]
        pub string: String,
        #[serde(alias = "env_integer")]
        pub integer: i64,
        #[serde(alias = "env_float")]
        pub float: f64,
        #[serde(alias = "env_boolean")]
        pub boolean: bool,
        #[serde(alias = "env_arr")]
        pub arr: Vec<String>,
        #[serde(alias = "sub_env")]
        pub sub_env: SubEnv,
        #[serde(alias = "sub_env_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "sub_env_string")]
        pub sub_env_string: String,
        #[serde(alias = "sub_env_integer")]
        pub sub_env_integer: i64,
        #[serde(alias = "sub_env_float")]
        pub sub_env_float: f64,
        #[serde(alias = "sub_env_boolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "sub_env_arr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "sub_sub_env")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "sub_sub_env_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "sub_sub_env_integer")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "sub_sub_env_float")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "sub_sub_env_boolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "sub_sub_env_arr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "sub_env_2_string")]
        pub sub_env_2_string: String,
        #[serde(alias = "sub_env_2_integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "sub_env_2_float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "sub_env_2_boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "sub_env_2_arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new()
        // .with_overwrite()
        .with_json(file_path)
        .build::<Env>();
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_boolean, true);
    assert_eq!(
        cfg.sub_env.sub_sub_env.sub_sub_env_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
    assert_eq!(
        cfg.sub_env_2.sub_env_2_arr,
        vec!["anu", "nganu", "lskmdf", "lwkef", "lkemrg"]
    );

    std::env::remove_var("env_string");
}
