// test env vars with prefix

use serde::Deserialize;

use configrs::config::*;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

// TEST_MUTEX will make sure tests are run synchronized, for env vars access and modify.
// will be poisoned if one of the tests is panicked.
static TEST_MUTEX: Mutex<()> = Mutex::new(());

// test configurations with prefix from env variables
#[test]
fn test_prefix_from_env_vars_success() {
    let _lock = TEST_MUTEX.lock().unwrap();

    env::set_var("ENV_KEY", "string value");
    env::set_var("ENV_INT", "123");
    env::set_var("BOOL", "true");

    #[derive(Debug, Deserialize)]
    struct Env {
        #[serde(alias = "ENV_KEY")]
        string: String,
        #[serde(alias = "ENV_INT")]
        integer: i32,
        #[serde(alias = "BOOL")]
        boolean: Option<bool>,
        #[serde(alias = "ENVFLOAT")]
        float: Option<f64>,
        #[serde(alias = "env_FLOAT")]
        string_2: Option<String>,
    }

    let prefix = "ENV_";
    let cfg = Config::new().with_env_prefix(prefix).build::<Env>();
    dbg!(&cfg);

    assert!(cfg.is_ok());

    let cfg = cfg.unwrap();

    assert_eq!(cfg.string, "string value");
    assert_eq!(cfg.integer, 123);
    assert_eq!(cfg.boolean, None);
    assert_eq!(cfg.float, None);
    assert_eq!(cfg.string_2, None);

    env::remove_var("ENV_KEY");
    env::remove_var("ENV_INT");
    env::remove_var("BOOL");
}

// test configurations with prefix from env vars and .env file
fn test_prefix_from_env_vars_env_success() {}
