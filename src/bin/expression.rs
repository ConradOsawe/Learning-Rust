// expression and statement

// statement:
/** fn main () {
    println!("Hello, world!"); //statement
} **/

// expresssion
/** fn main() {
    let x = 5;
    let y = 10;

    x + y // expression
 } **/

 // this is a scope
 /** fn main() {
    let x = {
        let y = 5;
        y + 1
    }; 

    println!("The value of x is:{}", x);
  } **/

  //function return Values 
  fn add(a:i32, b:i32) -> i32 {
    //return a + b; or:
    a + b
  }

  fn main() {
    // let result = add(5, 3);

    // println!("The sum of 5 and 3 is {}", result);

    //directly using the return value
    println!("The sum of 5 and 3 is {}", add(5, 3));
  }

  