use std::mem;

pub fn run() {
    let mut arr: [i8; 2] = [1, 2];

    println!("index 0: {}", arr[0]);

    arr[1] = 3;

    for i in &arr {
        println!("{}", i);
    }

    println!("Array occupies {} bytes", mem::size_of_val(&arr));

    let slice: &[i8] = &arr[0..2];
    println!("slice: {:?}", slice);
}
