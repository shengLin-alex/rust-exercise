pub struct Rectangle {
    pub length: i32,
    pub width: i32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}