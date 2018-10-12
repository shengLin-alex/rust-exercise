use std::ops::Deref;

fn main() {
    deref_coercions();
}

#[test]
fn dereference() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);

    // * 解引用, 類似於 c/c++ de-pointer
    assert_eq!(5, *y);
}

#[test]
fn debox() {
    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);

    // * 也能夠拆箱
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("{}", name);
}

#[test]
fn open_mybox() {
    let x = MyBox::new(5);

    assert_eq!(5, *x); // 實際上編譯器這邊會編譯成 *(x.deref())
}

/// 解引用之強制轉型(多型)
fn deref_coercions() {
    let m: MyBox<String> = MyBox::new(String::from("Rust"));

    hello(&m); 
    // 由於 Deref trait, Rust 通過 deref coercions
    // &MyBox<String> -> &String, 標準庫中 &String deref -> &str

    // 若沒有 deref coercions
    hello(&(*m)[..]); // m 先 deref 變成 String, 然後取 slice 變成 str 再取 ref
}

// T: Deref<Target=U> &T -> &U
// T: DerefMut<Target=U> &mut T -> &mut U
// T: DerefMut<Target=U> &mut T -> U // 不可逆