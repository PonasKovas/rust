//@ check-pass
//@ aux-build:helper-attr.rs
//@ edition:2021

// This test checks that helper attributes of a derive proc macro can be used together with
// other built-in derive macros.

#[macro_use]
extern crate helper_attr;

use helper_attr::WithHelperAttr;

#[derive(WithHelperAttr, Debug, Clone, PartialEq)]
struct MyStruct<#[x] 'a, #[x] const A: usize, #[x] B> {
    #[x]
    field: &'a [B; A],
}

fn main() {}
