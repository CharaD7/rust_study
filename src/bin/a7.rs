// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal.
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
// name to print

// * Use an enum with color names as variants
enum Color {
    Red,
    Gold,
    Green,
    Indigo,
    Cyan,
    Magenta,
    Purple,
    Beige,
    Orange,
    Aqua,
    RoyalBlue,
    Yellow,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn display_color( color: Color) {
    // * Use a match expression to determine which color
    // name to print
    match color {
        Color::Red => println!("Red"),
        Color::Gold => println!("Gold"),
        Color::Green => println!("Green"),
        Color::Indigo => println!("Indigo"),
        Color::Cyan => println!("Cyan"),
        Color::Magenta => println!("Magenta"),
        Color::Purple => println!("Purple"),
        Color::Beige => println!("Beige"),
        Color::Orange => println!("Orange"),
        Color::Aqua => println!("Aqua"),
        Color::RoyalBlue => println!("RoyalBlue"),
        Color::Yellow => println!("Yellow"),
    }
    
}

fn main() {
    display_color(Color::Aqua);
}
