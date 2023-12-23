// test file for loading configuration from environment variables

use serde::Deserialize;

use crate::config::*;
use std::env;

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
}

#[test]
fn test_env_vars_missing_field_failed() {}

#[test]
fn test_env_vars_duplicate_field_alias_failed() {}

#[test]
fn test_env_vars_case_sensitivity_fields_failed() {}

#[test]
fn test_env_vars_case_sensitivity_alias_failed() {}

#[test]
fn test_env_vars_default_success() {}

#[test]
fn test_env_vars_default_custom_success() {}

#[test]
fn test_env_vars_default_failed() {}

#[test]
fn test_env_vars_default_custom_failed() {}

// nested

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
