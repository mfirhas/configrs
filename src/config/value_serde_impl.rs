// --- START ---
// source: https://github.com/mehcode/config-rs
// MIT & Apache Licensed.
// -------------
// Copyright (c) 2017 Ryan Leckey
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
// -------------
// with some modifications adjusted.
// --- END ---

use super::{config_error_impl::ConfigErrorImpl, ConfigError};
use serde::de::{Deserializer, Visitor};
use std::{
    collections::{HashMap, VecDeque},
    iter::Enumerate,
};

impl<'de> Deserializer<'de> for super::Value {
    type Error = ConfigError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            super::Value::Bool(v) => visitor.visit_bool(v),
            super::Value::Int64(v) => visitor.visit_i64(v),
            super::Value::Float64(v) => visitor.visit_f64(v),
            super::Value::String(v) => visitor.visit_string(v),
            super::Value::Array(v) => visitor.visit_seq(SeqAccess::new(v)),
            super::Value::Map(v) => visitor.visit_map(MapAccess::new(v)),
            super::Value::None => visitor.visit_unit(),
        }
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Bool(v) = &self {
            return visitor.visit_bool(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected bool, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i8(v.to_owned() as i8);
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i8, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i16(v.to_owned() as i16);
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i16, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i32(v.to_owned() as i32);
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i32, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i64(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i64(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i64(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i64(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Int64(v) = &self {
            return visitor.visit_i64(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected i64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Float64(v) = &self {
            return visitor.visit_f32(v.to_owned() as f32);
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected f64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::Float64(v) = &self {
            return visitor.visit_f64(v.to_owned());
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected f64, found {:?}",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
            self
        ))
        .into())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::String(v) = self {
            return visitor.visit_string(v);
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected string",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX
        ))
        .into())
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let super::Value::String(v) = self {
            return visitor.visit_string(v);
        }
        Err(ConfigErrorImpl::SerdeError(format!(
            "{} expected string",
            ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX
        ))
        .into())
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::None => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(EnumAccess {
            value: self,
            name,
            variants,
        })
    }

    serde::forward_to_deserialize_any! {
        char seq
        bytes byte_buf map struct unit
        identifier ignored_any unit_struct tuple_struct tuple
    }
}

// Apache or MIT licensed from https://github.com/mehcode/config-rs
// source: https://github.com/mehcode/config-rs/blob/4457cffb681e9765e364afb24cd28e247f9dc6e5/src/de.rs#L36
struct SeqAccess {
    elements: Enumerate<::std::vec::IntoIter<super::Value>>,
}

impl SeqAccess {
    fn new(elements: Vec<super::Value>) -> Self {
        Self {
            elements: elements.into_iter().enumerate(),
        }
    }
}

impl<'de> serde::de::SeqAccess<'de> for SeqAccess {
    type Error = ConfigError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.elements.next() {
            Some((idx, value)) => seed.deserialize(value).map(Some).map_err(|e| {
                ConfigErrorImpl::SerdeError(format!(
                    "{} failed deserializing array/slice at index {}: {}",
                    ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
                    idx,
                    e
                ))
                .into()
            }),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.elements.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

// Apache or MIT licensed from https://github.com/mehcode/config-rs
// source: https://github.com/mehcode/config-rs/blob/4457cffb681e9765e364afb24cd28e247f9dc6e5/src/de.rs#L227
struct MapAccess {
    elements: VecDeque<(String, super::Value)>,
}

impl MapAccess {
    fn new(table: HashMap<String, super::Value>) -> Self {
        Self {
            elements: table.into_iter().collect(),
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for MapAccess {
    type Error = ConfigError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if let Some(&(ref key_s, _)) = self.elements.front() {
            let key_de = super::Value::String(key_s.into());
            let key = serde::de::DeserializeSeed::deserialize(seed, key_de)?;

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let (key, value) = self.elements.pop_front().unwrap();
        serde::de::DeserializeSeed::deserialize(seed, value).map_err(|e| {
            ConfigErrorImpl::SerdeError(format!(
                "{} failed deserializing map at key {}: {}",
                ConfigErrorImpl::CONFIG_ERROR_IMPL_SERDE_PREFIX,
                &key,
                e
            ))
            .into()
        })
    }
}

// Apache or MIT licensed from https://github.com/mehcode/config-rs
struct StrDeserializer<'a>(&'a str);

impl<'de, 'a> serde::de::Deserializer<'de> for StrDeserializer<'a> {
    type Error = ConfigError;

    #[inline]
    fn deserialize_any<V: serde::de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, ConfigError> {
        visitor.visit_str(self.0)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple option
    }
}

// Apache or MIT licensed from https://github.com/mehcode/config-rs
struct EnumAccess {
    value: super::Value,
    name: &'static str,
    variants: &'static [&'static str],
}

impl EnumAccess {
    fn variant_deserializer(&self, name: &str) -> Result<StrDeserializer, ConfigError> {
        self.variants
            .iter()
            .find(|&&s| s == name)
            .map(|&s| StrDeserializer(s))
            .ok_or_else(|| self.no_constructor_error(name))
    }

    fn table_deserializer(
        &self,
        table: &HashMap<String, super::Value>,
    ) -> Result<StrDeserializer, ConfigError> {
        if table.len() == 1 {
            self.variant_deserializer(table.iter().next().unwrap().0)
        } else {
            Err(self.structural_error())
        }
    }

    fn no_constructor_error(&self, supposed_variant: &str) -> ConfigError {
        ConfigErrorImpl::SerdeError(format!(
            "enum {} does not have variant constructor {}",
            self.name, supposed_variant
        ))
        .into()
    }

    fn structural_error(&self) -> ConfigError {
        ConfigErrorImpl::SerdeError(format!(
            "value of enum {} should be represented by either string or table with exactly one key",
            self.name
        ))
        .into()
    }
}

impl<'de> serde::de::EnumAccess<'de> for EnumAccess {
    type Error = ConfigError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), ConfigError>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let value = {
            let deserializer = match self.value {
                super::Value::String(ref s) => self.variant_deserializer(s),
                super::Value::Map(ref t) => self.table_deserializer(t),
                _ => Err(self.structural_error()),
            }?;
            seed.deserialize(deserializer)?
        };

        Ok((value, self))
    }
}

// Apache or MIT licensed from https://github.com/mehcode/config-rs
impl<'de> serde::de::VariantAccess<'de> for EnumAccess {
    type Error = ConfigError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.value {
            super::Value::Map(t) => seed.deserialize(t.into_iter().next().unwrap().1),
            _ => unreachable!(),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            super::Value::Map(t) => {
                serde::de::Deserializer::deserialize_seq(t.into_iter().next().unwrap().1, visitor)
            }
            _ => unreachable!(),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            super::Value::Map(t) => {
                serde::de::Deserializer::deserialize_map(t.into_iter().next().unwrap().1, visitor)
            }
            _ => unreachable!(),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for super::Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = super::Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("any valid configuration value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value.into())
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value.into())
            }

            #[inline]
            fn visit_i128<E>(self, value: i128) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value > i64::MAX as i128 {
                    return Err(E::custom("i128 is not supported"));
                }
                let val: i64 = value as i64;
                Ok(val.into())
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok((i64::from(value)).into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value > i64::MAX as u64 {
                    return Err(E::custom("u64 is not supported"));
                }
                Ok((value as i64).into())
            }

            #[inline]
            fn visit_u128<E>(self, value: u128) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value > i64::MAX as u128 {
                    return Err(E::custom("u128 is not supported"));
                }
                let val: i64 = value as i64;
                Ok(val.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value.into())
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value.into())
            }

            #[inline]
            fn visit_none<E>(self) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::None)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> ::std::result::Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                serde::de::Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> ::std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::None)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> ::std::result::Result<Self::Value, V::Error>
            where
                V: ::serde::de::SeqAccess<'de>,
            {
                let mut vec: Vec<Self::Value> = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(vec.into())
            }

            fn visit_map<V>(self, mut visitor: V) -> ::std::result::Result<Self::Value, V::Error>
            where
                V: ::serde::de::MapAccess<'de>,
            {
                let mut values: HashMap<String, Self::Value> = HashMap::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values.into())
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
