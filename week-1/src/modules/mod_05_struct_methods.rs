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

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    pub fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn use_rectangle() -> (i32, bool) {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    (rect.area(), rect.is_square())
}
