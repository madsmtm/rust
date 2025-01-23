#![crate_type = "rlib"]
#![feature(native_link_modifiers_weak)]

#[link(name = "CoreFoundation", kind = "framework", modifiers = "+weak")]
extern "C" {}
