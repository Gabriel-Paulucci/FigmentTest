use crate::config::Config;

pub mod config;

#[macro_use]
extern crate serde;

fn main() {
    let config: Config = Config::figment().extract().unwrap();
    println!("{:#?}", config);
}
