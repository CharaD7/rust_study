// Topic: Functions
//
// Program Requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// * Use a function to display your first name
fn first_name( name: &str ) -> String {
    name.to_string()
}

// * Use a function to display your last name
fn last_name( name: &str ) -> String {
    name.to_string()
}


fn main() {

    let fname = first_name("Joy");
    let lname = last_name("Ayitey");

    println!("{fname} {lname}")
}
