#![allow(clippy::multiple_crate_versions)]

use derive_config::DeriveYamlConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, DeriveYamlConfig, Deserialize, Serialize)]
struct ExampleConfig {
    foo: String,
}

fn main() {
    let mut config = ExampleConfig::load().unwrap_or_default();
    println!("{}", config.foo);

    config.foo = String::from(if config.foo == "bar" { "baz" } else { "bar" });
    config.save().expect("Failed to save");
    println!("{}", config.foo);
}
