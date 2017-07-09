//! Hex serialization for bigint

use std::{fmt, marker};
use serde::{Serializer, Deserializer};
use serde::de::{Error, Visitor};
use {U128, U256, U512};

pub trait HexPrefixedSerializable {
	fn to_hex(&self) -> String;

	fn from_hex<E>(value: &str) -> Result<Self, E> where Self: Sized, E: Error;
}

macro_rules! impl_hex_prefixed_serializable {
	($($type: ident),+) => (
		$(
			impl HexPrefixedSerializable for $type {
				fn to_hex(&self) -> String {
					"0x".to_owned() + &self.to_hex()
				}

				fn from_hex<E>(value: &str) -> Result<Self, E> where E: Error {
					match value.len() {
						2 if &value[0..2] == "0x" => value[2..].parse().map_err(|e| E::custom(&format!("{:?}", e))),
						_ => Err(E::custom("expected hex prefixed value")),
					}
				}
			}
		)+
	)
}

impl_hex_prefixed_serializable!(U128, U256, U512);

pub fn serialize<U, S>(u: &U, serializer: S) -> Result<S::Ok, S::Error> where U: HexPrefixedSerializable, S: Serializer {
	serializer.serialize_str(&u.to_hex())
}

pub fn deserialize<'de, U, D>(deserializer: D) -> Result<U, D::Error> where U: HexPrefixedSerializable, D: Deserializer<'de> {
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

impl<'de, U> Visitor<'de> for UintVisitor<U> where U: HexPrefixedSerializable {
	type Value = U;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "hex-encoded number")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
		U::from_hex(value)
	}

	fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
		self.visit_str(&value)
	}
}
