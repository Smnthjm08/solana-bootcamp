pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

pub fn rect_area(w: i32, h: i32) -> i32 {
    // Create a Rectangle and return its area
    let rect = Rectangle {
        width: w,
        height: h,
    };
    rect.area()
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn _is_square(&self) -> bool {
        self.width == self.height
    }

    pub fn _square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn _use_rectangle() -> (i32, bool) {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    (rect.area(), rect._is_square())
}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_len = 0;
    let mut curr_len = 0;

    for num in nums {
        if num == 1 {
            curr_len += 1;
            max_len = max_len.max(curr_len);
        } else {
            curr_len = 0;
        }
    }

    max_len
}

pub fn _shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut res = vec![];
    for i in 0..n {
        res.push(nums[i]);
        res.push(nums[n + i]);
    }
    res
}
