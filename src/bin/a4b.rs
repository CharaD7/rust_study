// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether 
// the value of variable is 1, 2, 3, or some other number,
// respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an undersscore (_) to match on any value

fn main () {
    // * Use a variable set to any integer
    let num_of_oranges = 3;

    // * Use a match expression to determine which message to display
    match num_of_oranges {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

