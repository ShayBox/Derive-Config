# Derive Config

My simple configuration library

```rust
use derive_config::DeriveTomlConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, DeriveTomlConfig, Deserialize, Serialize)]
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
```
