fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn main() {
    let pair = (1, true);
    println!("The first and second item in the tuples are {} and {}", pair.0, pair.1);
    println!("The reversed pair is: {:?}", reverse(pair));

    let (a, b) = pair;

    println!("A is now {}", a);
    println!("B is now {}", b);
}