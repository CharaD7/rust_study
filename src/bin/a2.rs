// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers.
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


// * Use a function to add two numbers together
fn add_numbers ( num_1: i32, num_2: i32 ) -> i32 {
    num_1 + num_2
}

// * Use a function to display the result
fn display_result ( result: i32 ) {
    // * Use the "{:?}" token in the println macro to display the result
    println!("{:?}", result);
}
fn main () {

    let numbers = add_numbers(5, 8);
    display_result(numbers);

}
