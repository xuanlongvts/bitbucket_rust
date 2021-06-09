#![allow(unused)]
fn main() {
	let decimal = 65.4321_f32;

	// expected `u8`, found `f32`
	// Error! No implicit conversion
	// let integer: u8 = decimal;

    // Explicit conversion
	let integer = decimal as u8;
	let character = integer as char;
	println!("decimal: {}, integer: {}, character: {}", decimal, integer, character);

	// Error! There are limitations in conversion rules. A float cannot be directly converted to a char.
	// invalid cast
	// let character = decimal as char;


	// when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

	// 1000 already fits in a u16
	println!("1000 as a u16 is: {}", 1000 as u16);

	// 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    // println!("1000 as a u8 is : {}", 1000 as u8);

	// -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

	println!("1000 mod 256 is: {}", 1000 % 256);

	println!("128 as a i16 is: {}", 128 as i16);

	// the literal `128` does not fit into the type `i8` whose range is `-128..=127`
	// println!("128 as a i8 is: {}", 128 as i8);

	// println!("1000 as a u8 is : {}", 1000 as u8);
	// println!(" 232 as a i8 is : {}", 232 as i8);


	// Since Rust 1.45, the `as` keyword performs a *saturating cast* when casting from float to int.  
    // If the floating point value exceeds the upper bound or is less than the lower bound, the returned value will be equal to the bound crossed.

	println!("300.0 is {}", 300.0_f32 as u8);
	println!("-100.0 as u8 is {}", -100.0_f32 as u8);
	println!("nan as u8 is {}", f32::NAN as u8);

	println!("========================");
    // This behavior incures a small runtime cost and can be avoided with unsafe methods, however the results might overflow and return **unsound values**. Use these methods wisely:
	unsafe {
		println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
		println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
	    println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
	}
}
