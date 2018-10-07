fn main() {
    let s1 = String::from("fuckyou");
    let s2 = s1;

    // this won't work, cause s2 already reference s1, avoid twice drop, s1 is invalid now.
    // 在一些其他語言，這樣的行為為淺拷貝，由於擁有 GC，因此自動回收了變數
    // 但rust 沒有GC，按照記憶體參照，自動無效 s1，因此這邊的行為稱為 "move"
    // println!("{}", s1);

    let s3 = String::from("fuckyou");

    // use clone to deep clone.
    let s4 = s3.clone();

    println!("{}{}", s3, s4);
}