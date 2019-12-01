// Struct

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("{} {} {}", c.red, c.green, c.blue);

    c.red = 200;
    println!("{} {} {}", c.red, c.green, c.blue);

    let c2 = Color2(255, 0, 0);
    println!("{} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Tin", "Le");
    println!("Person: {} {}", p.first_name, p.last_name);

    p.set_first_name("Oanh");
    p.set_last_name("Bo");
    println!("FullName: {}", p.full_name());
    println!("tuple: {:?}", p.to_tuple());
}
