// test with_value hardcoded config values

use configrs::config;
use serde::{Deserialize, Serialize};

// test config from values
#[test]
fn test_with_value_success() {
    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "STRING_KEY")]
        string: String,

        #[serde(alias = "FLOAT_KEY")]
        float: f64,

        #[serde(alias = "INTEGER_KEY")]
        int: i64,

        #[serde(alias = "ARRAY_KEY")]
        vector: Vec<String>,
    }

    let cfg = config::Config::new()
        .with_value("STRING_KEY", "string value")
        .with_value("FLOAT_KEY", 123.340)
        .with_value("INTEGER_KEY", 3434)
        .with_value(
            "ARRAY_KEY",
            vec!["lksdmf".to_string(), "blabla".to_string()],
        )
        .build::<Cfg>();
    dbg!(&cfg);

    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!("string value", cfg.string);
    assert_eq!(123.340, cfg.float);
    assert_eq!(3434, cfg.int);
    assert_eq!(vec!["lksdmf".to_string(), "blabla".to_string()], cfg.vector);
}

#[test]
fn test_with_value_default_success() {
    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "STRING_KEY", default)]
        string: String,

        #[serde(alias = "FLOAT_KEY")]
        float: f64,

        #[serde(alias = "INTEGER_KEY")]
        int: i64,

        #[serde(alias = "ARRAY_KEY")]
        vector: Vec<String>,
    }

    let cfg = config::Config::new()
        .with_value("FLOAT_KEY", 123.340)
        .with_value("INTEGER_KEY", 3434)
        .with_value(
            "ARRAY_KEY",
            vec!["lksdmf".to_string(), "blabla".to_string()],
        )
        .build::<Cfg>();
    dbg!(&cfg);

    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!("", cfg.string);
    assert_eq!(123.340, cfg.float);
    assert_eq!(3434, cfg.int);
    assert_eq!(vec!["lksdmf".to_string(), "blabla".to_string()], cfg.vector);
}

#[test]
fn test_with_value_custom_default_success() {
    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "STRING_KEY", default = "default_string")]
        string: String,

        #[serde(alias = "FLOAT_KEY")]
        float: f64,

        #[serde(alias = "INTEGER_KEY")]
        int: i64,

        #[serde(alias = "ARRAY_KEY")]
        vector: Vec<String>,
    }

    fn default_string() -> String {
        "im default string".to_string()
    }

    let cfg = config::Config::new()
        .with_value("STRING_KEY", "im default string")
        .with_value("FLOAT_KEY", 123.340)
        .with_value("INTEGER_KEY", 3434)
        .with_value(
            "ARRAY_KEY",
            vec!["lksdmf".to_string(), "blabla".to_string()],
        )
        .build::<Cfg>();
    dbg!(&cfg);

    assert!(cfg.is_ok());
    let cfg = cfg.unwrap();
    assert_eq!("im default string", cfg.string);
    assert_eq!(123.340, cfg.float);
    assert_eq!(3434, cfg.int);
    assert_eq!(vec!["lksdmf".to_string(), "blabla".to_string()], cfg.vector);
}
