pub fn run() {
    let mut count = 0;

    // infinite
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 5 {
            break;
        }
    }

    // whilte loop (FizzBuzz)
    while count < 100 {
        count += 1;
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("Number: {}", count);
        }
    }

    let arr = 0..100;
    for i in arr {
        println!("i: {}", i);
    }
}
