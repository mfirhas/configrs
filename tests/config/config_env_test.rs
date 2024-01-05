// test file for loading configuration from .env file

use serde::{Deserialize, Serialize};

use configrs::config::*;

// load all and success
#[test]
fn test_env_success() {
    let file_path = "./tests/data/env/test.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRING")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN")]
        pub boolean: bool,
        #[serde(flatten)]
        pub sub_env: SubEnv,
        #[serde(flatten)]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(flatten)]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
    }

    let cfg = Config::new().with_env(file_path).build::<Env>();
    dbg!(&cfg);

    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();

    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
}

// there is missing env key in env file
#[test]
fn test_env_missing_key() {
    let file_path = "./tests/data/env/test_missing_field.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRING")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN")]
        pub boolean: bool,
        #[serde(alias = "SUB_ENV")]
        pub sub_env: SubEnv,
        #[serde(alias = "SUB_ENV_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(alias = "SUB_SUB_ENV")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
    }

    let cfg = Config::new().with_env(file_path).build::<Env>();
    assert!(cfg.is_err());
    let cfg = cfg.unwrap_err();
    dbg!(&cfg);
}

// load with default(trait) value for non-existing env in env file and success
#[test]
fn test_env_default_success() {
    let file_path = "./tests/data/env/test_missing_field.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRING")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN", default)]
        pub boolean: bool,
        #[serde(flatten)]
        pub sub_env: SubEnv,
        #[serde(flatten)]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(flatten)]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
    }

    let cfg = Config::new().with_env(file_path).build::<Env>();
    dbg!(&cfg);
    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "string");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.sub_env.sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_env_boolean, true);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "string");
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_integer, 123);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_float, 123.0);
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_boolean, true);
    assert_eq!(cfg.sub_env_2.sub_env_2_string, "string");
    assert_eq!(cfg.sub_env_2.sub_env_2_integer, 123);
    assert_eq!(cfg.sub_env_2.sub_env_2_float, 123.0);
    assert_eq!(cfg.sub_env_2.sub_env_2_boolean, true);
}

// load with custom default value for non-existing env in env file
#[test]
fn test_env_custom_default_success() {
    let file_path = "../tests/data/env/test_missing_field.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRING")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN", default = "default_boolean")]
        pub boolean: bool,
        #[serde(alias = "ENV_ARR")]
        pub arr: Vec<String>,
        #[serde(alias = "SUB_ENV")]
        pub sub_env: SubEnv,
        #[serde(alias = "SUB_ENV_2")]
        pub sub_env_2: SubEnv2,
    }
    const fn default_boolean() -> bool {
        true
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(alias = "SUB_ENV_ARR")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "SUB_SUB_ENV")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "SUB_SUB_ENV_ARR")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "SUB_ENV_2_ARR")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_env(file_path).build::<Env>();
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
fn test_env_field_case_sensitive_failed() {
    let file_path = "../tests/data/env/test.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        // #[serde(alias = "ENV_STRING")]
        pub ENV_STRiNG: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN")]
        pub boolean: bool,
        #[serde(alias = "ENV_ARR")]
        pub arr: Vec<String>,
        #[serde(alias = "SUB_ENV")]
        pub sub_env: SubEnv,
        #[serde(alias = "SUB_ENV_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(alias = "SUB_ENV_ARR")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "SUB_SUB_ENV")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "SUB_SUB_ENV_ARR")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "SUB_ENV_2_ARR")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_env(file_path).build::<Env>();
    assert!(cfg.is_err());
}

// load with different letter case between env file and structs's serde alias
#[test]
fn test_env_alias_case_sensitive_failed() {
    let file_path = "../tests/data/env/test.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRiNG")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN")]
        pub boolean: bool,
        #[serde(alias = "ENV_ARR")]
        pub arr: Vec<String>,
        #[serde(alias = "SUB_ENV")]
        pub sub_env: SubEnv,
        #[serde(alias = "SUB_ENV_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(alias = "SUB_ENV_ARR")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "SUB_SUB_ENV")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "SUB_SUB_ENV_ARR")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "SUB_ENV_2_ARR")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_env(file_path).build::<Env>();
    assert!(cfg.is_err());
}

// Load config from env vars then overwritten by .env file with overwrite, should success
#[test]
fn test_with_overwrite() {
    std::env::set_var("SUB_ENV_FLOAT", "234.3450"); // will be overwritten by 123.0 from test.env

    let file_path = "../tests/data/env/test.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRING")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN")]
        pub boolean: bool,
        #[serde(alias = "ENV_ARR")]
        pub arr: Vec<String>,
        #[serde(alias = "SUB_ENV")]
        pub sub_env: SubEnv,
        #[serde(alias = "SUB_ENV_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(alias = "SUB_ENV_ARR")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "SUB_SUB_ENV")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "SUB_SUB_ENV_ARR")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "SUB_ENV_2_ARR")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new()
        .with_overwrite()
        .with_env(file_path)
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

    std::env::remove_var("SUB_ENV_FLOAT");
}

// Load config from env vars then overwritten by .env file without overwrite, should failed
#[test]
fn test_without_overwrite() {
    std::env::set_var("SUB_ENV_FLOAT", "234.3450");

    let file_path = "../tests/data/env/test.env";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "ENV_STRING")]
        pub string: String,
        #[serde(alias = "ENV_INTEGER")]
        pub integer: i64,
        #[serde(alias = "ENV_FLOAT")]
        pub float: f64,
        #[serde(alias = "ENV_BOOLEAN")]
        pub boolean: bool,
        #[serde(alias = "ENV_ARR")]
        pub arr: Vec<String>,
        #[serde(alias = "SUB_ENV")]
        pub sub_env: SubEnv,
        #[serde(alias = "SUB_ENV_2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "SUB_ENV_STRING")]
        pub sub_env_string: String,
        #[serde(alias = "SUB_ENV_INTEGER")]
        pub sub_env_integer: i64,
        #[serde(alias = "SUB_ENV_FLOAT")]
        pub sub_env_float: f64,
        #[serde(alias = "SUB_ENV_BOOLEAN")]
        pub sub_env_boolean: bool,
        #[serde(alias = "SUB_ENV_ARR")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "SUB_SUB_ENV")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "SUB_SUB_ENV_STRING")]
        pub sub_sub_env_string: String,
        #[serde(alias = "SUB_SUB_ENV_INTEGER")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "SUB_SUB_ENV_FLOAT")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "SUB_SUB_ENV_BOOLEAN")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "SUB_SUB_ENV_ARR")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "SUB_ENV_2_STRING")]
        pub sub_env_2_string: String,
        #[serde(alias = "SUB_ENV_2_INTEGER")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "SUB_ENV_2_FLOAT")]
        pub sub_env_2_float: f64,
        #[serde(alias = "SUB_ENV_2_BOOLEAN")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "SUB_ENV_2_ARR")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new()
        // .with_overwrite()
        .with_env(file_path)
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

    std::env::remove_var("SUB_ENV_FLOAT");
}
