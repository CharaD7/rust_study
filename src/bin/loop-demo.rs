fn main () {
    let mut i = 3; // create a mutable variable

    loop {
        println!("Count on {:?}", i);
        i = i - 1; // decrement the count

        // check to see if i = 0
        if i == 0 {
            break;
        }
    }

    println!("Loop completed");
}
