# configrs

Configuration Library in Rust.

Features:
- Load environment variables from OS Env into User Data Types(struct, enum).
- Load and merge configuration data from multiple sources: JSON, YAML, .env file, and TOML
- Interop with [serde](https://serde.rs/) for user's types annotations for deserializing configurations data.
- Filter configs from os env and .env file with prefix(case sensitive).
- Add overwrite mechanism for optional sources when existing config values need to be overwritten by new ones.
- Add configs values from hard-coded values.
- More features soon...

See [example](examples/) for samples.

License: MIT