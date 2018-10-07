// add #[derive(Debug)] to Rectangle for debug print feature.
#[derive(Debug)]
struct Rectange {
    width: u32,
    length: u32
}

//impl
impl Rectange {

    // impl fn's arguments always begin with &self
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // also if you want &mut, use fn area(&mut self)
    fn bigger(&mut self) {
        self.length += 1;
        self.width += 1;
    }

    fn can_hold(&self, other: &Rectange) -> bool {
        self.width > other.width && self.length > other.length
    }

    // associated function
    fn square(size: u32) -> Rectange {
        Rectange { width: size, length: size }
    }
}

fn main() {
    let mut rect = Rectange {
        width: 30,
        length: 50
    };

    println!("{}", get_area(&rect));

    // this won't work, cause Rectangle not satisfied std::fmt::Display from trait bound.
    // println!("rect1 is {}", rect1);

    println!("{:?}", rect);

    rect.bigger();

    println!("{:?}", rect);
    println!("{}", rect.area());

    // call associated functions
    let square = Rectange::square(30);
    println!("{}", square.area());
}

fn get_area(rect: &Rectange) -> u32 {
    rect.width * rect.length
}