use std::mem;


fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice is: {}", slice[0]);
    println!("the slice has {} element", slice.len());
}

fn main () {

    // The first item is the pointer to the data, 
    // the second the length of the slice
    let xs: [i32;5] = [1, 2, 3, 4, 5];

    // Initialze all elements with the same value
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array is {}", xs[0]);
    println!("second element of the array is {}", xs[1]);

    println!("size of the array is: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
}