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
 fn main() {
    let x = {
        let y = 5;
        y + 1
    };

    println!("The value of x is:{}", x);
  }