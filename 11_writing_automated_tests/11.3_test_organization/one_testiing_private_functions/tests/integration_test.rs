use one_testiing_private_functions;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, one_testiing_private_functions::add_two(2));
}

// cargo test --test integration_test for only integration test
