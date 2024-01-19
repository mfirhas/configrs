use std::any::Any;

mod init;

// run with `TEST_ENV=donat TEST_ENV_2=1000 cargo run --example multi`
fn main() {
    let cfg = init::init_config();
    println!("init_config: {:?}", cfg);
    println!("integer: {:?}", init::CONFIG.integer);
    println!("string: {:?}", init::CONFIG.string);
    println!("env key: {:?}", init::CONFIG.key);
    println!("env key 2: {:?}", init::CONFIG.key_2);
}
