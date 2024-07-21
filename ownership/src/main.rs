fn main() {
    

    
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s1}, world!");

    let mut s1 = String::from("hello");
    let mut s2 = &mut s1;

    println!("{s1}, world!");
    println!("{s2}, world!");

    //s1.push_str(" world");

    println!("{s1}!!!");
    println!("{s2}!!!");
}
