// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(c_variadic_fn_def)]

pub unsafe extern "C" fn f(_: ...) {} //~ ERROR variadic fn requires at least one nonvariable argument

pub unsafe extern "C" fn g(_: i32, _: ..., _: u8) {} //~ ERROR variable arguments must be at the end of the argument list

pub unsafe fn h(_: i32, _: ...) {} //~ ERROR variadic fn must have `C` ABI

pub extern "C" fn i(_: i32, _: ...) {} //~ ERROR variadic fn must be `unsafe`
