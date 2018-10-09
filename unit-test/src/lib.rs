pub mod shapes;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("boom! {}", value);
        }

        Guess { value }
    }

    fn private_function() -> i32 {
        3
    }
}