pub const NAME: &str = "tinlvv";

pub fn say() {
  let message: String = format!("hello from {} file", "print.rs");
  println!("{}", message);
  println!("{} is from {}", NAME, "Hue");
  println!("{0} is from {1} and {0} likes {2}", NAME, "Hue", "football");
  println!("{name} is a {job}", name = NAME, job = "coder");

  say_hello("Number", 1);

  let total: i16 = calculate(5, 2);
  println!("Total: {}", total);
}

pub fn say_hello(message: &str, number: i16) {
  println!("{}: {}", message, number);
}

pub fn calculate(x: i16, y: i16) -> (i16) {
  let total = x + y;

  return total;
}
