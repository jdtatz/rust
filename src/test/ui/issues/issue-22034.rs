// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private)]

extern crate libc;

fn main() {
    let ptr: *mut () = 0 as *mut _;
    let _: &mut Fn() = unsafe {
        &mut *(ptr as *mut Fn())
        //~^ ERROR expected a `std::ops::Fn<()>` closure, found `()`
    };
}
