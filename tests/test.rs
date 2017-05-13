
#[macro_use]
extern crate err_macros;

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
        let msg: &str = err_macros::msg(&e);
        assert_eq!("Forty two", msg);
    });
    std::panic::catch_unwind(|| {
        panic!("Forty three".to_string());
    }).unwrap_or_else(|e| {
        let msg: &str = err_macros::msg(&e);
        assert_eq!("Forty three", msg);
    });
}
