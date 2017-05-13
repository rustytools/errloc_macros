
#[macro_use]
extern crate errloc_macros;

#[test]
fn test_loc() {
    // note: checks are sensitive to line numbers
    assert_eq!("tests/test.rs:8", errloc!());
    assert_eq!("Hello! (at tests/test.rs:9)", errlocm!("Hello!"));
}

#[test]
fn test_msg() {
    std::panic::catch_unwind(|| {
        panic!("Forty two");
    }).unwrap_or_else(|e| {
        assert_eq!("Forty two", errloc_macros::msg(&e));
    });
    std::panic::catch_unwind(|| {
        panic!("Forty three".to_string());
    }).unwrap_or_else(|e| {
        assert_eq!("Forty three", errloc_macros::msg(&e));
    });
}
