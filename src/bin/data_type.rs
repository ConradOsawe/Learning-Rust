fn main() {
    //scalable data types
    // integer : signeed and unsigned
    // float
    // string
    // boolean
    let z:  i32 = "999".parse().unwrap();

    println!("The value of z is: {}", z);

    //integer literals
    // decimal
    // hexadecimal
    // octal
    // binary
    // byte


    //Mathematical operations 
    let sum = 5 + 10;
    let difference = 95.5 -  4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 45 % 5;

    //let product = 4 * 30.4; compile time error because of diffrenet type
      // let product = 4 * 40.4 as i32; it's going to work
      let quotien = 11/10;

    println!("The sum is: {}", sum);
    println!("The difference is: {}", difference);
    println!("The product is: {}", product);
    println!("The quotient is: {}", quotient);
    println!("The remainder is: {}", remainder);
    println!("The 2quotient is: {}", quotien);

    //Booleans are used for conditional logic and control flows
    let is_true = true;
    let is_false = false;

    println!("{}", is_true);
    println!("{}", is_false);

    //char 
    let c = 'z'; // inferred as `char`
    let z: char = '😀';
    let chinese: char = '中';
    let japanese: char = '日';
 
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of chinese is: {}", chinese);
    println!("The value of japanese is: {}", japanese);
}
    

