// test file for loading configuration from json file

use serde::Deserialize;

use crate::config::*;

#[test]
fn test_json_success() {}

#[test]
fn test_json_missing_field_failed() {}

#[test]
fn test_json_duplicate_field_alias_failed() {}

#[test]
fn test_json_case_sensitivity_fields_failed() {}

#[test]
fn test_json_case_sensitivity_alias_failed() {}

#[test]
fn test_json_default_success() {}

#[test]
fn test_json_default_custom_success() {}

#[test]
fn test_json_default_failed() {}

#[test]
fn test_json_default_custom_failed() {}

// nested

#[test]
fn test_json_nested_success() {}

#[test]
fn test_json_missing_field_nested_failed() {}

#[test]
fn test_json_duplicate_field_alias_nested_failed() {}

#[test]
fn test_json_case_sensitivity_fields_nested_failed() {}

#[test]
fn test_json_case_sensitivity_alias_nested_failed() {}

#[test]
fn test_json_default_nested_success() {}

#[test]
fn test_json_default_custom_nested_success() {}

#[test]
fn test_json_default_nested_failed() {}

#[test]
fn test_json_default_custom_nested_failed() {}
