// rust 的 & 用於引用變數，但不取其所有權(not move), c++ 則引用變數本身的位址
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("ass");
    change_mut(&mut s2);

    let mut s3 = String::from("bitch");
    let s3r = &mut s3;

    // this won't work, cannot &mut twice in some scope.
    // let s3r1 = &mut s3;

    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change(some_string: &String) {

    // this won't work, cause change() borrow some_string by a reference
    // must use "&mut"
    // some_string.push_str(", world");
}

fn change_mut(some_string: &mut String) {

    // this won't work, cause change() borrow some_string by a reference
    // must use "&mut"
    some_string.push_str(", world");
}

fn mut_in_new_scope() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}

fn borrow_mut_at_same_time_with_immut() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    // won't work, because we borrow immut above.
    let r3 = &mut s; // BIG PROBLEM
}