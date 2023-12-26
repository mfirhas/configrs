// test file for loading configuration from environment variables

use serde::Deserialize;

use crate::config::*;
use std::env;

/// Load config from environment variables and success
#[test]
fn test_env_vars_success() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(alias = "ENV_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR")]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
}

/// Load from environment variables with missing required value from env.
#[test]
fn test_env_vars_missing_field_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR")]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    // assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);
}

/// Load config with duplicated field in struct from the same env var value
#[test]
fn test_env_vars_duplicate_field_alias_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(alias = "ENV_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR")]
        arr: Vec<i32>,
        #[serde(alias = "ENV_ARR")]
        arr_2: Vec<i32>,
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);
}

/// Load config with different letter case between struct's field and env vars key.
#[test]
fn test_env_vars_case_sensitivity_fields_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        // #[serde(alias = "ENV_STRING")]
        eNv_sTrInG: String, // env var: ENV_STRING
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(alias = "ENV_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR")]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    // assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);
}

/// Load config with different letter case between serde's alias field and env vars key.
#[test]
fn test_env_vars_case_sensitivity_alias_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRiNG")]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(alias = "ENV_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR")]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);
}

/// Load config with some default values(Default trait) with serde default attribute.
#[test]
fn test_env_vars_default_success() {
    // setup
    // env::set_var("ENV_STRING", "anu"); // will use default instead
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    // env::set_var("ENV_ARR", "123,234,345,456"); // will use default instead

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING", default)]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(alias = "ENV_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR", default)]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);

    // teardown
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
}

/// Load config with custom default values
#[test]
fn test_env_vars_default_custom_success() {
    // setup
    // env::set_var("ENV_STRING", "anu"); // will use default instead
    env::set_var("ENV_INT", "123");
    // env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    // env::set_var("ENV_ARR", "123,234,345,456"); // will use default instead

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING", default)]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i64,
        #[serde(default = "default_float")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
        #[serde(alias = "ENV_ARR", default = "default_vector")]
        arr: Vec<i32>,
    }
    fn default_vector() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }
    const fn default_float() -> f64 {
        3.5
    }

    // run
    let cfg = Config::new().build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);

    // teardown
    env::remove_var("ENV_INT");
    env::remove_var("ENV_BOOL");
}

// /// Load config with failed default trait value
// #[test]
// #[ignore = "rustc catch them at compile time"]
// fn test_env_vars_default_failed() {
//     // setup
//     env::set_var("ENV_STRING", "anu");
//     env::set_var("ENV_INT", "123");
//     env::set_var("ENV_FLOAT", "123.0");
//     env::set_var("ENV_BOOL", "true");
//     env::set_var("ENV_ARR", "123,234,345,456");

//     // type
//     #[derive(Debug, Deserialize)]
//     struct Cfg {
//         #[serde(alias = "ENV_STRING")]
//         string: String,
//         #[serde(alias = "ENV_INT")]
//         int: i64,
//         #[serde(alias = "ENV_FLOAT")]
//         float: f64,
//         #[serde(alias = "ENV_BOOL")]
//         boolean: bool,
//         #[serde(alias = "ENV_ARR")]
//         arr: Vec<i32>,
//         #[serde(alias = "ENV_CUSTOM", default)]
//         custom: Custom,
//     }
//     // no default implemented
//     #[derive(Debug, Deserialize)]
//     struct Custom {
//         string: String,
//     }

//     // run
//     let cfg = Config::new().build::<Cfg>();

//     // assert
//     assert!(&cfg.is_ok());
//     let cfg = cfg.unwrap();
//     assert_eq!(cfg.string, "anu");
//     assert_eq!(cfg.boolean, false);
//     assert_eq!(cfg.int, 123);
//     assert_eq!(cfg.float, 123.0);
//     assert_eq!(cfg.arr, vec![123, 234, 345, 456]);

//     // teardown
//     env::remove_var("ENV_STRING");
//     env::remove_var("ENV_INT");
//     env::remove_var("ENV_FLOAT");
//     env::remove_var("ENV_BOOL");
//     env::remove_var("ENV_ARR");
// }

// /// Load config with failed custom default value
// #[test]
// #[ignore = "rustc catch them at compile time"]
// fn test_env_vars_default_custom_failed() {
//     // setup
//     // env::set_var("ENV_STRING", "anu");
//     env::set_var("ENV_INT", "123");
//     env::set_var("ENV_FLOAT", "123.0");
//     env::set_var("ENV_BOOL", "true");
//     // env::set_var("ENV_ARR", "123,234,345,456");

//     // type
//     #[derive(Debug, Deserialize)]
//     struct Cfg {
//         #[serde(alias = "ENV_STRING", default = "default_string")]
//         string: String,
//         #[serde(alias = "ENV_INT")]
//         int: i64,
//         #[serde(alias = "ENV_FLOAT")]
//         float: f64,
//         #[serde(alias = "ENV_BOOL")]
//         boolean: bool,
//         #[serde(alias = "ENV_ARR", default = "default_vector")] // no default_vector exist
//         arr: Vec<i32>,
//     }
//     // return i32 instead of string
//     fn default_string() -> i32 {
//         234
//     }

//     // run
//     let cfg = Config::new().build::<Cfg>();

//     // assert
//     assert!(&cfg.is_ok());
//     let cfg = cfg.unwrap();
//     assert_eq!(cfg.string, "anu");
//     assert_eq!(cfg.boolean, false);
//     assert_eq!(cfg.int, 123);
//     assert_eq!(cfg.float, 123.0);
//     assert_eq!(cfg.arr, vec![123, 234, 345, 456]);

//     // teardown
//     env::remove_var("ENV_STRING");
//     env::remove_var("ENV_INT");
//     env::remove_var("ENV_FLOAT");
//     env::remove_var("ENV_BOOL");
//     env::remove_var("ENV_ARR");
// }

// nested---------------------------------------------------------------------------------------------

#[test]
fn test_env_vars_nested_success() {}

#[test]
fn test_env_vars_missing_field_nested_failed() {}

#[test]
fn test_env_vars_duplicate_field_alias_nested_failed() {}

#[test]
fn test_env_vars_case_sensitivity_fields_nested_failed() {}

#[test]
fn test_env_vars_case_sensitivity_alias_nested_failed() {}

#[test]
fn test_env_vars_default_nested_success() {}

#[test]
fn test_env_vars_default_custom_nested_success() {}

#[test]
fn test_env_vars_default_nested_failed() {}

#[test]
fn test_env_vars_default_custom_nested_failed() {}
