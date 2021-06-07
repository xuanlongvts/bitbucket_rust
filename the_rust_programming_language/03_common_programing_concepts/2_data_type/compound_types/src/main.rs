fn main() {
    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {} ", tup.0);

    let tup_2 = (300, 5.1, 10);
    let (x, y, z) = tup_2;
    println!("x, y, z = {}, {}, {}", x, y, z);
    println!("x, y, z = {}, {}, {}", tup_2.0, tup_2.1, tup_2.2);

    // The Array Type
    let arr_a = [1, 2, 3, 4, 5];
    let month = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("arrA 1 = {}", arr_a[0]);
    println!("month any = {}", month[2]);

    let arr_b: [i32; 5] = [6, 7, 8, 9, 10];
    println!("arrB 2 = {}", arr_b[2]);

    let arr_c = [3; 5]; // the sam arr_c = [3, 3, 3, 3, 3]
    println!("arrC 4 = {}", arr_c[4]);
}
