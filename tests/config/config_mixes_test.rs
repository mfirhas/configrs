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

// // use config Value map as config values
// #[test]
// fn test_env_env_values_with_value_map() {
//     let _lock = TEST_MUTEX.lock().unwrap();

//     env::set_var("ENV_STRING", "asd");
//     env::set_var("ENV_INT", "4");
//     env::set_var("ENV_BOOL", "true");

//     #[derive(Debug, Deserialize, Serialize)]
//     struct Cfg {
//         #[serde(alias = "ENV_STRING")]
//         string: String,
//         #[serde(alias = "ENV_INT")]
//         integer: i64,
//         #[serde(alias = "VAL_FLOAT")]
//         float: f64,
//         #[serde(alias = "ENV_BOOL")]
//         boolean: bool,
//         #[serde(alias = "VAL_ARR")]
//         arr: Vec<String>,
//         #[serde(alias = "VAL_INT")]
//         integer_2: u32,
//         #[serde(alias = "VAL_MAP")]
//         map: HashMap<String, serde_json::Value>,
//         #[serde(alias = "VAL_MAP_2")]
//         map_2: HashMap<String, Value>,
//     }

//     let a: &[&str] = &["anu", "nganu"];
//     let map: HashMap<String, serde_json::Value> = HashMap::from([
//         ("key_1".to_string(), serde_json::Value::from(1)),
//         ("key_2".to_string(), serde_json::Value::from("anu")),
//     ]);
//     let map_2: HashMap<String, Value> = HashMap::from([
//         ("key_1".to_string(), Value::Bool(true)),
//         ("key_2".to_string(), Value::String("nganu".to_string())),
//     ]);
//     let cfg = Config::new()
//         .with_value("VAL_FLOAT", 2.340)
//         .with_value("VAL_ARR", a)
//         .with_value("VAL_INT", 409)
//         .with_value("VAL_MAP", map)
//         .with_value("VAL_MAP_2", map_2)
//         .build::<Cfg>()
//         .expect("test_env_env_values error");
//     dbg!(&cfg);

//     assert_eq!(cfg.string, "asd");
//     assert_eq!(cfg.integer, 4);
//     assert_eq!(cfg.float, 2.340);
//     assert_eq!(cfg.boolean, true);
//     assert_eq!(cfg.arr, &["anu", "nganu"]);
//     assert_eq!(cfg.integer_2, 409);

//     env::remove_var("ENV_STRING");
//     env::remove_var("ENV_INT");
//     env::remove_var("ENV_BOOL");
// }
