fn main() {
    let s1 = String::from("fuckyou");

    // 當需要獲取字串長度但又必須留住原始字串值，可以使用 tuple 的方式
    let result = get_len(s1);

    println!("{}{}", result.0, result.1);
}

fn get_len(string: String) -> (String, usize) {
    let length = string.len();

    (string, length)
}