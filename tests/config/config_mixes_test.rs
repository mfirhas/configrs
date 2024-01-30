// test mixing configurations from multiple sources

use configrs::config::{Config, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

// mix env vars and hardcoded configs
#[test]
fn test_env_env_values() {
    let _lock = TEST_MUTEX.lock().unwrap();

    env::set_var("ENV_STRING", "asd");
    env::set_var("ENV_INT", "4");
    env::set_var("ENV_BOOL", "true");

    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        integer: i64,
        #[serde(alias = "VAL_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "VAL_ARR")]
        arr: Vec<String>,
        #[serde(alias = "VAL_INT")]
        integer_2: u32,
        #[serde(alias = "VAL_MAP")]
        map: HashMap<String, serde_json::Value>,
    }

    let a: &[&str] = &["anu", "nganu"];
    let map: HashMap<String, serde_json::Value> = HashMap::from([
        ("key_1".to_string(), serde_json::Value::from(1)),
        ("key_2".to_string(), serde_json::Value::from("anu")),
    ]);
    let cfg = Config::new()
        .with_value("VAL_FLOAT", 2.340)
        .with_value("VAL_ARR", a)
        .with_value("VAL_INT", 409)
        .with_value("VAL_MAP", map)
        .build::<Cfg>()
        .expect("test_env_env_values error");
    dbg!(&cfg);

    assert_eq!(cfg.string, "asd");
    assert_eq!(cfg.integer, 4);
    assert_eq!(cfg.float, 2.340);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, &["anu", "nganu"]);
    assert_eq!(cfg.integer_2, 409);

    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_BOOL");
}

// test by mixing all sources of configs
#[test]
fn test_mix_all() {
    let _lock = TEST_MUTEX.lock().unwrap();

    env::set_var("ENV_STRING", "asd");
    env::set_var("ENV_INT", "4");
    env::set_var("ENV_BOOL", "true");

    let file_path_env = "./tests/data/mixes/test.env";
    let file_path_json = "./tests/data/mixes/test.json";
    let file_path_toml = "./tests/data/mixes/test.toml";
    let file_path_yaml = "./tests/data/mixes/test.yaml";

    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        integer: i64,
        #[serde(alias = "VAL_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "VAL_ARR")]
        arr: Vec<String>,
        #[serde(alias = "VAL_INT")]
        integer_2: u32,
        #[serde(alias = "VAL_MAP")]
        map: HashMap<String, serde_json::Value>,
        // env
        #[serde(flatten)]
        env_env: env_file::Env,

        // json
        #[serde(flatten)]
        json_env: json::Env,

        // toml
        #[serde(flatten)]
        toml_env: toml::Env,

        // yaml
        #[serde(flatten)]
        yaml_env: yaml::Env,
    }

    // env
    mod env_file {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Env {
            #[serde(alias = "ENV_ENV_STRING")]
            pub string: String,
            #[serde(alias = "ENV_ENV_INTEGER")]
            pub integer: i64,
            #[serde(alias = "ENV_ENV_FLOAT")]
            pub float: f64,
            #[serde(alias = "ENV_ENV_BOOLEAN")]
            pub boolean: bool,
            #[serde(flatten)]
            pub env_sub_env: SubEnv,
            #[serde(flatten)]
            pub env_sub_env_2: SubEnv2,
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
    }

    // json
    mod json {
        use serde::{Deserialize, Serialize};
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
            pub json_sub_env: SubEnv,
            #[serde(alias = "subEnv2")]
            pub json_sub_env_2: SubEnv2,
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
    }

    mod toml {
        use serde::{Deserialize, Serialize};
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
            pub toml_sub_env: SubEnv,
            #[serde(alias = "sub_env_2")]
            pub toml_sub_env_2: SubEnv2,
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
    }

    mod yaml {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Env {
            #[serde(alias = "yamlEnvString")]
            pub string: String,
            #[serde(alias = "yamlEnvInteger")]
            pub integer: i64,
            #[serde(alias = "yamlEnvFloat")]
            pub float: f64,
            #[serde(alias = "yamlEnvBoolean")]
            pub boolean: bool,
            #[serde(alias = "yamlEnvArr")]
            pub arr: Vec<String>,
            #[serde(alias = "yamlSubEnv")]
            pub sub_env: SubEnv,
            #[serde(alias = "yamlSubEnv2")]
            pub sub_env_2: SubEnv2,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubEnv {
            #[serde(alias = "yamlSubEnvString")]
            pub sub_env_string: String,
            #[serde(alias = "yamlSubEnvInteger")]
            pub sub_env_integer: i64,
            #[serde(alias = "yamlSubEnvFloat")]
            pub sub_env_float: f64,
            #[serde(alias = "yamlSubEnvBoolean")]
            pub sub_env_boolean: bool,
            #[serde(alias = "yamlSubEnvArr")]
            pub sub_env_arr: Vec<String>,
            #[serde(alias = "yamlSubSubEnv")]
            pub sub_sub_env: SubSubEnv,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubSubEnv {
            #[serde(alias = "yamlSubSubEnvString")]
            pub sub_sub_env_string: String,
            #[serde(alias = "yamlSubSubEnvInteger")]
            pub sub_sub_env_integer: i64,
            #[serde(alias = "yamlSubSubEnvFloat")]
            pub sub_sub_env_float: f64,
            #[serde(alias = "yamlSubSubEnvBoolean")]
            pub sub_sub_env_boolean: bool,
            #[serde(alias = "yamlSubSubEnvArr")]
            pub sub_sub_env_arr: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubEnv2 {
            #[serde(alias = "yamlSubEnv2String")]
            pub sub_env_2_string: String,
            #[serde(alias = "yamlSubEnv2Integer")]
            pub sub_env_2_integer: i64,
            #[serde(alias = "yamlSubEnv2Float")]
            pub sub_env_2_float: f64,
            #[serde(alias = "yamlSubEnv2Boolean")]
            pub sub_env_2_boolean: bool,
            #[serde(alias = "yamlSubEnv2Arr")]
            pub sub_env_2_arr: Vec<String>,
        }
    }

    let a: &[&str] = &["anu", "nganu"];
    let map: HashMap<String, serde_json::Value> = HashMap::from([
        ("key_1".to_string(), serde_json::Value::from(1)),
        ("key_2".to_string(), serde_json::Value::from("anu")),
    ]);
    let cfg = Config::new()
        .with_value("VAL_FLOAT", 2.340)
        .with_value("VAL_ARR", a)
        .with_value("VAL_INT", 409)
        .with_value("VAL_MAP", map.clone())
        .with_env(file_path_env)
        .with_json(file_path_json)
        .with_toml(file_path_toml)
        .with_yaml(file_path_yaml)
        .build::<Cfg>()
        .expect("test_env_env_values error");
    dbg!(&cfg);

    assert_eq!(cfg.string, "asd");
    assert_eq!(cfg.integer, 4);
    assert_eq!(cfg.float, 2.340);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, &["anu", "nganu"]);
    assert_eq!(cfg.integer_2, 409);
    assert_eq!(cfg.map, map);

    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_BOOL");
}

// use config Value map as config values
#[test]
fn test_env_env_values_with_value_map() {
    let _lock = TEST_MUTEX.lock().unwrap();

    env::set_var("ENV_STRING", "asd");
    env::set_var("ENV_INT", "4");
    env::set_var("ENV_BOOL", "true");

    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        integer: i64,
        #[serde(alias = "VAL_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "VAL_ARR")]
        arr: Vec<String>,
        #[serde(alias = "VAL_INT")]
        integer_2: u32,
        #[serde(alias = "VAL_MAP")]
        map: HashMap<String, serde_json::Value>,
        #[serde(alias = "VAL_MAP_2")]
        map_2: HashMap<String, Value>,
    }

    let a: &[&str] = &["anu", "nganu"];
    let map: HashMap<String, serde_json::Value> = HashMap::from([
        ("key_1".to_string(), serde_json::Value::from(1)),
        ("key_2".to_string(), serde_json::Value::from("anu")),
    ]);
    let map_2: HashMap<String, Value> = HashMap::from([
        ("key_1".to_string(), Value::Bool(true)),
        ("key_2".to_string(), Value::String("nganu".to_string())),
    ]);
    let cfg = Config::new()
        .with_value("VAL_FLOAT", 2.340)
        .with_value("VAL_ARR", a)
        .with_value("VAL_INT", 409)
        .with_value("VAL_MAP", map)
        .with_value("VAL_MAP_2", map_2)
        .build::<Cfg>()
        .expect("test_env_env_values error");
    dbg!(&cfg);

    assert_eq!(cfg.string, "asd");
    assert_eq!(cfg.integer, 4);
    assert_eq!(cfg.float, 2.340);
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.arr, &["anu", "nganu"]);
    assert_eq!(cfg.integer_2, 409);

    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_BOOL");
}
