/// Guess 物件
#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    /// 產生新的 Guess
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    /// 取得 Guess 的值
    pub fn value(&self) -> i32 {
        self.value
    }
}
