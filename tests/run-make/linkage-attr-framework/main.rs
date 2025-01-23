#![cfg_attr(any(weak, both), feature(native_link_modifiers_weak))]

#[cfg_attr(any(link, both), link(name = "CoreFoundation", kind = "framework"))]
#[cfg_attr(any(weak, both), link(name = "CoreFoundation", kind = "framework", modifiers = "+weak"))]
extern "C" {
    fn CFRunLoopGetTypeID() -> core::ffi::c_ulong;
}

fn main() {
    unsafe {
        CFRunLoopGetTypeID();
    }
}
