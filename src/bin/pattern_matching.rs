// pattern matching
//Allows you to match the feature of complex data types

//using the match keyword
fn main() {
    let x = 5;

    let y = 6;
    match x {
        1 => println!("one"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("other"),

    }

    // Guards in a match expression
    match y {
        1 => println!("one"),
        2 => println!("Two"),
        3 => println!("Three"),
        n if n > 3 => println!("Greater than 3"), //this is a guard
        _ => println!("other"),
        
    }
}