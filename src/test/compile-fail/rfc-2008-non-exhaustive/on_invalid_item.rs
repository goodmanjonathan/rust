// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that the `non_exhaustive` attribute may not be used on anything other than struct and
// enum definitions and enum variants.

#![no_std]
#![feature(non_exhaustive)]

#![non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
mod m {}

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
extern crate std;

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
use std as other_std;

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
extern {
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    fn foreign();
}

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
trait Trait {
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    type T;

    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    const C: usize;
    
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    fn foo();
}

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
impl Trait for usize {
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    type T = Self;

    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    const C: usize = 1;
    
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    fn foo() {}
}

struct Struct;

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
impl Struct {
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    const C: usize = 1;
    
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    fn foo() {}
}

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
type T = i32;

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
fn bar() {}

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
const C: i32 = 1;

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
static S: bool = true;

#[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
union U {
    _x: i32,
}

fn main() {
    #[non_exhaustive] //~ ERROR cannot apply `non_exhaustive` attribute to this item
    let _ = 0;
}
