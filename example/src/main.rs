use config::{ConfigError, ConfigFile};
use serde::{Deserialize, Serialize};

#[derive(ConfigFile, Debug, Default, Deserialize, Serialize)]
struct ExampleConfig {
    foo: String,
}

fn main() -> Result<(), ConfigError> {
    let mut config = ExampleConfig::load().unwrap_or_default();
    println!("{}", config.foo);

    config.foo = String::from(if config.foo == "bar" { "baz" } else { "bar" });
    config.save()?;
    println!("{}", config.foo);

    Ok(())
}
