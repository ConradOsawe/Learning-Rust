/** while loop accepts a bool value as a condition and will keep executing the block of code until the condition is false */
//while loops
// while loop is more readable than just loop
fn main () {
    let mut counter = 0;

    while counter < 10 {
        println!("{}. Bonito", counter + 1);
        counter += 1;
    }

    //for loops
    // for loops are a way to loop over an item like arrays
let numbers = [1,2,3,4,5];

for number in numbers.iter() {
    println!("The number is: {}", number);
}

// You can also use for loop to loop over a range of numbers
for number in 1..5 {
    println!("The number is: {}", number);

    for number in 1..=5 {
        println!("The number is: {}", number);
    }
}

}