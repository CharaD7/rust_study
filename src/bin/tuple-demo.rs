enum Access {
    Full,
}

// the parenthesis signifies that the function returns 3 32 bit tuple values
fn one_two_three () -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three(); // this is destructuring the tuple into the three variables

    println!("");

    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);

    println!("");

    let (employee, access) = ("Jake", Access::Full);
}
