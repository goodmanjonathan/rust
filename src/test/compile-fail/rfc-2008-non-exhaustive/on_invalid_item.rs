// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that the `non_exhaustive` attribute may not be used on invalid AST nodes (i.e. anything
// other than a struct or enum definition or, in the future, an enum variant).

#![no_std]
#![feature(non_exhaustive)]
#![feature(global_asm)]
#![feature(trait_alias)]

#![non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
mod m {
    #![non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
}

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
extern crate std;

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
use std as other_std;

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
extern {
    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    fn ffn();
}

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
global_asm!(r#"
    .global foo
  foo:
    jmp baz
"#);

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
macro_rules! m {
    () => {}
}

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
trait Trait {
    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    type T;

    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    const C: usize;
    
    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    fn foo();
}

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
impl Trait for usize {
    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    type T = Self;

    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    const C: usize = 1;
    
    #[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
    fn foo() {}
}

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
type T = i32;

// FIXME: trait aliases are not fully implemented (issue #41517)
//#[non_exhaustive] // attribute should be applied to struct or enum definition [E0698]
//trait OtherTrait = Trait;

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
fn bar() {}

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
const C: i32 = 1;

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
static S: bool = true;

#[non_exhaustive] //~ ERROR attribute should be applied to struct or enum definition [E0698]
union U {
    _x: i32,
}

fn main() {}
