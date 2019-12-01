// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up = 1,
    Down = 2,
    Left,
    Right,
}

fn move_car(m: Movement) {
    // Perform action dependencing on info
    match m {
        Movement::Up => log("up"),
        Movement::Down => log("down"),
        Movement::Left => log("left"),
        Movement::Right => log("right"),
    }

    fn log(mess: &str) {
        println!("Car moving {}", mess)
    }
}

pub fn run() {
    move_car(Movement::Down);
    move_car(Movement::Up);
    move_car(Movement::Right);
    move_car(Movement::Left);
}
