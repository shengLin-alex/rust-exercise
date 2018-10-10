extern crate closure;

use closure::cacher::Cacher;

#[test]
fn call_with_different_values() {
    let mut cal = Cacher::new(|a| a);

    // 經過 Cacher 將暫存值之機制改為使用 hashmap 之後, 此測試便可以通過
    let v1 = cal.value(1);
    let v2 = cal.value(2);

    assert_eq!(2, v2);
}

#[test]
fn capture_outside() {
    let x = 4;

    let equal_to_x = |z| z == x; // 閉包可以捕獲外部變數

    // fn equal_to_x_fn(z: i32) -> bool {
    //     z == x // inner function 不能捕獲外部變數
    // }

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn move_to_capture_outside() {
    let x = vec![1,2,3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); // 錯誤, 因為 x 被 move 了

    let y = vec![1,2,3];

    assert!(equal_to_x(y)); 
}