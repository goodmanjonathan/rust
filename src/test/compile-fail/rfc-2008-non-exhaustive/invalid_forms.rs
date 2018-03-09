// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Check for improper syntax for the `#[non_exhaustive]` attribute.

#![feature(non_exhaustive)]

#[non_exhaustive(foo)] //~ ERROR malformed `#[non_exhaustive]` attribute [E0699]
pub enum E {}

#[non_exhaustive = "bar"] //~ ERROR malformed `#[non_exhaustive]` attribute [E0699]
pub struct S {}

fn main() {}
