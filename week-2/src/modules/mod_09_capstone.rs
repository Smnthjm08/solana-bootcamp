use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

// state transitions
#[derive(Clone, Copy)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

fn next_state(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
    }
}

fn name(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Red",
        TrafficLight::Green => "Green",
        TrafficLight::Yellow => "Yellow",
    }
}

fn simulate(steps: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = TrafficLight::Red;

    result.push(name(current).to_string());
    for _ in 0..steps {
        result.push(name(current).to_string());
        current = next_state(current);
    }

    result
}

// recursive expression
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(x) => -eval(x),
    }
}

// cached_squares
pub fn cached_squares(inputs: Vec<i32>) -> Vec<i32> {
    let cache: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for num in inputs {
        let cache_clone = Arc::clone(&cache);

        let handle = thread::spawn(move || {
            let mut map = cache_clone.lock().unwrap();

            if let Some(&val) = map.get(&num) {
                val
            } else {
                let square = num * num;
                map.insert(num, square);
                square
            }
        });

        handles.push(handle);
    }

    handles.into_iter().map(|x| x.join().unwrap()).collect()
}
