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

	pub fn main() {
		cc::Build::new()
			.file("./bigint-asm/u256.c")
			.opt_level(3)
			.static_flag(true)
			.compile("u256");
	}
}

#[cfg(not(feature = "cc"))]
mod inner {
	pub fn main() {
	}
}

fn main() {
	inner::main()
}
