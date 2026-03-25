use std::collections::{HashMap, HashSet};

mod modules;

fn main() {
    // module 01
    modules::mod_01_hello_rust::hello_rust();
    modules::mod_01_hello_rust::declaring_variables();
    modules::mod_01_hello_rust::sum_to(10);
    modules::mod_01_hello_rust::transform_input("25");
    // practice
    modules::mod_01_hello_rust::convert_temperature("23");
    modules::mod_01_hello_rust::process_greeting("23");

    // module 02
    modules::mod_02_datatypes::multiply(23, 234);
    modules::mod_02_datatypes::swap(23, 26);
    modules::mod_02_datatypes::first_last(&[23, 34, 54, 54]);
    // practice
    modules::mod_02_datatypes::middle_element(&[23, 34, 54, 54]);

    // module 03
    modules::mod_03_functions::max_of_three(4, 66, 5);
    modules::mod_03_functions::abs_value(33);
    modules::mod_03_functions::factorial(2);
    modules::mod_03_functions::fizzbuzz(15);

    // module 04
    modules::mod_04_owrship_borrow::join_strings(String::from("Hello"), String::from("Rust!"));
    modules::mod_04_owrship_borrow::count_chars("1");
    modules::mod_04_owrship_borrow::double_all(&mut vec![2, 34, 4, 3]);
    modules::mod_04_owrship_borrow::first_word("one two three");

    // module 05
    modules::mod_05_struct_methods::rect_area(23, 34);
    // modules::mod_05_struct_methods::use_rectangle()

    coin_value("penny");
    safe_divide(4, 0);
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

    let nums = [1, 2, 3, 4, 5];
    println!("sum of squares: {}", sum_of_squares(&nums));
    println!("doubled: {:?}", double_all(&nums));
    println!("evens only: {:?}", evens_only(&nums));

    let mut countdown = Countdown::new(5);
    while let Some(n) = countdown.next() {
        println!("count: {}", n);
    }

    most_frequent("A for apple and B for banana and C for cat");

    // diagonal_sum("1,2,3;4,5,6;7,8,9");

    // mutable_borrowing_double_all(&mut [1,3,4,5]);
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
    if x.len() >= y.len() {
        x
    } else {
        y
    }
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

trait Summarize {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct Numbers {
    data: Vec<i32>,
}
struct Sentence {
    words: Vec<String>,
}

// Implement Summarize for Numbers (Output = i32, return sum)
impl Summarize for Numbers {
    type Output = i32;

    fn summarize(&self) -> Self::Output {
        self.data.iter().sum()
    }
}

impl Summarize for Sentence {
    type Output = String;

    fn summarize(&self) -> Self::Output {
        self.words.join(" ")
    }
}

fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|x| x * 2).collect()
}

fn sum_of_squares(nums: &[i32]) -> i32 {
    nums.iter().map(|x| x * x).sum()
}

fn evens_only(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&x| *x % 2 == 0).copied().collect()
}

struct Countdown {
    n: i32,
}

impl Countdown {
    fn new(n: i32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n <= 0 {
            None
        } else {
            let current = self.n;
            self.n -= 1;
            Some(current)
        }
    }
}

fn most_frequent(s: &str) -> String {
    let mut map = HashMap::new();
    let words = s.to_lowercase();

    for word in words.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }

    map.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(word, _)| (*word).to_string())
        .unwrap()
}

fn diagonal_sum(input: &str) -> i32 {
    let grid: Vec<Vec<i32>> = input
        .split(';')
        .map(|row| {
            row.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let n = grid.len();
    let mut sum = 0;

    for i in 0..n {
        sum += grid[i][i];
        sum += grid[i][n - 1 - i];
    }

    if n % 2 == 1 {
        sum -= grid[n / 2][n / 2];
    }

    sum
}

fn evaluate(expr: &str) -> Result<i32, String> {
    let expr = expr.trim();

    if expr.chars().any(|c| c.is_alphabetic()) {
        return Err("invalid expression".into());
    }

    if expr
        .chars()
        .any(|c| !c.is_ascii_digit() && !"+-*/ ".contains(c))
    {
        return Err("unknown operator".into());
    }

    let (left, right, operator) = if let Some(pos) = expr.find('+') {
        (&expr[..pos], &expr[pos + 1..], '+')
    } else if let Some(pos) = expr.find('-') {
        (&expr[..pos], &expr[pos + 1..], '-')
    } else if let Some(pos) = expr.find('*') {
        (&expr[..pos], &expr[pos + 1..], '*')
    } else if let Some(pos) = expr.find('/') {
        (&expr[..pos], &expr[pos + 1..], '/')
    } else {
        return Err("unknown operator".into());
    };

    if left.trim().is_empty() || right.trim().is_empty() {
        return Err("invalid expression".into());
    }

    let a: i32 = left.trim().parse().map_err(|_| "invalid expression")?;
    let b: i32 = right.trim().parse().map_err(|_| "invalid expression")?;

    let result = match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0 {
                return Err("division by zero".into());
            }
            a / b
        }
        _ => return Err("invalid expression".into()),
    };

    Ok(result)
}

// fn mutable_borrowing_double_all(){
// }

fn mutable_borrowing_double_all(nums: &mut Vec<i32>) {
    for x in nums {
        *x *= 2;
    }
}
