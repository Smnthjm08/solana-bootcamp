mod modules;

use modules::mod_01_smart_pointers::List::{Cons, Nil};
use modules::mod_02_dyn::{Upper, Snake, Trim};
use crate::modules::mod_02_dyn::{self, Circle, Rect, Shape};

fn main() {
    // module 01
    let linked_list = Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))));
    let sum = modules::mod_01_smart_pointers::list_sum(&linked_list);
    println!("list_sum: {}", sum);

    let s = "hello";
    let double = modules::mod_01_smart_pointers::double_len(s);
    println!("double_len: {}", double);

    let owners = modules::mod_01_smart_pointers::count_owners(5);
    println!("count_owners: {}", owners);

    let count = modules::mod_01_smart_pointers::count_to(7);
    println!("count_to: {}", count);

    // module 02
    // pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rect { w: 3.0, h: 4.0 }),
    ];

    let total_area = mod_02_dyn::total_area(&shapes);
    println!("total_area {}", total_area);

    let input = "  Hello Rust World  ";
    let formatters: Vec<Box<dyn modules::mod_02_dyn::Formatter>> = vec![
        Box::new(Trim),
        Box::new(Snake),
        Box::new(Upper),
    ];
    let formatted = modules::mod_02_dyn::apply_all(input, &formatters);
    println!("Formatted string: {}", formatted);

    // module 07
    modules::mod_07_dec_macros::compute(24);
    modules::mod_07_dec_macros::temp_test(100);
    modules::mod_07_dec_macros::total(1, 2, 3);
    // practice
    modules::mod_07_dec_macros::multiply_three(23, 1, 4);
    modules::mod_07_dec_macros::find_max(23, 1, 4);
}
