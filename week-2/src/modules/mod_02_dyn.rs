use std::{f64::consts::PI};

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}
pub struct Rect {
    pub w: f64,
    pub h: f64,
}

// Implement Shape for Circle and Rect
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.h * self.w
    }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// pipeline
pub trait Formatter {
    fn format(&self, input: &str) -> String;
}

pub struct Upper;
pub struct Snake;
pub struct Trim;

impl Formatter for Upper {
    fn format(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

impl Formatter for Snake {
    fn format(&self, input: &str) -> String {
        input.to_lowercase().replace(" ", "_")
    }
}

impl Formatter for Trim {
    fn format(&self, input: &str) -> String {
        input.trim().to_string()
    }
}

pub fn apply_all(input: &str, fmts: &[Box<dyn Formatter>]) -> String {
    let mut res = input.to_string();

    for each in fmts {
        res = each.format(&res);
    }
    res
}
