/*
 * Copyright 2017, https://github.com/rustytools
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate errloc_macros;

#[test]
fn test_loc() {
    // note: checks are sensitive to line numbers
    assert_eq!("tests/test.rs:23", errloc!());
    assert_eq!("Hello! (at tests/test.rs:24)", errlocm!("Hello!"));
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
