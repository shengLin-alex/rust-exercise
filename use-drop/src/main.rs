// Drop trait, 類似於 c++ 中搭配 RAII 的 destructor,
// 更類似於 .NET 中的 IDisposable

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer: {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    // 使用 std::mom::drop 來提早清理
    drop(c);
    println!("c dropped");
} // 離開作用域時呼叫 drop()