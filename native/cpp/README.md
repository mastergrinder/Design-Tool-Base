# Native C++ modules

Reserved for justified performance-critical code (SIMD geometry, advanced spatial indexes, text shaping) linked into the Rust engine via FFI.

Prefer Rust until a measured hotspot requires C++.
