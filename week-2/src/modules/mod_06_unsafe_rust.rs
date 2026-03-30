// raw pointers
pub fn swap_values(a: &mut i32, b: &mut i32) {
    // Convert to raw pointers, then swap in unsafe block
    let raw_a = a as *mut i32;
    let raw_b = b as *mut i32;
    unsafe { std::ptr::swap(raw_b, raw_a) }
}

// safe api
pub struct SafeArray {
    data: Vec<i32>,
}

impl SafeArray {
    pub fn new(data: Vec<i32>) -> Self {
        SafeArray { data }
    }

    pub fn get(&self, i: usize) -> Option<i32> {
        self.data.get(i).copied()
    }

    pub unsafe fn get_unchecked(&self, i: usize) -> i32 {
        *self.data.get_unchecked(i)
    }

    pub fn sum_all(&self) -> i32 {
        self.data.iter().sum()
    }
}
