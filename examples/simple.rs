use std::{borrow::Borrow, collections::HashMap, iter::Once};

use configrs::config::{Config, ConfigError};
use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    #[serde(alias = "TEST_ENV")]
    pub string: String,

    #[serde(alias = "TEST_ENV_2")]
    pub integer: i64,
}

lazy_static! {
    pub static ref CONFIG_LAZY_STATIC: Cfg = Config::new()
        .build()
        .expect("error initializing config lazily");
}

lazy_static! {
    pub static ref CONFIG_LAZY_STATIC_ONCE: OnceCell<Cfg> = OnceCell::new();
}

pub static CONFIG_LAZY: Lazy<Cfg> = Lazy::new(|| {
    Config::new()
        .build::<Cfg>()
        .expect("error initializing config lazily...")
});

pub static CONFIG_ONCE: OnceCell<Cfg> = OnceCell::new();

pub static CONFIG_LAZY_ONCE: Lazy<OnceCell<Cfg>> = Lazy::new(|| OnceCell::new());

// run with `TEST_ENV=donat TEST_ENV_2=1000 cargo run --example simple`
fn main() {
    // initialize directly
    let cfg = Config::new()
        .build::<Cfg>()
        .expect("error initializing config");
    println!("config integer: {:?}", cfg.integer);
    println!("config string: {:?}", cfg.string);

    // printing with lazy_static
    let anu = take_and_return_value_lazy_static(&CONFIG_LAZY_STATIC);
    println!("take_and_return_value_lazy_static string: {}", anu);
    take_lazy_static(&CONFIG_LAZY_STATIC);
    let int = CONFIG_LAZY_STATIC.integer;
    let setring = CONFIG_LAZY_STATIC.string.as_str();
    println!("config lazy_static integer: {}", CONFIG_LAZY_STATIC.integer);
    println!("config lazy_static string: {}", CONFIG_LAZY_STATIC.string);

    // initialize with lazy_static + OnceCell
    let cfg = Config::new()
        .build::<Cfg>()
        .expect("error initializing config with lazy_static");
    CONFIG_LAZY_STATIC_ONCE
        .set(cfg)
        .expect("error setting lazy_static oncecell");
    println!(
        "config lazy_static+once integer: {:?}",
        CONFIG_LAZY_STATIC_ONCE.get().unwrap().integer
    );
    println!(
        "config lazy_static+once string: {:?}",
        CONFIG_LAZY_STATIC_ONCE.get().unwrap().string
    );
    println!(
        "config lazy_static+once integer(with init): {:?}",
        CONFIG_LAZY_STATIC_ONCE
            .get_or_init(|| Cfg {
                string: "setring".to_string(),
                integer: 2
            })
            .integer
    );
    println!(
        "config lazy_static+once string(with init): {:?}",
        CONFIG_LAZY_STATIC_ONCE
            .get_or_init(|| Cfg {
                string: "setringggg".to_string(),
                integer: 2
            })
            .string
    );

    // printing initialization with once_cell Lazy
    println!("config lazy integer: {:?}", CONFIG_LAZY.integer);
    println!("config lazy string: {:?}", CONFIG_LAZY.string);

    // initializing with OnceCell
    let once_config = Config::new()
        .build::<Cfg>()
        .expect("error initializing once config");
    CONFIG_ONCE.set(once_config);
    println!(
        "config once integer: {:?}",
        CONFIG_ONCE.get().unwrap().integer
    );
    println!(
        "config once string: {:?}",
        CONFIG_ONCE.get().unwrap().string
    );

    // initializing with Lazy + OnceCell
    let cfg_lazy_once = Config::new()
        .build::<Cfg>()
        .expect("error initializing with OnceCell + Lazy");
    CONFIG_LAZY_ONCE.set(cfg_lazy_once);
    println!(
        "config lazy once integer: {:?}",
        CONFIG_LAZY_ONCE.get().unwrap().integer
    );
    println!(
        "config lazy once string: {:?}",
        CONFIG_LAZY_ONCE.get().unwrap().string
    );

    #[derive(Debug, Deserialize, Serialize)]
    struct Configs {
        #[serde(alias = "ENV_KEY")]
        key: i64,
        #[serde(alias = "ENV_KEY_2")]
        key_2: String,
    }

    let cfg = Config::new();
    let cfg_with_env = cfg.with_env("./examples/multi/.env");
    let config_build = cfg_with_env.build::<Configs>();
    dbg!(&config_build);
}

fn take_lazy_static(cfg: &'static Cfg) {
    println!("take_lazy_static integer: {}", cfg.integer);
    println!("take_lazy_static string: {}", cfg.string);
}

fn take_and_return_value_lazy_static(cfg: &Cfg) -> &str {
    cfg.string.as_str()
}
