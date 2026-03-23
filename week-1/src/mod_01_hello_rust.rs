pub fn hello_rust() {
    // Print: Hello, Rust!
    println!("Hello, Rust!");
}

pub fn declaring_variables() {
    // Create x and name, then print them
    let x = 42;
    let name = "Rust";
    println!("x = {}", x);
    println!("name = {}", name);
}

pub fn sum_to(n: i32) -> i32 {
    // Use a mutable variable to accumulate the sum
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    println!("sum_to: sum to {} is {}", n, sum);
    sum
}

pub fn transform_input(s: &str) -> i32 {
    // Shadow a variable through each step
    let s = s.trim();
    let s = s.parse::<i32>().unwrap();
    let result = s * s;
    println!("transform_input: input squared is {}", result);
    result
}

// practice
pub fn convert_temperature(temp_str: &str) -> f64 {
    // Trims whitespace from the input string
    // Parses it into an f64 (Celsius temperature)
    // Converts to Fahrenheit using the formula: F = C * 9.0 / 5.0 + 32.0
    // Use shadowing to reuse the variable name temp at each step.
    let temp_str = temp_str.trim();
    let temp_str = temp_str.parse::<f64>().unwrap();
    let temp_str = temp_str * 9.0 / 5.0 + 32.0;
    println!("convert_temperature: {}", temp_str);
    temp_str
}

pub fn process_greeting(s: &str) -> usize {
    // Trims whitespace from the input string
    // Converts it to lowercase
    // Counts the number of characters (excluding spaces)
    // Use shadowing to reuse the same variable name s at each step.
    let s = s.trim();
    let s = s.to_lowercase();
    let s = s.replace(" ", "");
    let len = s.len();
    println!("process_greeting: processed greeting length is {}", len);
    len
}
