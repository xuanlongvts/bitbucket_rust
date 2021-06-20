#![allow(unused)]

#[derive(Debug)]
enum Food {
	Apple,
	Carot,
	Potato
}

struct Peeled(Food); // Peeled --> Lột vỏ
struct Chopped(Food); // Chopped --> Băm nhỏ

#[derive(Debug)]
struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`. Otherwise, return the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled> {
	match food {
		Some(value)	=> Some(Peeled(value)),
		None		=> None,
	}
}

// Chopping food. If there isn't any, then return `None`. Otherwise, return the chopped food.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
	match peeled {
		Some(Peeled(food))	=> Some(Chopped(food)),
		None				=> None,
	}
}

// Cooking food. Here, we showcase `map()` instead of `match` for case handling.
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
	chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop, and cook food all in sequence. We chain multiple uses of `map()` to simplify the code.
fn process(food: Option<Food>) -> Option<Cooked> {
	food.map(|f| Peeled(f)).map(|Peeled(f)| Chopped(f)).map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>) {
	match food {
		Some(val)	=> println!("Mmm. I love {:?}", val),
		None		=> println!("Oh no! It wasn't edible."),
	}
}

fn main() {
	let apple	= Some(Food::Apple);
	let carot	= Some(Food::Carot);
	let potato	= None;

	let cooked_apple	= cook(chop(peel(apple)));
	let cooked_carrot	= cook(chop(peel(carot)));

	// Let's try the simpler looking `process()` now.
	let cooked_potato	= process(potato);

	eat(cooked_apple);
	eat(cooked_carrot);
	eat(cooked_potato);
}