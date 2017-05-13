Error location utility macros
=============================

This project containst two utility macros and a function
that can make the use of `std::panic::catch_unwind` more convenient.

Plese note, that stack unwinding is not an idiomatic Rust error handling
(but may be useful nonetheless).

Usage example
-------------

```rust
    #[macro_use]
    extern crate errloc_macros;

    std::panic::catch_unwind(|| {
        foo.api_call_that_shoud_not_fail.expect(errloc!());
    }).unwrap_or_else(|e| {
        // output will include the location of "expect" call
        println!("Fatal fail: {:?}", errloc_macros::msg(&e));
    });
```

License information
-------------------

This project is released under the [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0).

Changelog
---------

**2017-05-13**

 * version 0.1.0
 * initial public version
