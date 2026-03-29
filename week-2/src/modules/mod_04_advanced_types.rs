// Functions as Values
pub fn double(x: i32) -> i32 {
    x * 2
}

pub fn increment(x: i32) -> i32 {
    x + 1
}

pub fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// Returning Unnamed Types
pub fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x*n )
}

pub fn compose(
    f: Box<dyn Fn(i32) -> i32>,
    g: Box<dyn Fn(i32) -> i32>,
) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| f(g(x)))
}
