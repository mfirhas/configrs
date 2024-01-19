use configrs::config::{Config, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

// test by mixing all sources of configs
#[test]
fn test_overwrite_env_env_vars() {
    let _lock = TEST_MUTEX.lock().unwrap();

    env::set_var("ENV_STRING", "asd");
    env::set_var("ENV_INT", "4");
    env::set_var("ENV_BOOL", "true");

    let file_path_env = "./tests/data/env/test.env";
    let file_path_json = "./tests/data/json/test.json";
    let file_path_toml = "./tests/data/toml/test.toml";
    let file_path_yaml = "./tests/data/yaml/test.yaml";

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
    }

    // env
    mod env_file {
        use serde::{Deserialize, Serialize};
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
