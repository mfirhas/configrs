use configrs::config::Config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Cfg = Config::new()
        .with_env("./examples/multi/.env")
        .build::<Cfg>()
        .expect("error initializing config lazily");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    #[serde(alias = "TEST_ENV")]
    pub string: String,

    #[serde(alias = "TEST_ENV_2")]
    pub integer: i64,

    #[serde(alias = "ENV_KEY")]
    pub key: i64,

    #[serde(alias = "ENV_KEY_2")]
    pub key_2: String,
}

pub fn init_config() -> Cfg {
    Config::new()
        .with_env("./examples/multi/.env")
        .build::<Cfg>()
        .expect("error init_config")
}
