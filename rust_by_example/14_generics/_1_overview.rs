// A concrete A
struct A;

// struct single field
struct Single(A);

// Generic
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`.
	let _s = Single(A);

	// Here, `SingleGen` has a type parameter explicitly specified.
	let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
	let _t = SingleGen(A); // Uses `A` defined at the top.
	let _i32 = SingleGen(6); // Uses `i32`
	let _char = SingleGen('b'); // Uses `char`
}