pub fn join_strings(a: String, b: String) -> String {
    a + " " + &b
}

pub fn count_chars(s: &str) -> usize {
    let length = s.len();
    length
}

pub fn double_all(nums: &mut Vec<i32>) {
    for x in nums {
        *x *= 2;
    }
}

pub fn first_word(s: &str) -> String {
    for i in s.split_whitespace() {
        return i.to_string();
    }
    String::new()
}