/*
    Primitive types --
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    Floats: f32, f64
    Boolean: bool
    Characters: char
    Tuples
    Array
*/
pub fn run() {
    // default is "i32"
    let x = 1;

    // default is "f64"
    let y = 2.5;

    // explicit type
    let z: i32 = 321312321;

    // Boolean
    let is_active = true;

    println!("all {:?}", (x, y, z, is_active));

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i32::MAX);

    // Get boolean from expression
    let is_greater = x > z;

    println!("is greater {}", is_greater);

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (a1, face));
    println!("face: {}", face);
}
