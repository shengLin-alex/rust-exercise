use std::collections::HashMap;

/// 定義一個可以存放閉包之物件
/// 為了實現緩存(cache) 的機制, 將 values 定義為 Option,
/// 當 values 為 None 時, 執行閉包運算, 為 Some() 時即可直接取得數據
/// 
/// 因此便可以解決, 多次實際呼叫閉包進行耗時計算的資源浪費
pub struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    values: HashMap<u32, Option<u32>>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    /// 構造函數, 預設 values 為 none
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    /// 暫存機制之實現, values 為 None 時呼叫閉包, 為 Some 時, 直接回傳數據
    pub fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => {
                if let Some(result) = v {
                    *result
                } else {
                    panic!("Error when calculating.")
                }
            },
            None => {
                let result = (self.calculation)(arg);
                self.values.insert(arg, Some(result));

                result
            }
        }
    }
}