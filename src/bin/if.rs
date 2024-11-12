//control flow - if
// if is an expression

fn main () {
    let number = 3;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

//using if in a statement
// if is an expression in rust
    let condition = true;

    let number = if condition {
        10
    } else {
        100
    };
    println!("The value of number is {}", number);

    
 

 //The type of values in the if else blocks be the same

//  let condition = true;
 
//  let number = if condition {
//      10
//  } else {
//    "hello"
//  };

//  println!("The value of number is: {}", number);
// } 

// the code directly above will give an error

//using else if
let grades = 90;

if grades >= 90 {
    println!("You got an A");
} else if grades >= 80{ 
println!("You got a B");
} else if grades >= 70 {
    println!("You got a C");
    } else {
        println!("You failed");
 }
 }
