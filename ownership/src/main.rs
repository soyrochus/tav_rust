fn immutable_and_shared_example() {
    // Immutable and Shared Example

    // Define an immutable value
    let x = 5;

    // Create multiple immutable references to `x`
    let ref1 = &x;
    let ref2 = &x;
    let ref3 = &x;

    // Print the references to demonstrate they are all pointing to the same value
    println!("Immutable references:");
    println!("ref1: {}, ref2: {}, ref3: {}", ref1, ref2, ref3);

    // Note: We can't modify `x` through any of these references, as they are immutable.
    // This guarantees that the value `x` is shared but not modified.

    // Mutable and Exclusive Example

    // Define a mutable value
    let mut y = 10;

    // Create a single mutable reference to `y`
    let ref_mut = &mut y;

    // Modify the value through the mutable reference
    //y += 5;   //Cannot use y directly as it is borrowed
    *ref_mut += 5;

    // Print the new value to demonstrate the change
    println!("Mutable reference:");
    println!("ref_mut: {}", ref_mut);

    // Note: We can't create another mutable reference to `y` while `ref_mut` is active.
    // This guarantees that the value `y` can be modified, but only through one reference at a time.
}

// Ownership Example

fn ownership_example() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // `s1` is the owner of the string "hello"
    let s1 = String::from("hello");

    // `s2` takes ownership of the string from `s1`
    let s2 = s1;

    // Uncommenting the next line would cause a compile-time error because `s1` is no longer valid
    // println!("s1: {}", s1);

    // `s2` is now the owner of the string
    println!("s2: {}", s2);

    // When `s2` goes out of scope, the memory for the string will be automatically dropped.
}

fn main() {
    
    immutable_and_shared_example();
    ownership_example();
}
