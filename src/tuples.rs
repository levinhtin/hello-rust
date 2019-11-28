// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("tin", "Hue", 28);

    println!("{:?}", person);
    println!("{} is form {} and is {}", person.0, person.1, person.2);
}
