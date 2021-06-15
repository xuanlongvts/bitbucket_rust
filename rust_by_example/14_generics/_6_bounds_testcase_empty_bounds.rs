struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
	"red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
	"blue"
}

fn main() {
	let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

	// `red()` won't work on a blue jay nor vice versa because of the bounds.
	println!("A cardinal is {}", red(&cardinal));
	println!("A blue jay is {}", blue(&blue_jay));
	// println!("A turkey is {}", red(&_turkey));
}