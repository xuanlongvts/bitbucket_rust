// https://medium.com/swlh/demystifying-closures-futures-and-async-await-in-rust-part-1-closures-97e531e4dc50
fn receives_closure_1<F>(closure: F) where F: Fn(i32, i32) -> i32 {
        let res = closure(1, 2);
        println!("result: {:?}", res);
    }

fn receives_closure_2<F>(closure: F) where F: Fn(i32) -> i32 {
    let res = closure(1);
    println!("closure(1) => {}", res);
}

fn return_closure() -> impl Fn(i32) -> i32 {
    |x| x + 4
}

fn main() {
    let add1 = |x, y| x + y;
    let result = add1(1, 2);
    println!("result: {:?}", result);
    
    receives_closure_1(add1);

    {
        let add = |x| x + 2;
        receives_closure_2(add);
    }
    {
        let add = |x| x + 3;
        receives_closure_2(add);
    }
    {
        let closure = return_closure();
        println!("return closure: {}", closure(1));
    }
}
