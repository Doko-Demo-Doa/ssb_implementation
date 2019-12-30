# v0.4.0
* updated dependencies
* error construction private to crate now
* refactoring of public API
* changed minimal rust version to 1.40.0

# v0.3.1
* updated dependencies
* require rust 1.37+ (updated sources to new rust features)
* disabled serialization feature by default

# v0.3.0
* updated dependencies
* removed logging
* changed degree type from f32 to f64 (compatible to coordinate type and sse2-based frontends)

# v0.2.4
* changed sections names to uppercase (according to SSB specification)

# v0.2.3
* Changed tag 'identity' to 'reset'

# v0.2.2
* Added logging tests
* Reduced heap allocations

# v0.2.1
* Fixed doc example
* Added serialization

# v0.2.0
* Minor performance improvements
* Added clone trait to all structures
* Allow empty texture value
* Save texture url in Ssb structure
* Removed search path from parsing parameters

# v0.1.1
* Updated documentation
* Log debug outputs

# v0.1.0
* First functional-complete version