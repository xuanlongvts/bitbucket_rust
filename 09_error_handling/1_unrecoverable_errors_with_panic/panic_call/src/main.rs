fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];

    // can run RUN_BACKTRACE=1 cargo run to list more error
    // RUST_BACKTRACE=full cargo run
}
