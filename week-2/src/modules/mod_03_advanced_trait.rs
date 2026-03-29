use std::{fmt};
use std::fmt::{Display};
use std::ops::Add;

// orphan rule

struct CommaSeparated(Vec<i32>);

// Implement Display for CommaSeparated
impl Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strs: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        write!(f, "{}", strs.join(", "))
    }
}

pub fn format_list(nums: Vec<i32>) -> String {
    format!("{}", CommaSeparated(nums))
}

// summary for numbers and words
trait Summary {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct Numbers {
    data: Vec<i32>,
}
struct Words {
    data: Vec<String>,
}

// Implement Summary for Numbers (Output = i32, return sum)
impl Summary for Numbers {
    type Output = i32;
    fn summarize(&self) -> Self::Output {
        self.data.iter().sum()
    }
}

// Implement Summary for Words (Output = String, return joined)
impl Summary for Words {
    type Output = String;
    fn summarize(&self) -> Self::Output {
        self.data.join("  ")
    }
}

// custom operators
struct Vec2 {
    x: f64,
    y: f64,
}

// Implement Add for Vec2
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Implement Display for Vec2 — format as (x, y) with 1 decimal
impl Display for Vec2 {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

fn add_vecs(a: Vec2, b: Vec2) -> String {
    format!("{}", a + b)
}
