fn main() {

    let result = loop_from(0);

    if result != 20 {
        loop {
            println!("Hello, world!");
        }
    }

    nested();

    countdown(5);

    better_countdown(5);

    print_arr();

    better_print_arr();
}

fn loop_from(n: u32) -> u32 {
    let mut counter = n;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    return result;
}

fn nested() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Final count: {count}");
}

fn countdown(n: u32) {
    let mut count = n;

    while count != 0 {
        println!("{count}");
        count -= 1;
    }

    println!("Lift off! Printed down from {n}");
}

fn better_countdown(n: u32) {
    for count in (1..n+1).rev() {
        println!("{count}");
    }

    println!("Lift off! Printed down from {n}");
}

 
fn print_arr() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("a[{index}] is: {}", a[index]);
        index += 1;
    }
}

fn better_print_arr() {
    let a = [10, 20, 30, 40, 50];

    for elem in a {
        println!("val is: {elem}");
    }
}
