// test for enum Value representing data types can be used as value of configs.

use configrs::config::Value;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

// test Display trait impl
#[test]
fn test_value_bool() {
    let input = Value::Bool(true);
    let expected = "true";
    assert_eq!(input.to_string(), expected);
}

#[test]
fn test_value_int64() {
    let input = Value::Int64(123);
    let expected = "123";
    assert_eq!(input.to_string(), expected);
}

#[test]
fn test_value_float64() {
    let input = Value::Float64(123.028);
    let expected = "123.028";
    assert_eq!(input.to_string(), expected);
}

#[test]
fn test_value_string() {
    let input = Value::String("string".to_string());
    let expected = "string";
    assert_eq!(input.to_string(), expected);
}

#[test]
fn test_value_array() {
    let input = Value::Array(vec![
        Value::Int64(1),
        Value::Int64(2),
        Value::Int64(3),
        Value::Int64(4),
    ]);
    let expected = "\"1, 2, 3, 4, \"";
    assert_eq!(input.to_string(), expected);
}

#[test]
fn test_value_map() {
    let input = Value::Map(HashMap::from([("1".to_string(), Value::Float64(123.457))]));
    let expected = "{ 1 => 123.457,  }";
    assert_eq!(input.to_string(), expected);
}

// from basic types into Value
// bool
#[test]
fn test_bool_conversion() {
    let b = true;
    let val: Value = b.into();
    let val_b = if let Value::Bool(v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(Value::Bool(true), val);
    assert_eq!(b, val_b);
}

// i64
#[test]
fn test_i64_conversion() {
    let b = 123;
    let val: Value = b.into();
    let val_b = if let Value::Int64(v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(Value::Int64(123), val);
    assert_eq!(b, val_b);
}

// f64
#[test]
fn test_f64_conversion() {
    let b = 123.00;
    let val: Value = b.into();
    let val_b = if let Value::Float64(v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(Value::Float64(123.00), val);
    assert_eq!(b, val_b);
}

// str
#[test]
fn test_str_conversion() {
    let b = "str";
    let val: Value = b.into();
    let val_b = if let Value::String(ref v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(Value::String("str".to_string()), val);
    assert_eq!(b, val_b);
}

// String
#[test]
fn test_string_conversion() {
    let b = "string".to_string();
    let val: Value = b.into();
    let val_b = if let Value::String(v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!("string", val_b);
}

// Vec<V>
#[test]
fn test_vec_conversion() {
    let b = vec![1, 2, 3];
    let val: Value = b.into();
    let val_b = if let Value::Array(ref v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(
        Value::Array(vec![Value::Int64(1), Value::Int64(2), Value::Int64(3)]),
        val
    );
}

// &[V]
#[test]
fn test_slice_conversion() {
    let b: &[i64] = &[1, 2, 3];
    let val: Value = b.into();
    let val_b = if let Value::Array(ref v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(
        Value::Array(vec![Value::Int64(1), Value::Int64(2), Value::Int64(3)]),
        val
    );
}

#[test]
fn test_map_conversion() {
    let b = HashMap::from([
        ("key1".to_string(), Value::Int64(1)),
        ("key2".to_string(), Value::String("sdf".to_string())),
        ("key3".to_string(), Value::Bool(true)),
    ]);
    let c = b.clone();
    let val: Value = b.into();
    let val_b = if let Value::Map(v) = val {
        v
    } else {
        panic!("conversion failed")
    };

    assert_eq!(c, val_b);
}

// from Value into serde_json::Value
// test the conversion
#[test]
fn test_value_serde_conversion() {
    let bool = Value::Bool(true);
    let string = Value::String("string".to_string());
    let int64 = Value::Int64(123);
    let float64 = Value::Float64(23.30);
    let arr = Value::Array(vec![
        Value::Int64(1),
        Value::Int64(2),
        Value::Int64(3),
        Value::Int64(4),
    ]);
    let map = Value::Map(HashMap::from([
        ("1".to_string(), Value::Int64(1)),
        ("2".to_string(), Value::Float64(2.0)),
        ("3".to_string(), Value::String("4".to_string())),
        ("4".to_string(), Value::Bool(true)),
    ]));

    let serde_bool: serde_json::Value = bool.into();
    let serde_string: serde_json::Value = string.into();
    let serde_number_int64: serde_json::Value = int64.into();
    let serde_number_float64: serde_json::Value = float64.into();
    let serde_arr: serde_json::Value = arr.into();
    let serde_map: serde_json::Value = map.into();
}

// serde tests
#[test]
fn test_serde_deserialization() {
    let json_str = r#"
    {
        "array": [
          1,
          2,
          3
        ],
        "boolean": true,
        "color": "gold",
        "null": null,
        "number": 123,
        "object": {
          "a": "b",
          "c": "d",
          "d": {
            "a": "string",
            "b": 123,
            "c": 123.00,
            "d": true,
            "e": null
          },
          "e": {
            "a": "string",
            "b": 123,
            "c": 123.00,
            "d": true,
            "e": null,
            "f": {
              "a": "string",
              "b": 123,
              "c": 123.00,
              "d": true,
              "e": null
          }
          }
        },
        "string": "Hello World",
        "negativeNumber": -234,
        "negativeFloat": -234.2,
        "arrayMix": [
            123,
            123.0,
            false,
            null,
            "setring",
            [
              123,
              123.0,
              false,
              null,
              "setring"
           ],
           {
              "emptyObj": {
      
              }
           },
           {
            "bla": "blabla",
            "ff": 4.20
           }
        ],
        "emptyObj": {},
        "null": null
      }
    "#;

    let expected_map = HashMap::from([
        ("number".to_string(), Value::Int64(123)),
        ("boolean".to_string(), Value::Bool(true)),
        ("negativeNumber".to_string(), Value::Int64(-234)),
        ("color".to_string(), Value::String("gold".to_string())),
        (
            "array".to_string(),
            Value::Array(vec![Value::Int64(1), Value::Int64(2), Value::Int64(3)]),
        ),
        ("negativeFloat".to_string(), Value::Float64(-234.2)),
        (
            "object".to_string(),
            Value::Map(HashMap::from([
                ("a".to_string(), Value::String(String::from("b"))),
                ("c".to_string(), Value::String(String::from("d"))),
                (
                    "d".to_string(),
                    Value::Map(HashMap::from([
                        ("a".to_string(), Value::String(String::from("string"))),
                        ("b".to_string(), Value::Int64(123)),
                        ("c".to_string(), Value::Float64(123.0)),
                        ("d".to_string(), Value::Bool(true)),
                        ("e".to_string(), Value::None),
                    ])),
                ),
                (
                    "e".to_string(),
                    Value::Map(HashMap::from([
                        ("a".to_string(), Value::String(String::from("string"))),
                        ("b".to_string(), Value::Int64(123)),
                        ("c".to_string(), Value::Float64(123.0)),
                        ("d".to_string(), Value::Bool(true)),
                        ("e".to_string(), Value::None),
                        (
                            "f".to_string(),
                            Value::Map(HashMap::from([
                                ("a".to_string(), Value::String(String::from("string"))),
                                ("b".to_string(), Value::Int64(123)),
                                ("c".to_string(), Value::Float64(123.0)),
                                ("d".to_string(), Value::Bool(true)),
                                ("e".to_string(), Value::None),
                            ])),
                        ),
                    ])),
                ),
            ])),
        ),
        (
            "string".to_string(),
            Value::String(String::from("Hello World")),
        ),
        ("emptyObj".to_string(), Value::Map(HashMap::new())),
        ("null".to_string(), Value::None),
        (
            "arrayMix".to_string(),
            Value::Array(vec![
                Value::Int64(123),
                Value::Float64(123.0),
                Value::Bool(false),
                Value::None,
                Value::String("setring".to_string()),
                Value::Array(vec![
                    Value::Int64(123),
                    Value::Float64(123.0),
                    Value::Bool(false),
                    Value::None,
                    Value::String("setring".to_string()),
                ]),
                Value::Map(HashMap::from([(
                    "emptyObj".to_string(),
                    Value::Map(HashMap::new()),
                )])),
                Value::Map(HashMap::from([
                    ("ff".to_string(), Value::Float64(4.2)),
                    ("bla".to_string(), Value::String(String::from("blabla"))),
                ])),
            ]),
        ),
    ]);

    let json = serde_json::from_str::<HashMap<String, Value>>(json_str);
    assert!(json.is_ok());
    let json = json.unwrap();
    dbg!(&json);
    assert_eq!(expected_map, json);
}
