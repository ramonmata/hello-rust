use hello_rust;

mod common;

#[test]
fn it_adds_two_at_integration() {
    common::setup();
    assert_eq!(4, hello_rust::adds_two(2));
}