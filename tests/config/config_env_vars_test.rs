// test file for loading configuration from environment variables
// TODO: refactor this tests into more succinct/simpler,
// option1: https://crates.io/crates/test-case
// option2: https://crates.io/crates/proptest

use serde::Deserialize;

use configrs::config::*;
use std::env;

/// Load config from environment variables and success
#[test]
fn test_env_vars_success() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");

    // type
    #[derive(Debug, Deserialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INT")]
        int: i32,
        #[serde(alias = "ENV_FLOAT")]
        float: f64,
        #[serde(alias = "ENV_BOOL")]
        boolean: bool,
    }

    // run
    let cfg = Config::new().build::<Cfg>();
    dbg!(&cfg);
    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, true);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
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

// nested---------------------------------------------------------------------------------------------

// nested tests, env vars have no nested structures, so nested structs need to be flatten with serde's attribute

/// Load config from environment variables and success
#[test]
fn test_env_vars_nested_success() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
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
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        vec![123, 234, 345, 456]
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

/// Load from environment variables with missing required value from env.
#[test]
fn test_env_vars_nested_missing_field_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        // #[serde(alias = "ENV_FLOAT_NESTED")]
        // float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
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

    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
}

/// Load config with duplicated field in struct from the same env var value
#[test]
fn test_env_vars_nested_duplicate_field_alias_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_BOOL_NESTED")] // duplicated field
        boolean_2: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_STRING_NESTED_NESTED")] // duplicated field
        string_2: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
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
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        vec![123, 234, 345, 456]
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

/// Load config with different letter case between struct's field and env vars key.
#[test]
fn test_env_vars_nested_case_sensitivity_fields_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        // #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        ENV_BOOL_NESTED_NESTeD: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
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
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.ENV_BOOL_NESTED_NESTeD,
        false
    );
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        vec![123, 234, 345, 456]
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

/// Load config with different letter case between serde's alias field and env vars key.
#[test]
fn test_env_vars_nested_case_sensitivity_alias_failed() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOoL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
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
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        vec![123, 234, 345, 456]
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

/// Load config with some default values(Default trait) with serde default attribute.
#[test]
fn test_env_vars_nested_default_success() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED", default)]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED", default)]
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
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        vec![123, 234, 345, 456]
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

/// Load config with custom default values
#[test]
fn test_env_vars_nested_default_custom_success() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED", default = "default_bool")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    const fn default_bool() -> bool {
        true
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED", default = "default_string")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
        arr: Vec<i32>,
    }
    fn default_string() -> String {
        String::from("default_anu")
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
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        vec![123, 234, 345, 456]
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

// Load config from env with additional constant config with overwrite will overwrite config from env
#[test]
fn test_with_overwrite() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new()
        .with_overwrite()
        .with_value("ENV_ARR_NESTED_NESTED", vec![1, 2])
        .build::<Cfg>();

    // assert
    assert!(&cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!(cfg.string, "anu");
    assert_eq!(cfg.boolean, false);
    assert_eq!(cfg.int, 123);
    assert_eq!(cfg.float, 123.0);
    assert_eq!(cfg.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.int, 123);
    assert_eq!(cfg.cfg_nested.float, 123.0);
    assert_eq!(cfg.cfg_nested.arr, vec![123, 234, 345, 456]);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.string, "anu");
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.boolean, false);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.int, 123);
    assert_eq!(cfg.cfg_nested.cfg_nested_nested.float, 123.0);
    assert_eq!(
        cfg.cfg_nested.cfg_nested_nested.arr,
        // vec![123, 234, 345, 456]
        vec![1, 2] // overwritten
    );

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}

// Load config from env with additional constant config without overwrite success will failed to overwrite config from env
#[test]
fn test_without_overwrite() {
    // setup
    env::set_var("ENV_STRING", "anu");
    env::set_var("ENV_INT", "123");
    env::set_var("ENV_FLOAT", "123.0");
    env::set_var("ENV_BOOL", "true");
    env::set_var("ENV_ARR", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED", "anu");
    env::set_var("ENV_INT_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED", "true");
    env::set_var("ENV_ARR_NESTED", "123,234,345,456");
    env::set_var("ENV_STRING_NESTED_NESTED", "anu");
    env::set_var("ENV_INT_NESTED_NESTED", "123");
    env::set_var("ENV_FLOAT_NESTED_NESTED", "123.0");
    env::set_var("ENV_BOOL_NESTED_NESTED", "true");
    env::set_var("ENV_ARR_NESTED_NESTED", "123,234,345,456");

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
        #[serde(flatten)]
        cfg_nested: CfgNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNested {
        #[serde(alias = "ENV_STRING_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED")]
        arr: Vec<i32>,
        #[serde(flatten)]
        cfg_nested_nested: CfgNestedNested,
    }
    #[derive(Debug, Deserialize)]
    struct CfgNestedNested {
        #[serde(alias = "ENV_STRING_NESTED_NESTED")]
        string: String,
        #[serde(alias = "ENV_INT_NESTED_NESTED")]
        int: i64,
        #[serde(alias = "ENV_FLOAT_NESTED_NESTED")]
        float: f64,
        #[serde(alias = "ENV_BOOL_NESTED_NESTED")]
        boolean: bool,
        #[serde(alias = "ENV_ARR_NESTED_NESTED")]
        arr: Vec<i32>,
    }

    // run
    let cfg = Config::new()
        // .with_overwrite() // no overwrite
        .with_value("ENV_ARR_NESTED_NESTED", vec![1, 2])
        .build::<Cfg>();

    // assert
    assert!(&cfg.is_err());

    // teardown
    env::remove_var("ENV_STRING");
    env::remove_var("ENV_INT");
    env::remove_var("ENV_FLOAT");
    env::remove_var("ENV_BOOL");
    env::remove_var("ENV_ARR");
    env::remove_var("ENV_STRING_NESTED");
    env::remove_var("ENV_INT_NESTED");
    env::remove_var("ENV_FLOAT_NESTED");
    env::remove_var("ENV_BOOL_NESTED");
    env::remove_var("ENV_ARR_NESTED");
    env::remove_var("ENV_STRING_NESTED_NESTED");
    env::remove_var("ENV_INT_NESTED_NESTED");
    env::remove_var("ENV_FLOAT_NESTED_NESTED");
    env::remove_var("ENV_BOOL_NESTED_NESTED");
    env::remove_var("ENV_ARR_NESTED_NESTED");
}
