// Loop
fn main () {
   let mut counter = 0;

   loop {
    println!("{}. Hello, world!", counter + 1);

    counter += 1;

    if counter == 10 {
    break;
   }
   }
}


