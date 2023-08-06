Please use like this:

```rs
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
