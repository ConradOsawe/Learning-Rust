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


   // loop can be an expression-- to evaluate or calculate value

   let mut xounter = 0;

let result = loop {
   xounter += 1; 

   if counter == 10 {
      break xounter * 2;
   }
  };

  println!("The result is: {}", result);



  // Skippping an iteartion with "continue" keyword

  let mut sounter = 0;

  loop { 
   sounter += 1;

   if sounter % 2 == 0 { // if sounter % 2 ! = 0 for even number 2-10
      continue;
   }

   if sounter > 9 {
      break;
   }
   println!("The number is: {}", sounter);

  }
  
   

}







