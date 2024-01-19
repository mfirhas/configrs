use configrs::config::{Config, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

// test overwriting env var with values and success
#[test]
fn test_overwrite_with_values_success() {
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
        .with_overwrite()
        .with_value("VAL_FLOAT", 2.340)
        .with_value("VAL_ARR", a)
        .with_value("VAL_INT", 409)
        .with_value("VAL_MAP", map.clone())
        .with_value("ENV_STRING", "this value overwrite") // duplicate key
        .build::<Cfg>()
        .expect("test_env_env_values error");
    dbg!(&cfg);

    assert_eq!(cfg.string, "this value overwrite");
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

// test overwriting env var with values and success
#[test]
fn test_overwrite_map_with_values_success() {
    let _lock = TEST_MUTEX.lock().unwrap();

    env::set_var("ENV_STRING", "asd");
    env::set_var("ENV_INT", "4");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_MAP", "this is original map");

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
        #[serde(alias = "ENV_MAP")]
        map: HashMap<String, serde_json::Value>,
    }

    let a: &[&str] = &["anu", "nganu"];
    let map: HashMap<String, serde_json::Value> = HashMap::from([
        ("key_1".to_string(), serde_json::Value::from(1)),
        ("key_2".to_string(), serde_json::Value::from("anu")),
    ]);
    let cfg = Config::new()
        .with_overwrite()
        .with_value("VAL_FLOAT", 2.340)
        .with_value("VAL_ARR", a)
        .with_value("VAL_INT", 409)
        .with_value("ENV_MAP", map.clone()) // duplicate key
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
    env::remove_var("ENV_MAP");
}

// test overwriting env var with values and failed by deactivating .with_overwrite()
#[test]
fn test_overwrite_with_values_failed() {
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
        // .with_overwrite() //
        .with_value("VAL_FLOAT", 2.340)
        .with_value("VAL_ARR", a)
        .with_value("VAL_INT", 409)
        .with_value("VAL_MAP", map.clone())
        .with_value("ENV_STRING", "this value overwrite") // duplicate key
        .build::<Cfg>();
    dbg!(&cfg);

    assert!(cfg.is_err());

    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_BOOL");
}
