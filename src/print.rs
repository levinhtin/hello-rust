pub const NAME: &str = "tinlvv";

pub fn say() {
  let message: String = format!("hello from {} file", "print.rs");
  println!("{}", message);

  // Basic format
  println!("{} is from {}", NAME, "Hue");

  // Position arguments
  println!("{0} is from {1} and {0} likes {2}", NAME, "Hue", "football");

  // named arguments
  println!("{name} is a {job}", name = NAME, job = "coder");

  // Placeholder traits
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "hello"));

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
