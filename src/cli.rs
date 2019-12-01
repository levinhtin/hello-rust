use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Tin";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    }
}
