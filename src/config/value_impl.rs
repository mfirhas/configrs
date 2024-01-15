// implementations for enum Value

use std::{collections::HashMap, fmt::Display};

// Display
impl Display for super::Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            super::Value::Bool(v) => {
                write!(f, "{}", v)
            }
            super::Value::Int64(v) => {
                write!(f, "{}", v)
            }
            super::Value::Float64(v) => {
                write!(f, "{:?}", v)
            }
            super::Value::String(ref v) => {
                write!(f, "{}", v)
            }
            // TODO: need to adjust trailing extras chars
            super::Value::Array(ref v) => write!(f, "{:?}", {
                v.iter().map(|e| format!("{}, ", e)).collect::<String>()
            }),
            // TODO: need to adjust trailing extras chars
            super::Value::Map(ref v) => write!(f, "{{ {} }}", {
                v.iter()
                    .map(|(k, v)| format!("{} => {}, ", k, v))
                    .collect::<String>()
            }),
            // TODO: refactor
            super::Value::None => write!(f, "<NULL>"),
        }
    }
}

// from basic types
impl From<bool> for super::Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for super::Value {
    fn from(value: i64) -> Self {
        Self::Int64(value)
    }
}

impl From<f64> for super::Value {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl<'a> From<&'a str> for super::Value {
    fn from(value: &'a str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for super::Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<V> From<Vec<V>> for super::Value
where
    V: Into<super::Value>,
{
    fn from(value: Vec<V>) -> Self {
        Self::Array(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<V> From<&[V]> for super::Value
where
    V: Into<super::Value> + Clone,
{
    fn from(value: &[V]) -> Self {
        Self::Array(value.to_vec().into_iter().map(|v| v.into()).collect())
    }
}

impl<V> From<HashMap<String, V>> for super::Value
where
    V: Into<super::Value>,
{
    fn from(value: HashMap<String, V>) -> Self {
        Self::Map(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

// from Value to serde_json::Value
impl From<super::Value> for serde_json::Value {
    fn from(value: super::Value) -> Self {
        match value {
            super::Value::Bool(v) => serde_json::Value::Bool(v),
            super::Value::Int64(v) => serde_json::Value::Number(serde_json::Number::from(v)),
            super::Value::Float64(v) => serde_json::Value::Number(
                serde_json::Number::from_f64(v).unwrap(), // TODO: remove unwrap.
            ),
            super::Value::String(v) => serde_json::Value::String(v),
            super::Value::Array(v) => serde_json::Value::Array(
                v.into_iter()
                    .map(|v| v.into())
                    .collect::<Vec<serde_json::Value>>(),
            ),
            super::Value::Map(v) => serde_json::Value::Object(
                v.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<serde_json::Map<_, serde_json::Value>>(),
            ),
            super::Value::None => serde_json::Value::Null,
        }
    }
}
