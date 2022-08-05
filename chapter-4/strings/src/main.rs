fn main() {
    let mut s = String::from("hello");

    s.push_str(" world");

    println!("{}", s);

    int_allocate();
    str_allocate();

    str_clone();
}

fn int_allocate() {
    let x = 5;
    let y = x;

    println!("Both data in x and y are allocated. y: {y}");
}

fn str_allocate() {
    let s1 = String::from("hi");
    let s2 = s1;

    println!("Only data in s1 is allocated, s2 stores a pointer. s2: {s2}");
    println!("s1 is no longer valid, as value has moved to s2.");
    println!("This is to prevent double free error, as at the end of this function scope we need to clean up memory");
}

fn str_clone() {
    let s1 = String::from("hi");
    let s2 = s1.clone();

    // deep copies explicitly
    // and thus pointers are different
    println!("s1 = {s1}, s2 = {s2}");
}
