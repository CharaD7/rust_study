// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable.
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal


fn main() {

    // * Use a variable set to either true or false
    let gender = 'M';

    // * Use an if..else block to determine which message to display
    if gender == 'M' {
        // * Use the println macro to display messages to the terminal
        println!("You are male");
    }
    else {
        // * Use the println macro to display messages to the terminal
        println!("You are not male")
    }
}
