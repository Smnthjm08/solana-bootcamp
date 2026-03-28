// Define the square! macro here
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

pub fn compute(n: i32) -> i32 {
    square!(n)
}

// Define the convert! macro
macro_rules! convert {
    (celsius_to_f, $temp: expr) => {
        // (TEMP * 9 / 5 + 32)
        $temp * 9 / 5 + 32
    };
    // ((TEMP - 32) * 5 / 9)
    (f_to_celsius, $temp: expr) => {
        ($temp - 32) * 5 / 9
    };
}

pub fn temp_test(c: i32) -> i32 {
    convert!(celsius_to_f, c)
}

// Define the sum! macro with repetitions
macro_rules! sum {
    ($a:expr, $b:expr, $c:expr) => {
        $a + $b + $c
    };
}

pub fn total(a: i32, b: i32, c: i32) -> i32 {
    sum!(a, b, c)
}

// Define the product! macro with repetitions
macro_rules! product {
    ($a:expr, $b:expr, $c:expr) => {
        $a * $b * $c
    };
}

pub fn multiply_three(a: i32, b: i32, c: i32) -> i32 {
    product!(a, b, c)
}

// Define the max! macro with repetitions
macro_rules! max { 
    ($a:expr, $b:expr, $c:expr) => {
        $a.max($b).max($c)
    }
 }

pub fn find_max(a: i32, b: i32, c: i32) -> i32 {
    max!(a,b,c)
}
