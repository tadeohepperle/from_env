# Populate structs from cli args and/or `.env` file

Intended to be used like this:

```rs, no_run
use from_env::FromEnv;
use lazy_static::lazy_static;
use serde::Deserialize;

fn cred_file() -> String {
    "credentials.json".into()
}

fn server_url() -> String {
    "127.0.0.1:8080".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Constants {
    #[serde(default = "cred_file")]
    pub cred_file: String,
    #[serde(default = "server_url")]
    pub server_url: String,
}

lazy_static! {
    pub static ref CONSTANTS: Constants =
        Constants::from_env().expect("Please provide valid args for constants");
}
```

Now you can either provide values for `cred_file` and `server_url` via CLI or .env file, or a mix of both. Any value can be left out.
CLI values override .env files, which in turn override defaults.

### with a `.env` file:

```txt
cred_file = credentials.json
```

### or directly in the CLI:

```txt
cargo run -- --server_url localhost://8080
```

It uses `serde_json` under the hood.
