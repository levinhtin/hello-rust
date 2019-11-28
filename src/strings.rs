// Primitive str = Immutable fixed-length string somewhere inmemory
// String = Growable, heap-allowcated data structure - Use when you need to modify or own string data

pub fn run() {
    let first_name: &str = "tin";
    println!("{}", first_name);
    println!("length {}", first_name.len());
    println!("is empty {}", first_name.is_empty());
    let mut last_name = String::from("le");
    last_name.push('-');
    last_name.push_str("van vinh");
    println!("{}", last_name);
    println!("{}", last_name.len());
    println!("capacity: {}", last_name.capacity());
    println!("is empty {}", last_name.is_empty());

    println!("Contains 'Le' {}", last_name.contains("le"));
    println!("Replace: {}", last_name.replace("vinh", "oanh le thi kieu"));

    // loop throught string by whitespace
    for _char in last_name.split_whitespace() {
        println!("{}", _char);
    }

    // create string with capacity
    let mut job = String::from("abcdefg8");
    println!("capacity: {}", job.capacity());
    job.push('1');
    println!("capacity: {}", job.capacity());
    job.push_str("five5");
    println!("capacity: {}", job.capacity());
    job.push_str("123");
    println!("capacity: {}", job.capacity());
    println!("{}", job);
}
