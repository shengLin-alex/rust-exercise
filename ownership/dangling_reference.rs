fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // leave scope, s is no more available, is dangerous return not available reference.

fn not_dangle() -> String {
    let s = String::from("hello");

    s
} // fine, we move s

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效。