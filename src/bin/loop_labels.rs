// loop labels
// nested loops in rust with labelled breaks to contol the flow between them
fn main () {
let mut outer_counter = 0;
'outer: loop {
    let mut inner_counter = 0;
    'inner: loop {
        println!(
            "Outer loop: {}, Inner loop: {}",
            outer_counter, inner_counter
        );

        if inner_counter == 5 {
            break 'inner; //Exit the inner loop
        }
        if outer_counter == 2 {
            break 'outer; //Exit the outer loop
        }
        inner_counter += 1;
        }
        outer_counter += 1;
    }
}

