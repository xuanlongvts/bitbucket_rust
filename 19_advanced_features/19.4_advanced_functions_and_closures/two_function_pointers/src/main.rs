fn main() {
    let list_of_numbers_one = vec![1, 2, 3];
    let list_of_strings_one: Vec<String> =
        list_of_numbers_one.iter().map(|i| i.to_string()).collect();
    println!("list_of_strings_one: {:?}", list_of_strings_one);

    // another way
    let list_of_strings_two: Vec<String> = list_of_numbers_one
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("list_of_strings_two {:?}", list_of_strings_two);

    // another way
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses_three: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
