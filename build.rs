// Copyright 2015-2017 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "cc")]
mod inner {
	extern crate cc;

	macro_rules! redef {
		($name:ident, $identifier:expr) => {
			let $name = (stringify!($name), format!("{}_{}", stringify!($name), $identifier));
		}
	}

	pub fn main() {
		use std::io::Write;

		let identifier = {
			use std::collections::hash_map::DefaultHasher;
			use std::hash::{Hash, Hasher};

			let mut hasher = DefaultHasher::new();
			env!("CARGO_PKG_VERSION").hash(&mut hasher);
			hasher.finish().to_string()
		};

		let mut builder = cc::Build::new();
		builder
			.file("./bigint-asm/u256.c")
			.opt_level(3)
			.static_flag(true);

		redef!(u256mul, identifier);
		redef!(u256add, identifier);

		for def in &[&u256mul, &u256add] {
			builder.define(def.0, Some(def.1.as_ref()));
		}

		let mut out = ::std::fs::File::create(format!(
			"{}/{}",
			::std::env::var("OUT_DIR").unwrap(),
			"/ffi.rs"
		)).unwrap();

    // TODO: Use bindgen?
		write!(
			out,
			r#"
mod inner {{
	#[cfg(all(feature = "asm", target_arch = "x86_64"))]
	extern "C" {{
		// Currently, u256add is slower than the Rust implementation (assumably
		// because of the overhead of calling a function with C calling convention),
		// so we only use `u256mul` for now.

		pub fn {u256mul_out}(first: *const u64, second: *const u64, out: *mut u64) -> u64;
	}}
}}

pub use self::inner::{u256mul_out} as {u256mul};
"#,
			u256mul = u256mul.0,
			u256mul_out = u256mul.1
		).unwrap();

		builder.compile("u256");
	}
}

#[cfg(not(feature = "cc"))]
mod inner {
	pub fn main() {}
}

fn main() {
	inner::main()
}
