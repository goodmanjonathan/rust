// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// check for improper syntax for the `non_exhaustive` attribute

#![feature(non_exhaustive)]

#[non_exhaustive(foo)] //~ ERROR
enum BadEnum1 {
    // FIXME: #[non_exhaustive] is not yet supported on enum variants
    //#[non_exhaustive(foo)] A, //~ ERROR
}

#[non_exhaustive(foo)] //~ ERROR
struct BadStruct1 {}

#[non_exhaustive = "bar"] //~ ERROR
enum BadEnum2 {
    // FIXME: #[non_exhaustive] is not yet supported on enum variants
    //#[non_exhaustive = "bar"] A, //~ ERROR
}

#[non_exhaustive = "bar"] //~ ERROR
struct BadStruct2 {}

fn main() {}
