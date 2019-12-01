// Vectors - Resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i8> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    println!("Vector len: {}", numbers.len());
    println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));

    // Add to vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);

    // pop of last value in vector
    numbers.pop();
    println!("{:?}", numbers);

    let slice: &[i8] = &numbers[1..5];
    println!("Slice: {:?}", slice);

    // loop in vector
    for i in numbers.iter() {
        println!("{}", i);
    }

    // loop mutable values
    for i in numbers.iter_mut() {
        *i += 1;
        println!("{}", i);
    }
    println!("{:?}", numbers);
}
