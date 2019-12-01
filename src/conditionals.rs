// Check conditional

pub fn run() {
    let age: i8 = 21;
    let check_id: bool = false;

    if age >= 21 {
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your Id");
    }

    let a: i8 = 2;
    if (a > 1 && a < 5) || a == 2 {
        println!("checked");
    }

    let mess = if (a > 1 && a < 5) || a == 2 {
        "checked"
    } else {
        "nothing"
    };

    println!("{}", mess);
}
