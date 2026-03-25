pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

pub fn abs_value(n: i32) -> i32 {
    // n.abs()
    n.abs()
}

pub fn factorial(n: u64) -> u64 {
    // Compute n factorial
    let mut num = n;
    let mut result = 1;
    while num >= 1 {
        result *= num;
        num -= 1;
    }
    result
}

pub fn fizzbuzz(n: i32) -> String {
    if n % 5 == 0 && n % 3 == 0 {
        String::from("FizzBuzz")
    } else if n % 5 == 0 {
        String::from("Fizz")
    } else if n % 3 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}
