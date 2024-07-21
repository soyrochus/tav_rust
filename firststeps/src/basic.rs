
pub fn basic() {
    // Rust is a statically typed language, meaning all variables have a type known at compile time.

    // Immutable variable with inferred type (i32 by default)
    let x = 42;
    println!("x: {}", x); // Output: x: 42

    // Mutable variable with explicit type annotation
    let mut y: i32 = 10;
    y += 5; // y can be changed because it's mutable
    println!("y: {}", y); // Output: y: 15

    // Scalar types: integers, floating-point numbers, Booleans, and characters

    // Integer types with explicit type annotations
    let a: i8 = -128;  // 8-bit signed integer
    let b: u16 = 65535; // 16-bit unsigned integer
    println!("a: {}, b: {}", a, b); // Output: a: -128, b: 65535

    // Floating-point types
    let c: f32 = 3.14;  // 32-bit floating point
    let d: f64 = 2.71828; // 64-bit floating point (default)
    println!("c: {}, d: {}", c, d); // Output: c: 3.14, d: 2.71828

    // Boolean type
    let t: bool = true;
    let f: bool = false;
    println!("t: {}, f: {}", t, f); // Output: t: true, f: false

    // Character type (supports Unicode)
    let heart_eyed_cat: char = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat); // Output: heart_eyed_cat: 😻

    // Compound types: tuples and arrays

    // Tuple with elements of different types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple: {:?}", tuple); // Output: tuple: (500, 6.4, 1)

    // Destructuring a tuple into individual variables
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z); // Output: x: 500, y: 6.4, z: 1

    // Accessing tuple elements by index
    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one); // Output: five_hundred: 500, six_point_four: 6.4, one: 1

    // Array with elements of the same type
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 elements, all i32
    println!("array: {:?}", array); // Output: array: [1, 2, 3, 4, 5]

    // Accessing array elements by index
    let first = array[0];
    let second = array[1];
    println!("first: {}, second: {}", first, second); // Output: first: 1, second: 2

    // Arrays with initialized repeated values
    let same_values = [3; 5]; // Array with 5 elements, all initialized to 3
    println!("same_values: {:?}", same_values); // Output: same_values: [3, 3, 3, 3, 3]

    // Slices: reference to a contiguous sequence of elements in a collection
    let slice = &array[1..3]; // Slices the array to include elements at index 1 and 2
    println!("slice: {:?}", slice); // Output: slice: [2, 3]

}