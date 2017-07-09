// Copyright 2015-2017 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Efficient large, fixed-size big integers and hashes.

#![cfg_attr(asm_available, feature(asm))]

extern crate byteorder;
extern crate rustc_hex;

#[cfg(feature="heapsize")]
#[macro_use] 
extern crate heapsize as _heapsize;

#[cfg(feature="heapsize")]
mod heapsize;

#[cfg(feature="serde")]
extern crate serde;

#[cfg(feature="serde")]
pub mod serialization;

mod uint;
pub use uint::{U128, U256, U512};
