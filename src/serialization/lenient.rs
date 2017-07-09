//! Hex serialization for bigint

use std::{fmt, marker};
use serde::{Serializer, Deserializer};
use serde::de::{Error, Visitor};
use serialization::decimal::DecimalSerializable;
use serialization::hex_prefixed::HexPrefixedSerializable;

pub fn serialize<U, S>(u: &U, serializer: S) -> Result<S::Ok, S::Error> where U: HexPrefixedSerializable, S: Serializer {
	serializer.serialize_str(&u.to_hex())
}

pub fn deserialize<'de, U, D>(deserializer: D) -> Result<U, D::Error> where U: DecimalSerializable + HexPrefixedSerializable + From<u64>, D: Deserializer<'de> {
	deserializer.deserialize_any(UintVisitor::new())
}

struct UintVisitor<U> {
	marker: marker::PhantomData<U>,
}

impl<U> UintVisitor<U> {
	fn new() -> Self {
		UintVisitor {
			marker: marker::PhantomData,
		}
	}
}

impl<'de, U> Visitor<'de> for UintVisitor<U> where U: HexPrefixedSerializable + DecimalSerializable + From<u64> {
	type Value = U;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "hex-encoded number")
	}
	
	fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> where E: Error {
		Ok(U::from(value as u64))
	}

	fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: Error {
		Ok(U::from(value))
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
		U::from_hex(value).or_else(|_: E| U::from_decimal(value))
	}

	fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
		self.visit_str(&value)
	}
}
