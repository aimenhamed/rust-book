fn main() {
    println!("Hello, world!");

    my_func(7, 'a');
    new_scope();
}

fn my_func(x: i32, y: char) {
    println!("Hi :) {x} {y}");
}

fn zero() -> i32 {
    return 0;
}

fn seven() -> i32 {
    7
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn new_scope() {
    let y = {
        let mut x = zero();
        x = x + seven();
        plus_one(x)
    };

    println!("y: {y}");
}
