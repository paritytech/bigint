//! Decimal serialization for bigint

use std::{fmt, marker};
use serde::{Serializer, Deserializer};
use serde::de::{Error, Visitor};
use {U128, U256, U512};

pub trait DecimalSerializable {
	fn to_decimal(&self) -> String;

	fn from_decimal<E>(value: &str) -> Result<Self, E> where Self: Sized, E: Error;
}

macro_rules! impl_decimal_serializable {
	($($type: ident),+) => (
		$(
			impl DecimalSerializable for $type {
				fn to_decimal(&self) -> String {
					self.to_string()
				}

				fn from_decimal<E>(value: &str) -> Result<Self, E> where E: Error {
					Self::from_dec_str(value).map_err(|e| E::custom(&format!("{:?}", e)))
				}
			}
		)+
	)
}

impl_decimal_serializable!(U128, U256, U512);

pub fn serialize<U, S>(u: &U, serializer: S) -> Result<S::Ok, S::Error> where U: DecimalSerializable, S: Serializer {
	serializer.serialize_str(&u.to_decimal())
}

pub fn deserialize<'de, U, D>(deserializer: D) -> Result<U, D::Error> where U: DecimalSerializable, D: Deserializer<'de> {
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

impl<'de, U> Visitor<'de> for UintVisitor<U> where U: DecimalSerializable {
	type Value = U;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "hex-encoded number")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
		U::from_decimal(value)
	}

	fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
		self.visit_str(&value)
	}
}
