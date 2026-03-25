pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}


pub fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

pub fn first_last(nums: &[i32]) -> (i32, i32) {
    let first_elem = nums[0];
    let last_elem = nums[nums.len() - 1];
    (first_elem, last_elem)
}

// practice
pub fn middle_element(nums: &[i32]) -> i32 {
    let is_even = nums.len() % 2 == 0;
    if is_even {
        nums[nums.len() / 2] 
    } else {
        nums[nums.len() / 2]
    }
}