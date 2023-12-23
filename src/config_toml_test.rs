// test file for loading configuration from environment variables

use serde::Deserialize;

use crate::config::*;

#[test]
fn test_toml_success() {}

#[test]
fn test_toml_missing_field_failed() {}

#[test]
fn test_toml_duplicate_field_alias_failed() {}

#[test]
fn test_toml_case_sensitivity_fields_failed() {}

#[test]
fn test_toml_case_sensitivity_alias_failed() {}

#[test]
fn test_toml_default_success() {}

#[test]
fn test_toml_default_custom_success() {}

#[test]
fn test_toml_default_failed() {}

#[test]
fn test_toml_default_custom_failed() {}

// nested

#[test]
fn test_toml_nested_success() {}

#[test]
fn test_toml_missing_field_nested_failed() {}

#[test]
fn test_toml_duplicate_field_alias_nested_failed() {}

#[test]
fn test_toml_case_sensitivity_fields_nested_failed() {}

#[test]
fn test_toml_case_sensitivity_alias_nested_failed() {}

#[test]
fn test_toml_default_nested_success() {}

#[test]
fn test_toml_default_custom_nested_success() {}

#[test]
fn test_toml_default_nested_failed() {}

#[test]
fn test_toml_default_custom_nested_failed() {}
