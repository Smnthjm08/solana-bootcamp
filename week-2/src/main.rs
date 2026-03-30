mod modules;

fn main() {
    // module 01
    let linked_list = modules::mod_01_smart_pointers::List::Cons(
        3,
        Box::new(modules::mod_01_smart_pointers::List::Cons(
            4,
            Box::new(modules::mod_01_smart_pointers::List::Cons(
                5,
                Box::new(modules::mod_01_smart_pointers::List::Nil),
            )),
        )),
    );
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

    let shapes: Vec<Box<dyn modules::mod_02_dyn::Shape>> = vec![
        Box::new(modules::mod_02_dyn::Circle { radius: 2.0 }),
        Box::new(modules::mod_02_dyn::Rect { w: 3.0, h: 4.0 }),
    ];

    let total_area = modules::mod_02_dyn::total_area(&shapes);
    println!("total_area {}", total_area);

    let input = "  Hello Rust World  ";
    let formatters: Vec<Box<dyn modules::mod_02_dyn::Formatter>> = vec![
        Box::new(modules::mod_02_dyn::Trim),
        Box::new(modules::mod_02_dyn::Snake),
        Box::new(modules::mod_02_dyn::Upper),
    ];
    let formatted = modules::mod_02_dyn::apply_all(input, &formatters);
    println!("Formatted string: {}", formatted);

    // module 03
    let formatted = modules::mod_03_advanced_trait::format_list(vec![1, 2, 3, 4, 5]);
    println!("Formatted list: {}", formatted);

    // module 04
    let double = modules::mod_04_advanced_types::double(10);
    let incremented = modules::mod_04_advanced_types::increment(10);
    let applied =
        modules::mod_04_advanced_types::apply_twice(modules::mod_04_advanced_types::double, 5);
    let multiplier = modules::mod_04_advanced_types::make_multiplier(3);
    println!(
        "double: {}, incremented: {}, apply_twice: {}, multiplier(4): {}",
        double,
        incremented,
        applied,
        multiplier(4)
    );

    // module 05 (one match case example)
    let result = modules::mod_05_ptrn_matching::parse_command("add 2 3");
    println!("parse_command add: {}", result);
    let class = modules::mod_05_ptrn_matching::classify(5);
    println!("classify 5: {}", class);

    // module 06
    let mut a = 1;
    let mut b = 2;
    modules::mod_06_unsafe_rust::swap_values(&mut a, &mut b);
    println!("After swap: a = {}, b = {}", a, b);
    let arr = modules::mod_06_unsafe_rust::SafeArray::new(vec![1, 2, 3, 4]);
    println!("SafeArray get(2): {:?}", arr.get(2));
    println!("SafeArray sum_all: {}", arr.sum_all());

    // module 07
    modules::mod_07_dec_macros::compute(24);
    modules::mod_07_dec_macros::temp_test(100);
    modules::mod_07_dec_macros::total(1, 2, 3);
    // practice
    modules::mod_07_dec_macros::multiply_three(23, 1, 4);
    modules::mod_07_dec_macros::find_max(23, 1, 4);

    // module 08
    let processed = modules::mod_08_concurrency::process_values(vec![1, 2, 3]);
    println!("process_values: {:?}", processed);
    let sum = modules::mod_08_concurrency::parallel_sum(vec![1, 2, 3, 4, 5, 6]);
    println!("parallel_sum: {}", sum);
    let counter = modules::mod_08_concurrency::concurrent_counter(10);
    println!("concurrent_counter: {}", counter);
    let pipeline = modules::mod_08_concurrency::pipeline(vec![1, 2, 3, 4]);
    println!("pipeline: {:?}", pipeline);

    // module 09
    let squares = modules::mod_09_capstone::cached_squares(vec![2, 3, 4, 2, 3]);
    println!("cached_squares: {:?}", squares);
}
