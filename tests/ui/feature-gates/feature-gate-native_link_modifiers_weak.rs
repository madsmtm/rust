//@ revisions: in_attr in_flag
//@[in_flag] compile-flags: -l framework:+weak=Foo
//@ only-apple framework linkage kind only available on Apple platforms

#[cfg(in_attr)]
#[link(name = "Foo", kind = "framework", modifiers = "+weak")]
//[in_attr]~^ ERROR: linking modifier `weak` is unstable
extern "C" {}

fn main() {}
