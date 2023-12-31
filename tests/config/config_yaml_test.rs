// test file for loading configuration from yaml file

use serde::{Deserialize, Serialize};

use configrs::config::*;

// load all and success
#[test]
fn test_yaml_success() {
    let file_path = "./tests/data/yaml/test.yaml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "envString")]
        pub string: String,
        #[serde(alias = "envInteger")]
        pub integer: i64,
        #[serde(alias = "envFloat")]
        pub float: f64,
        #[serde(alias = "envBoolean")]
        pub boolean: bool,
        #[serde(alias = "envArr")]
        pub arr: Vec<String>,
        #[serde(alias = "subEnv")]
        pub sub_env: SubEnv,
        #[serde(alias = "subEnv2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "subEnvString")]
        pub sub_env_string: String,
        #[serde(alias = "subEnvInteger")]
        pub sub_env_integer: i64,
        #[serde(alias = "subEnvFloat")]
        pub sub_env_float: f64,
        #[serde(alias = "subEnvBoolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "subEnvArr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "subSubEnv")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "subSubEnvString")]
        pub sub_sub_env_string: String,
        #[serde(alias = "subSubEnvInteger")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "subSubEnvFloat")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "subSubEnvBoolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "subSubEnvArr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "subEnv2String")]
        pub sub_env_2_string: String,
        #[serde(alias = "subEnv2Integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "subEnv2Float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "subEnv2Boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "subEnv2Arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_yaml(file_path).build::<Env>();
    dbg!(&cfg);
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
fn test_yaml_missing_key() {
    let file_path = "./tests/data/yaml/test_missing_field.yaml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "envString")]
        pub string: String,
        #[serde(alias = "envInteger")]
        pub integer: i64,
        #[serde(alias = "envFloat")]
        pub float: f64,
        #[serde(alias = "envBoolean")]
        pub boolean: bool,
        #[serde(alias = "envArr")]
        pub arr: Vec<String>,
        #[serde(alias = "subEnv")]
        pub sub_env: SubEnv,
        #[serde(alias = "subEnv2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "subEnvString")]
        pub sub_env_string: String,
        #[serde(alias = "subEnvInteger")]
        pub sub_env_integer: i64,
        #[serde(alias = "subEnvFloat")]
        pub sub_env_float: f64,
        #[serde(alias = "subEnvBoolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "subEnvArr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "subSubEnv")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "subSubEnvString")]
        pub sub_sub_env_string: String,
        #[serde(alias = "subSubEnvInteger")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "subSubEnvFloat")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "subSubEnvBoolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "subSubEnvArr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "subEnv2String")]
        pub sub_env_2_string: String,
        #[serde(alias = "subEnv2Integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "subEnv2Float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "subEnv2Boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "subEnv2Arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_yaml(file_path).build::<Env>();
    dbg!(&cfg);
    assert!(cfg.is_err());
    let cfg = cfg.unwrap_err();
}

// load with default(trait) value for non-existing env in env file and success
#[test]
fn test_yaml_default_success() {
    let file_path = "./tests/data/yaml/test_missing_field.yaml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "envString")]
        pub string: String,
        #[serde(alias = "envInteger")]
        pub integer: i64,
        #[serde(alias = "envFloat")]
        pub float: f64,
        #[serde(alias = "envBoolean")]
        pub boolean: bool,
        #[serde(alias = "envArr")]
        pub arr: Vec<String>,
        #[serde(alias = "subEnv")]
        pub sub_env: SubEnv,
        #[serde(alias = "subEnv2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "subEnvString")]
        pub sub_env_string: String,
        #[serde(alias = "subEnvInteger")]
        pub sub_env_integer: i64,
        #[serde(alias = "subEnvFloat")]
        pub sub_env_float: f64,
        #[serde(alias = "subEnvBoolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "subEnvArr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "subSubEnv")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "subSubEnvString", default)]
        pub sub_sub_env_string: String,
        #[serde(alias = "subSubEnvInteger")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "subSubEnvFloat")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "subSubEnvBoolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "subSubEnvArr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "subEnv2String")]
        pub sub_env_2_string: String,
        #[serde(alias = "subEnv2Integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "subEnv2Float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "subEnv2Boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "subEnv2Arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_yaml(file_path).build::<Env>();
    dbg!(&cfg);
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
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "");
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
fn test_yaml_custom_default_success() {
    let file_path = "./tests/data/yaml/test_missing_field.yaml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "envString")]
        pub string: String,
        #[serde(alias = "envInteger")]
        pub integer: i64,
        #[serde(alias = "envFloat")]
        pub float: f64,
        #[serde(alias = "envBoolean")]
        pub boolean: bool,
        #[serde(alias = "envArr")]
        pub arr: Vec<String>,
        #[serde(alias = "subEnv")]
        pub sub_env: SubEnv,
        #[serde(alias = "subEnv2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "subEnvString")]
        pub sub_env_string: String,
        #[serde(alias = "subEnvInteger")]
        pub sub_env_integer: i64,
        #[serde(alias = "subEnvFloat")]
        pub sub_env_float: f64,
        #[serde(alias = "subEnvBoolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "subEnvArr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "subSubEnv")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "subSubEnvString", default = "default_string")]
        pub sub_sub_env_string: String,
        #[serde(alias = "subSubEnvInteger")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "subSubEnvFloat")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "subSubEnvBoolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "subSubEnvArr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    fn default_string() -> String {
        "test123".to_string()
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "subEnv2String")]
        pub sub_env_2_string: String,
        #[serde(alias = "subEnv2Integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "subEnv2Float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "subEnv2Boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "subEnv2Arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_yaml(file_path).build::<Env>();
    dbg!(&cfg);
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
    assert_eq!(cfg.sub_env.sub_sub_env.sub_sub_env_string, "test123");
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
fn test_yaml_field_case_sensitive_failed() {
    let file_path = "./tests/data/yaml/test.yaml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        // #[serde(alias = "envString")]
        pub ENV_STRiNG: String,
        #[serde(alias = "envInteger")]
        pub integer: i64,
        #[serde(alias = "envFloat")]
        pub float: f64,
        #[serde(alias = "envBoolean")]
        pub boolean: bool,
        #[serde(alias = "envArr")]
        pub arr: Vec<String>,
        #[serde(alias = "subEnv")]
        pub sub_env: SubEnv,
        #[serde(alias = "subEnv2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "subEnvString")]
        pub sub_env_string: String,
        #[serde(alias = "subEnvInteger")]
        pub sub_env_integer: i64,
        #[serde(alias = "subEnvFloat")]
        pub sub_env_float: f64,
        #[serde(alias = "subEnvBoolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "subEnvArr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "subSubEnv")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "subSubEnvString")]
        pub sub_sub_env_string: String,
        #[serde(alias = "subSubEnvInteger")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "subSubEnvFloat")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "subSubEnvBoolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "subSubEnvArr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "subEnv2String")]
        pub sub_env_2_string: String,
        #[serde(alias = "subEnv2Integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "subEnv2Float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "subEnv2Boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "subEnv2Arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_yaml(file_path).build::<Env>();
    dbg!(&cfg);
    assert!(cfg.is_err());
}

// load with different letter case between env file and structs's serde alias
#[test]
fn test_yaml_alias_case_sensitive_failed() {
    let file_path = "./tests/data/yaml/test.yaml";
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Env {
        #[serde(alias = "eNvString")]
        pub string: String,
        #[serde(alias = "envInteger")]
        pub integer: i64,
        #[serde(alias = "envFloat")]
        pub float: f64,
        #[serde(alias = "envBoolean")]
        pub boolean: bool,
        #[serde(alias = "envArr")]
        pub arr: Vec<String>,
        #[serde(alias = "subEnv")]
        pub sub_env: SubEnv,
        #[serde(alias = "subEnv2")]
        pub sub_env_2: SubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv {
        #[serde(alias = "subEnvString")]
        pub sub_env_string: String,
        #[serde(alias = "subEnvInteger")]
        pub sub_env_integer: i64,
        #[serde(alias = "subEnvFloat")]
        pub sub_env_float: f64,
        #[serde(alias = "subEnvBoolean")]
        pub sub_env_boolean: bool,
        #[serde(alias = "subEnvArr")]
        pub sub_env_arr: Vec<String>,
        #[serde(alias = "subSubEnv")]
        pub sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubSubEnv {
        #[serde(alias = "subSubEnvString")]
        pub sub_sub_env_string: String,
        #[serde(alias = "subSubEnvInteger")]
        pub sub_sub_env_integer: i64,
        #[serde(alias = "subSubEnvFloat")]
        pub sub_sub_env_float: f64,
        #[serde(alias = "subSubEnvBoolean")]
        pub sub_sub_env_boolean: bool,
        #[serde(alias = "subSubEnvArr")]
        pub sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubEnv2 {
        #[serde(alias = "subEnv2String")]
        pub sub_env_2_string: String,
        #[serde(alias = "subEnv2Integer")]
        pub sub_env_2_integer: i64,
        #[serde(alias = "subEnv2Float")]
        pub sub_env_2_float: f64,
        #[serde(alias = "subEnv2Boolean")]
        pub sub_env_2_boolean: bool,
        #[serde(alias = "subEnv2Arr")]
        pub sub_env_2_arr: Vec<String>,
    }

    let cfg = Config::new().with_yaml(file_path).build::<Env>();
    dbg!(&cfg);
    assert!(cfg.is_err());
}
