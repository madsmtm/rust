//@ edition: 2021
#![feature(mut_dont_reset_binding_mode_2024)]
#![allow(unused)]
#![forbid(dereferencing_mut_binding)]

struct Foo(u8);

fn main() {
    let Foo(mut a) = &Foo(0);
    //~^ ERROR: dereferencing `mut` binding
    //~| WARN: this changes meaning in Rust 2024
    a = 42;

    let Foo(mut a) = &mut Foo(0);
    //~^ ERROR: dereferencing `mut` binding
    //~| WARN: this changes meaning in Rust 2024
    a = 42;
}
