fn main() {
	fn move_away(s: String) {
		println!("s: {}", s);
	}
	let [john, roa] = ["John".to_string(), "Roa".to_string()];
	move_away(john);
	move_away(roa);
}