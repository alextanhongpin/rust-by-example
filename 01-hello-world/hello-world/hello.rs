// This structure cannot be printed either with `fmt::Display` or 
// with `fmt::Debug`
// struct Unprintable(i32);

// This `derive` attribute automatically creates the implementation 
// required to make this `struct` printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

/// A human being is represented here
fn main() {
    println!("Hello world!");

    println!("this is {0}, and this is {1}. {1}, say hi to {0}!", "John", "Doe");

    println!("You can also name food as {food}, paper as {paper}", food="apple", paper="tree");


    println!("This is how it will look like: {:?}", DebugPrintable(3))
}