fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function
                       // ... and is no longer valid here
    // cannot use s after this as its dropped in function

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into function
                  // but i32 is Copy, so it's okay to still
                  // use x afterwards
    println!("{x}");

    let s1 = gives_ownership();
    println!("{s1}");

    let s2 = String::from("hiii");

    let s3 = takes_and_gives_back(s2); // s2 ownership is passed to function
                                       // and then returned back into s3
                                       // thus can be used afterwards
    println!("{s3}");

    let (mut s4, len) = get_length(s3);
    println!("The length of '{}' is {}.", s4, len);

    let len = get_length_w_ref(&s4); // passing a reference 
                                     // doesn't transfer ownership
                                     // so we don't need to 
                                     // return the string back
    println!("The length of '{}' is {}.", s4, len);

    // references are immutable by default
    // meaning we can't change the modify the value of a reference
    // by default
    
    // we modify the value of a reference
    // with mut syntax
    update(&mut s4);

    println!("The length of '{}' is {}.", s4, get_length_w_ref(&s4));

    // we can only have one mutable reference to a value
    // at a time (within a scope)
    
    // new scopes allow for multiple mutable
    // references, just not at the same time
    
    let mut x = String::from("hi");
    {
        let r1 = &mut x;
        println!("{r1}");
    }

    let r2 = &mut x;
    println!("{r2}");

    // we also cannot have a immutable reference
    // whilst there is a mutable reference
    // for a value
}

fn takes_ownership(str: String) { // str comes into scope
    println!("{str}");
} // str goes out of scope and `drop` is called
  
fn makes_copy(int: i32) {
    println!("{int}");
}

fn gives_ownership() -> String {
    return String::from("himate");
}

fn takes_and_gives_back(str: String) -> String {
    println!("{str}");
    return str;
}

fn get_length(str: String) -> (String, usize) {
    let length = str.len();
    return (str, length); 
}

fn get_length_w_ref(str: &String) -> usize {
    return str.len();
} // since this function doesn't own str, the value
  // will not be dropped after the scope ends

fn update(str: &mut String) {
    str.push_str(" world");
}
