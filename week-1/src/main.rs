use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    max_of_three(4, 66, 5);
    abs_value(33);
    factorial(2);
    fizzbuzz(23);
    join_strings(String::from("Hello"), String::from("Rust!"));
    count_chars("1");
    // double_all(&mut [1, 2, 3].to_vec());
    first_word("qwe err e eew");
    use_rectangle();
    coin_value("penny");
    safe_divide(4, 0);
    println!("area of rect: {}", rect_area(100, 100));
    println!("total marks {}", get_grade(60));
    // println!("total marks {}", parse_number("3435"));
    let _ = parse_number("2432");
    sum_vec(&[1, 2, 3, 8, 42, 56, 6]);
    count_vowels("sdsdsdwi2eehwfbewhfhewihrwedhvbewuc");
    largest(&[1, 2, 34, 4, 5, 56, 343]);
    unique_word_count("this is the sample test");
    classify(234);
    longest("4", "4");
    trim_prefix("Hellohello", "hello");

    let _ = add_parsed("123", "23");
    let _ = parse_csv_sum("1,2,3");
}

// functions with return value
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

// if Returns a Value
fn abs_value(n: i32) -> i32 {
    // n.abs()
    n.abs()
}

fn factorial(n: u64) -> u64 {
    // Compute n factorial
    let mut num = n;
    let mut result = 1;
    while num >= 1 {
        result *= num;
        num -= 1;
    }
    result
}

fn fizzbuzz(n: i32) -> String {
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

fn join_strings(a: String, b: String) -> String {
    a + " " + &b
}

fn count_chars(s: &str) -> usize {
    let length = s.len();
    length
}

// fn double_all(nums: &mut Vec<i32>) {
//     // Double each element in place
//     let mut my_vec = vec![];

//     for x in nums{
//         my_vec.push(*x * 2);
//     }
//     my_vec
// }

fn first_word(s: &str) -> String {
    for i in s.split_whitespace() {
        println!("{}", i);
        return i.to_string();
    }
    String::new()
}

struct Rectangle {
    // Define width and height fields
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn rect_area(w: i32, h: i32) -> i32 {
    // Create a Rectangle and return its area
    w * h
}

fn use_rectangle() -> (i32, bool) {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    return (rect.area(), rect.is_square());
}

// impl Rectangle {
//     fn square(size: i32) -> Self {
//         Self { width: size, height: size }
//     }

//     fn area(&self) -> i32 {
//         self.width * self.height
//     }
// }

fn coin_value(coin: &str) -> i32 {
    // Use match on the coin string
    match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0,
    }
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

fn get_grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("invalid number".to_string()),
    }
}

fn sum_vec(nums: &[i32]) -> i32 {
    let sum = nums.iter().sum();
    println!("sum in vec: {}", sum);
    sum
}

fn count_vowels(s: &str) -> usize {
    let mut counts = 0;
    for c in s.chars() {
        if "aeiou".contains(c) {
            counts += 1;
        }
    }
    println!("{}", counts);
    counts
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for i in list {
        if i > &largest {
            largest = *i;
        }
    }
    largest
}

trait Describable {
    fn describe(&self) -> String;
}

struct Item {
    name: String,
    price: i32,
}

impl Describable for Item {
    fn describe(&self) -> String {
        format!("{}: {} cents", self.name, self.price)
    }
}

#[derive(PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn are_equal(a: &Point, b: &Point) -> bool {
    a == b
}

fn distance_sq(a: &Point, b: &Point) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn classify(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        x if x > 0 => "positive",
        _ => "zero",
    }
}

fn unique_word_count(s: &str) -> usize {
    let mut set = HashSet::new();

    for chars in s.split_whitespace() {
        set.insert(chars);
    }
    set.len()
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    // if  s.starts_with(prefix){
    //     s.strip_prefix(prefix).unwrap_or(s)
    // } else{
    //     s
    // }
    s.strip_prefix(prefix).unwrap_or(s)
}

fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    // Parse both strings and return their sum
    // Use ? to propagate errors
    let parsed_a = a.parse::<i32>().map_err(|_| "parse error".to_string())?;
    let parsed_b = b.parse::<i32>().map_err(|_| "parse error".to_string())?;

    Ok(parsed_a + parsed_b)
}

fn parse_csv_sum(s: &str) -> Result<i32, String> {
    let mut total = 0;

    for part in s.split(",") {
        if part.is_empty() {
            return Err("empty input".to_string());
        }

        let num = part
            .parse::<i32>()
            .map_err(|_| format!("invalid number: {}", part))?;
        total += num;
    }

    Ok(total)
}

