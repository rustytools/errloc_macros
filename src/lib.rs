//! Utility macros (and a function) to use with `std::panic::catch_unwind`

/// Macro that returns current source file name with a line number.
///
/// This macro returns "filename:linenumber" `&str`.
/// It can be used to include error location information
/// into panic payload like this: `foo().except(errloc!())`
#[macro_export]
macro_rules! errloc {
    () => {
        concat!(file!(), ':', line!())
    }
}

/// Macro that creates a message, concatenating current source file name
/// and a line number with the specified `&str`.
///
/// This macro return "<msg> (at filename:linenumber)" `&str`.
/// It can be used to include error location information
/// into panic payload like this: `panic!(errlocm!("Fatal error"))`
#[macro_export]
macro_rules! errlocm {
    ($e:expr) => {
        concat!($e, " (at ", file!(), ':', line!(), ')')
    }
}

/// This function extracts message from the panic payload, that
/// is obtained from `std::panic::catch_unwind`
///
/// This function extracts `&str` or `std::string::String` message
/// that was provided as a payload to `panic!` macro.
/// Result is returned as a `&str` and has the same lifetime
/// as an input payload.
///
/// Usage example:
///
/// ```
///std::panic::catch_unwind(|| {
///    panic!("Forty two");
///}).unwrap_or_else(|e| {
///    assert_eq!("Forty two", errloc_macros::msg(&e));
///});
/// ```
pub fn msg<'a>(e: &'a std::boxed::Box<std::any::Any + std::marker::Send + 'static>) -> &'a str {
	match e.downcast_ref::<&str>() {
		Some(st) => st,
		None => {
            match e.downcast_ref::<std::string::String>() {
                Some(stw) => stw.as_str(),
                None => "()",
            }
		},
	}
}
