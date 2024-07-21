The `#[derive(Debug)]` attribute in Rust is a macro that automatically implements the `Debug` trait for a struct or enum. The `Debug` trait enables formatting the struct or enum for debugging purposes, using the `{:?}` formatter.

When you derive `Debug`, Rust generates an implementation of the `Debug` trait for your type, which allows you to print the values of your type using the `println!` macro with the `{:?}` formatter. This is particularly useful for inspecting the values of your data structures during development and debugging.

### Example

Here's an example to demonstrate what `#[derive(Debug)]` does:

```rust
#[derive(Debug)]
struct GPSPos {
    latitude: f64,
    longitude: f64,
    description: String,
}

fn main() {
    let pos = GPSPos {
        latitude: 40.4168,
        longitude: -3.7038,
        description: String::from("Madrid"),
    };

    // Print the GPSPos instance using the Debug trait
    println!("{:?}", pos);
}
```

### Explanation

1. **Struct Definition with `#[derive(Debug)]`**:
    ```rust
    #[derive(Debug)]
    struct GPSPos {
        latitude: f64,
        longitude: f64,
        description: String,
    }
    ```
    - The `#[derive(Debug)]` attribute automatically implements the `Debug` trait for the `GPSPos` struct.

2. **Creating an Instance of the Struct**:
    ```rust
    let pos = GPSPos {
        latitude: 40.4168,
        longitude: -3.7038,
        description: String::from("Madrid"),
    };
    ```
    - An instance of `GPSPos` is created with the specified values.

3. **Printing the Struct Instance**:
    ```rust
    println!("{:?}", pos);
    ```
    - The `println!` macro with the `{:?}` formatter is used to print the `pos` instance. This requires the `Debug` trait to be implemented for `GPSPos`, which is automatically done by the `#[derive(Debug)]` attribute.
    - The output will be something like:
      ```
      GPSPos { latitude: 40.4168, longitude: -3.7038, description: "Madrid" }
      ```

### Custom Implementation of `Debug`

If you need more control over how your type is formatted for debugging, you can implement the `Debug` trait manually. Here is an example of a custom implementation:

```rust
use std::fmt;

struct GPSPos {
    latitude: f64,
    longitude: f64,
    description: String,
}

impl fmt::Debug for GPSPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GPSPos {{ lat: {}, lon: {}, desc: {} }}", self.latitude, self.longitude, self.description)
    }
}

fn main() {
    let pos = GPSPos {
        latitude: 40.4168,
        longitude: -3.7038,
        description: String::from("Madrid"),
    };

    // Print the GPSPos instance using the Debug trait
    println!("{:?}", pos);
}
```

### Explanation of Custom Implementation

1. **Implementing the `Debug` Trait**:
    ```rust
    impl fmt::Debug for GPSPos {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "GPSPos {{ lat: {}, lon: {}, desc: {} }}", self.latitude, self.longitude, self.description)
        }
    }
    ```
    - The `fmt` method is implemented to define how `GPSPos` should be formatted for debugging.

2. **Using the Custom `Debug` Implementation**:
    ```rust
    let pos = GPSPos {
        latitude: 40.4168,
        longitude: -3.7038,
        description: String::from("Madrid"),
    };
    println!("{:?}", pos);
    ```
    - The custom `Debug` implementation is used when printing the `pos` instance.

By using `#[derive(Debug)]`, you can quickly enable debugging output for your types without needing to manually implement the `Debug` trait, which is very convenient during development and debugging.