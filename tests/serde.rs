extern crate bigint;

#[cfg(feature="serde")]
extern crate serde;

#[cfg(feature="serde")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature="serde")]
extern crate toml;

#[cfg(feature="serde")]
mod tests {
	use toml;
	use bigint;
	
	#[derive(Serialize, Deserialize)]
	struct TestHex {
		// hex serialization
		#[serde(with = "bigint::serialization::hex")]
		uint128: bigint::U128,
		#[serde(with = "bigint::serialization::hex")]
		uint256: bigint::U256,
		#[serde(with = "bigint::serialization::hex")]
		uint512: bigint::U512,
	}

	#[test]
	fn test_hex() {
		let v = TestHex {
			uint128: 0.into(),
			uint256: 10.into(),
			uint512: 0xfff.into(),
		};

		let expected = 
r#"uint128 = "0"
uint256 = "a"
uint512 = "fff"
"#;

		assert_eq!(expected, toml::to_string(&v).unwrap());
	}

	#[derive(Serialize, Deserialize)]
	struct TestHexPrefixed {
		// hex prefixed serialization
		#[serde(with = "bigint::serialization::hex_prefixed")]
		uint128: bigint::U128,
		#[serde(with = "bigint::serialization::hex_prefixed")]
		uint256: bigint::U256,
		#[serde(with = "bigint::serialization::hex_prefixed")]
		uint512: bigint::U512,
	}

	#[test]
	fn test_hex_prefixed() {
		let v = TestHexPrefixed {
			uint128: 0.into(),
			uint256: 10.into(),
			uint512: 0xfff.into(),
		};

		let expected = 
r#"uint128 = "0x0"
uint256 = "0xa"
uint512 = "0xfff"
"#;

		assert_eq!(expected, toml::to_string(&v).unwrap());
	}

	#[derive(Serialize, Deserialize)]
	struct TestDecimal {	
		// decimal serialization
		#[serde(with = "bigint::serialization::decimal")]
		uint128: bigint::U128,
		#[serde(with = "bigint::serialization::decimal")]
		uint256: bigint::U256,
		#[serde(with = "bigint::serialization::decimal")]
		uint512: bigint::U512,
	}

	#[test]
	fn test_decimal() {
		let v = TestDecimal {
			uint128: 0.into(),
			uint256: 10.into(),
			uint512: 0xfff.into(),
		};

		let expected = 
r#"uint128 = "0"
uint256 = "10"
uint512 = "4095"
"#;

		assert_eq!(expected, toml::to_string(&v).unwrap());
	}
}
