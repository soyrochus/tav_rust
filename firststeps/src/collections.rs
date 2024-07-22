use std::collections::{HashMap, HashSet};

fn vector_example() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);


    println!("First element: {}", v[0]);

    for i in &v {
        println!("Element: {}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    println!("Modified vector: {:?}", v);
}

fn hashmap_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("Score for {}: {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);

    println!("Updated scores: {:?}", scores);
}

fn hashset_example() {
    let mut books = HashSet::new();
    books.insert("Pride and Prejudice");
    books.insert("To Kill a Mockingbird");
    books.insert("The Great Gatsby");

    if books.contains("To Kill a Mockingbird") {
        println!("The book is in the collection.");
    }

    for book in &books {
        println!("Book: {}", book);
    }

    books.remove("The Great Gatsby");

    println!("Updated book collection: {:?}", books);
}


pub fn collections() {
    vector_example();
    hashmap_example();
    hashset_example();
}