// Rust 的測試預設為並行
// 因此應避免測試之間互相依賴

// cargo test [test-name]
// 可以透過指定 test 方法名稱過濾
// cargo test -- --test-threads=1 使用單執行序
// cargo test -- --nocapture 不捕獲其他的 console 輸出
// cargo test -- --ignored 測試被指定忽略的測試方法

extern crate unit_test;

use unit_test::shapes::Rectangle;

// mod common;

fn print_and_return(a: &i32) -> i32 {
    // cargo test -- --nocapture 可以顯示方法中的輸出
    println!("I got {}", a);

    100
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_panic() {
    panic!("fuck you");
}

#[test]
fn larger_can_hold_smaller() {
    let a = Rectangle {
        length: 10,
        width: 43,
    };

    let b = Rectangle {
        length: 3,
        width: 3,
    };

    assert!(a.can_hold(&b));
}

#[test]
fn greeting_contains_name() {
    let result = "hello";
    assert!(
        result.contains("fuck"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[test]
#[should_panic(expected = "boom! 101.")] // 預期的 panic 訊息
fn guess_value_greater_than_100() {
    let g = unit_test::Guess::new(101);
}

#[test]
fn this_pass() {
    let value = print_and_return(&3);
    assert_eq!(100, value);
}

#[test]
fn this_not_pass() {
    let value = print_and_return(&3);
    assert_eq!(101, value);
}

#[test]
#[ignore]
fn ignore_this() {
    // this will ignore
}

/// 根據 Rust 的成員存取機制, 可以測試 private method
/// 但是不能是外部模組的私有方法
#[test]
fn test_private() {
    // assert_eq!(unit_test::Guess::private_function(), 3);
}