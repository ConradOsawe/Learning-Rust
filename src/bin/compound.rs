// Compound data types
fn main() {
    //array
    let a = [1,2,3,4,5];
    println!("The value of a is: {:?}", a);

    //arrays are useful for fixed number of elements of the sane type
    // Like the 4 seasons
    let seasons = ["spring", "summer", "Fall", "winter"];

    println!("The value of the season is: {:?}", seasons);
    println!("{}", seasons[3]);

    //accessing array element by their index
    let a = [1, 2, 3, 4, 5];
 
    println!("The first element of a is: {}", a[0]);
    println!("The second element of a is: {}", a[1]);
    println!("The third element of a is: {}", a[2]);
    println!("The fourth element of a is: {}", a[3]);
    println!("The fifth element of a is: {}", a[0]);

    //Tuples 
    // tuples are used to store element of the different type and you know for sure that the number of eelements will not change

    let tup = (999, 19.3, "sai");

    println!("The value of tup is: {:?}", tup);

    //accesing tuple with . operator and index
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);

    //Destructuring of tuples or pattern matching
    let tup = (999, 19.3, "sai");
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}