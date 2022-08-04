fn main() {
    let number = 3;

    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    if number != 0 {
        println!("Number was non-zero");
    } else if number == 1 {
        println!("Number was one");
    }

    let condition = number < 5;
    let number = if condition { 5 } else { 6 }; // NOTE: shadowing

    println!("Number is finally: {number}");
}
