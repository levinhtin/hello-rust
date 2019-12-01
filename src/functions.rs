pub fn run() {
    greeting("hello", "Tin");

    println!("func sum: {}", add(4, 5));

    // Closure
    let add_nums = |n1: i8, n2: i8| n1 + n2;
    println!("Closure sum: {}", add_nums(2, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i8, n2: i8) -> i8 {
    n1 + n2
}
