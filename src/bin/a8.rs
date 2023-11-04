// Topic: Organizing similar data using structs
//
// Program requirements:
// * Print the flavor of a drink and its fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavors {
    Vanilla,
    Grape,
    Strawberry,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavors,
    ounce: f64,
}

// * Use a function to print out the drink flavor and ounces
fn display_drink( drink: Drink ) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavors::Vanilla => println!("Flavor: Vanilla"),
        Flavors::Grape => println!("Flavor: Grape"),
        Flavors::Strawberry => println!("Flavor: Strawberry"),
    }

    println!( "Ounce: {:?}", drink.ounce );
}

fn main() {
    let vanilla = Drink {
        flavor: Flavors::Vanilla,
        ounce: 5.3
    };
    display_drink(vanilla);

    println!("");

    let strawberry = Drink {
        flavor: Flavors::Strawberry,
        ounce: 7.0
    };
    display_drink(strawberry);

    println!("");

    let grape = Drink {
        flavor: Flavors::Grape,
        ounce: 6.9
    };
    display_drink(grape);
}
