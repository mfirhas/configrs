use std::any::Any;

mod init;

fn main() {
    let cfg = init::init_config();
    println!("init_config: {:?}", cfg);
    println!("integer: {:?}", init::CONFIG.integer);
    println!("string: {:?}", init::CONFIG.string);
    println!("env key: {:?}", init::CONFIG.key);
    println!("env key 2: {:?}", init::CONFIG.key_2);
}
