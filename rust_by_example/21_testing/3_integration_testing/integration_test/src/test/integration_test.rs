#[test]
mod common;

fn test_add() {
	common::setup();
	
	assert_eq!(adder::add(2, 3), 5);
}