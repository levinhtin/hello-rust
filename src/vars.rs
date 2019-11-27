// variables

pub fn run() {
    let name = "Tin";
    let mut age = 27;
    age += 1;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Tin", 28);
    println!("{} is {}", my_name, my_age);
}
