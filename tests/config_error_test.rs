// test ConfigError

use configrs::config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

// Display test
#[test]
fn test_config_error_to_string() {
    let config_error_prefix = "[CONFIG][ERROR]";

    #[derive(Debug, Deserialize, Serialize)]
    struct Cfg {
        #[serde(alias = "ENV_STRING")]
        string: String,
        #[serde(alias = "ENV_INTEGER")]
        integer: i64,
    }
    let cfg = Config::new().build::<Cfg>();
    assert!(cfg.is_err());
    let cfg = cfg.unwrap_err();
    let config_error_msg = cfg.to_string();
    dbg!(&config_error_msg);
    let config_error_msg_prefix = &config_error_msg[0..15];
    dbg!(&config_error_msg_prefix);
    assert_eq!(config_error_prefix, config_error_msg_prefix)
}
