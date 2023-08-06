use std::{collections::BTreeMap, env, fmt::Display, fs::File};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use serde_json::Value;

pub trait FromEnv: Sized {
    fn from_env() -> Result<Self, serde_json::Error>;
}

impl<T> FromEnv for T
where
    T: Default + DeserializeOwned,
{
    fn from_env() -> Result<Self, serde_json::Error> {
        let kv = kv_from_dotenv_and_env();
        let value = kv_to_json_value(kv);
        serde_json::from_value(value)
    }
}

/// overrides values from dotenv with env
fn kv_from_dotenv_and_env() -> BTreeMap<String, String> {
    let mut dotenv = kv_from_dotenv();
    let env = kv_from_env();
    for (k, v) in env {
        dotenv.insert(k, v);
    }
    dotenv
}

fn kv_from_dotenv() -> BTreeMap<String, String> {
    let Ok(dotenv) = std::fs::read_to_string(".env") else {
        return Default::default();
    };
    let kv_pairs: BTreeMap<String, String> = dotenv
        .lines()
        .filter_map(|l| {
            let trimmed = l.trim();

            let split_eq: Vec<&str> = trimmed.split('=').collect();
            if trimmed.is_empty() || split_eq.len() != 2 {
                None
            } else {
                let key = split_eq[0].trim();
                let val = split_eq[1].trim();
                if key.is_empty() || val.is_empty() {
                    None
                } else {
                    Some((key.to_string(), val.to_string()))
                }
            }
        })
        .collect();
    kv_pairs
}

fn kv_from_env() -> BTreeMap<String, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut kv: BTreeMap<String, String> = Default::default();
    let mut kcache: Option<String> = None;
    for a in args {
        let k = kcache.take();
        if a.starts_with("--") {
            if let Some(k) = k {
                kv.insert(k, "true".to_string());
            }
            // set key:
            kcache = Some(a[2..].to_string());
        } else {
            if let Some(k) = k {
                kv.insert(k, a);
            } else {
                // ignore values without key
            }
        }
    }
    if let Some(k) = kcache {
        kv.insert(k, "true".to_string());
    }
    kv
}

fn kv_to_json_value(kv: BTreeMap<String, String>) -> Value {
    let mut map = serde_json::Map::new();

    for (k, v) in kv {
        map.insert(k, v_to_json_value(v));
    }

    Value::Object(map)
}

fn v_to_json_value(v: String) -> Value {
    if let Ok(e) = v.parse::<bool>() {
        Value::Bool(e)
    } else if let Ok(e) = v.parse::<u64>() {
        Value::Number(serde_json::Number::from(e))
    } else if let Ok(e) = v.parse::<i64>() {
        Value::Number(serde_json::Number::from(e))
    } else if let Ok(e) = v.parse::<f64>() {
        if let Some(e) = serde_json::Number::from_f64(e) {
            Value::Number(e)
        } else {
            Value::String(v)
        }
    } else {
        Value::String(v)
    }
}
