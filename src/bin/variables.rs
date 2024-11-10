
//Global variable; constant
//We can't use let outside the main fn
//const keyword are only immutable; you can't use the mut keyword
const MAX_SIZE: u32 = 100_000;
const PI: f32 = 3.14;
const AUHTOR: &str = "John Doe";

fn main () {
    //mutable variables
    let mut x= 5;
println!("{}", x);

x = 10;
println!("{}", x);

//Shadowing
let x = 5;

let x = x + 1;

let x = x * 2;

println!("The Value of x is: {}", x);

//constant outside fn main
println!("the max_size is {}", MAX_SIZE);
println!("The pi of a circle is {}", PI);
println!("The author of this code is {}", AUHTOR);


//scope
let x = 5;
{
    let y = 10;
    println!("The value of y is: {}", y);
}

println!("The value of x is: {}", x);

}