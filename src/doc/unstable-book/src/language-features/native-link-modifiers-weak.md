# `native_link_modifiers_weak`

The tracking issue for this feature is: [#TODO]

[#TODO]: https://github.com/rust-lang/rust/issues/TODO

------------------------

The `native_link_modifiers_weak` feature allows you to use the `weak` modifier. This is only compatible with the `dylib` and `framework` linking kinds.

Enabling this modifier (`+weak`) translates to `-weak-l` / `-weak_framework`, and makes the dynamic linker `dyld` wait with resolving symbols until they're actually used.

This is useful on Apple platforms to optimize startup time in case a library/framework is rarely needed, or when targetting older systems that don't support a given library/framework.

Note that for this to work, the library/framework must still be visible to the static / program linker `ld64`, so if linking to system libraries, your Xcode version must be new enough for the library/framework to be available there.

See [Apple's documentation][apple-doc] for more details.

[apple-doc]:https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPFrameworks/Concepts/WeakLinking.html#//apple_ref/doc/uid/20002378-107026
