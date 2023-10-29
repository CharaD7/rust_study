fn main() {

    let mut a = 0;

    // Starting with an infinite loop

    loop {
        if a == 5 {
            break;
        }

        println!("{:?}", a);

        a = a + 1;
    }

    // Starting with a finite loop

    let mut b = 0;

    while b != 5 {
        println!("{:?}", b);

        b = b + 1;
    }
}
