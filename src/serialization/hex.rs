//! Hex serialization for bigint

use std::{fmt, marker};
use serde::{Serializer, Deserializer};
use serde::de::{Error, Visitor};
use {U128, U256, U512};

pub trait HexSerializable {
	fn to_hex(&self) -> String;

	fn from_hex<E>(value: &str) -> Result<Self, E> where Self: Sized, E: Error;
}

macro_rules! impl_hex_serializable {
	($($type: ident),+) => (
		$(
			impl HexSerializable for $type {
				fn to_hex(&self) -> String {
					self.to_hex()
				}

				fn from_hex<E>(value: &str) -> Result<Self, E> where E: Error {
					value.parse().map_err(|e| E::custom(&format!("{:?}", e)))
				}
			}
		)+
	)
}

impl_hex_serializable!(U128, U256, U512);

pub fn serialize<U, S>(u: &U, serializer: S) -> Result<S::Ok, S::Error> where U: HexSerializable, S: Serializer {
	serializer.serialize_str(&u.to_hex())
}

pub fn deserialize<'de, U, D>(deserializer: D) -> Result<U, D::Error> where U: HexSerializable, D: Deserializer<'de> {
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

impl<'de, U> Visitor<'de> for UintVisitor<U> where U: HexSerializable {
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
