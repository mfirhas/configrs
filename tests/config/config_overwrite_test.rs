use configrs::config::{self, Config, ConfigError, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

// test overwriting env var with values and success
#[test]
fn test_overwrite_with_values_success() {
    let _lock = TEST_MUTEX.lock().unwrap();

    unsafe {
        env::set_var("ENV_STRING", "asd");
    }
    unsafe {
        env::set_var("ENV_INT", "4");
    }
    unsafe {
        env::set_var("ENV_BOOL", "true");
    }

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

    unsafe {
        env::remove_var("ENV_STRING");
    }
    unsafe {
        env::remove_var("ENV_INT");
    }
    unsafe {
        env::remove_var("ENV_BOOL");
    }
}

// test overwriting env var with values and success
#[test]
fn test_overwrite_map_with_values_success() {
    let _lock = TEST_MUTEX.lock().unwrap();

    unsafe {
        env::set_var("ENV_STRING", "asd");
    }
    unsafe {
        env::set_var("ENV_INT", "4");
    }
    unsafe {
        env::set_var("ENV_BOOL", "true");
    }
    unsafe {
        env::set_var("ENV_MAP", "this is original map");
    }

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

    unsafe {
        env::remove_var("ENV_STRING");
    }
    unsafe {
        env::remove_var("ENV_INT");
    }
    unsafe {
        env::remove_var("ENV_BOOL");
    }
    unsafe {
        env::remove_var("ENV_MAP");
    }
}

// test overwriting env var with values and failed by deactivating .with_overwrite()
#[test]
fn test_overwrite_with_values_failed() {
    let _lock = TEST_MUTEX.lock().unwrap();

    unsafe {
        env::set_var("ENV_STRING", "asd");
    }
    unsafe {
        env::set_var("ENV_INT", "4");
    }
    unsafe {
        env::set_var("ENV_BOOL", "true");
    }

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

    unsafe {
        env::remove_var("ENV_STRING");
    }
    unsafe {
        env::remove_var("ENV_INT");
    }
    unsafe {
        env::remove_var("ENV_BOOL");
    }
}

#[test]
fn test_overwrite_json_overwrite_off() {
    #[derive(Debug, Serialize, Deserialize)]
    struct SubSubEnv {
        #[serde(rename = "subSubEnvString")]
        sub_sub_env_string: String,
        #[serde(rename = "subSubEnvInteger")]
        sub_sub_env_integer: i32,
        #[serde(rename = "subSubEnvFloat")]
        sub_sub_env_float: f64,
        #[serde(rename = "subSubEnvBoolean")]
        sub_sub_env_boolean: bool,
        #[serde(rename = "subSubEnvArr")]
        sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SubEnv {
        #[serde(rename = "subEnvString")]
        sub_env_string: String,
        #[serde(rename = "subEnvInteger")]
        sub_env_integer: i32,
        #[serde(rename = "subEnvFloat")]
        sub_env_float: f64,
        #[serde(rename = "subEnvBoolean")]
        sub_env_boolean: bool,
        #[serde(rename = "subEnvArr")]
        sub_env_arr: Vec<String>,
        #[serde(rename = "subSubEnv")]
        sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Cfg {
        #[serde(rename = "envString")]
        env_string: String,
        #[serde(rename = "envInteger")]
        env_integer: i32,
        #[serde(rename = "envFloat")]
        env_float: f64,
        #[serde(rename = "envBoolean")]
        env_boolean: bool,
        #[serde(rename = "envArr")]
        env_arr: Vec<String>,
        #[serde(rename = "subEnv")]
        sub_env: SubEnv,
        #[serde(rename = "subEnv2")]
        sub_env2: SubEnv,
    }

    let path1 = "./tests/data/json/test_overwrite_1.json";
    let path2 = "./tests/data/json/test_overwrite_2.json";
    let cfg = Config::new()
        .with_json(path1)
        .with_json(path2)
        .build::<Cfg>();
    dbg!(&cfg);
    assert!(cfg.is_err());
    assert_eq!(
        cfg.err().unwrap().to_string(),
        String::from("[CONFIG][ERROR] Overwrite is false, found key: subEnv2 is duplicated")
    );
}

// this should merge and overwrite subEnv2 in json1 with subEnv2 in json2
#[test]
fn test_overwrite_json_overwrite_on() {
    #[derive(Debug, Serialize, Deserialize)]
    struct SubSubEnv {
        #[serde(rename = "subSubEnvString")]
        sub_sub_env_string: String,
        #[serde(rename = "subSubEnvInteger")]
        sub_sub_env_integer: i32,
        #[serde(rename = "subSubEnvFloat")]
        sub_sub_env_float: f64,
        #[serde(rename = "subSubEnvBoolean")]
        sub_sub_env_boolean: bool,
        #[serde(rename = "subSubEnvArr")]
        sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SubSubEnv2 {
        #[serde(rename = "subSubEnvString2")]
        sub_sub_env_string: String,
        #[serde(rename = "subSubEnvInteger2")]
        sub_sub_env_integer: i32,
        #[serde(rename = "subSubEnvFloat2")]
        sub_sub_env_float: f64,
        #[serde(rename = "subSubEnvBoolean2")]
        sub_sub_env_boolean: bool,
        #[serde(rename = "subSubEnvArr2")]
        sub_sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SubEnv {
        #[serde(rename = "subEnvString")]
        sub_env_string: String,
        #[serde(rename = "subEnvInteger")]
        sub_env_integer: i32,
        #[serde(rename = "subEnvFloat")]
        sub_env_float: f64,
        #[serde(rename = "subEnvBoolean")]
        sub_env_boolean: bool,
        #[serde(rename = "subEnvArr")]
        sub_env_arr: Vec<String>,
        #[serde(rename = "subSubEnv")]
        sub_sub_env: SubSubEnv,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SubEnv2 {
        #[serde(rename = "subEnvString2")]
        sub_env_string: String,
        #[serde(rename = "subEnvInteger2")]
        sub_env_integer: i32,
        #[serde(rename = "subEnvFloat2")]
        sub_env_float: f64,
        #[serde(rename = "subEnvBoolean2")]
        sub_env_boolean: bool,
        #[serde(rename = "subEnvArr2")]
        sub_env_arr: Vec<String>,
        #[serde(rename = "subSubEnv2")]
        sub_sub_env: SubSubEnv2,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SubEnv22 {
        #[serde(rename = "subEnv2String2")]
        sub_env_string: String,
        #[serde(rename = "subEnv2Integer2")]
        sub_env_integer: i32,
        #[serde(rename = "subEnv2Float2")]
        sub_env_float: f64,
        #[serde(rename = "subEnv2Boolean2")]
        sub_env_boolean: bool,
        #[serde(rename = "subEnv2Arr2")]
        sub_env_arr: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Cfg {
        #[serde(rename = "envString")]
        env_string: String,
        #[serde(rename = "envString2")]
        env_string_2: String,
        #[serde(rename = "envInteger")]
        env_integer: i32,
        #[serde(rename = "envInteger2")]
        env_integer_2: i32,
        #[serde(rename = "envFloat")]
        env_float: f64,
        #[serde(rename = "envFloat2")]
        env_float_2: f64,
        #[serde(rename = "envBoolean")]
        env_boolean: bool,
        #[serde(rename = "envBoolean2")]
        env_boolean_2: bool,
        #[serde(rename = "envArr")]
        env_arr: Vec<String>,
        #[serde(rename = "envArr2")]
        env_arr_2: Vec<String>,
        #[serde(rename = "subEnv")]
        sub_env: SubEnv,
        #[serde(rename = "subEnv2")]
        sub_env2: SubEnv2,
        #[serde(rename = "subEnv22")]
        sub_env22: SubEnv22,
    }

    let path1 = "./tests/data/json/test_overwrite_1.json";
    let path2 = "./tests/data/json/test_overwrite_2.json";
    let cfg = Config::new()
        .with_overwrite()
        .with_json(path1)
        .with_json(path2)
        .build::<Cfg>();
    dbg!(&cfg);
}
