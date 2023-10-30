fn main () {
    let my_name = "Bob";

    match my_name {
        // if it is Jayson, do this
        "Jayson" => println!("that is my name"),

        // if it is BOb, do this
        "Bob" => println!("not my name"),

        // if it is Alice, do this
        "Alice" => println!("hello Alice"),

        // if it is anything else, do this
        _ => println!("nice to meet you"),
    }
}
