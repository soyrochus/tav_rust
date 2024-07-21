fn shout(s: &str) {
    
    let upper_s = s.to_uppercase();
    println!("{}", upper_s);

}

fn main() {
    // Allocate a string
    let str = String::from("Hello, World!");

    shout(&str);

    // Write a NULL to the string address (by setting it to None)
    // str = NULL: is not allowed in Rust
    //str = None;

    // Call shout again with the NULL string
    shout(&str);
}