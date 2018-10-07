fn main() {
    let str1 = String::new();

    let s = "initial content";
    let s2 = s.to_string();
    let s3 = "initial content".to_string();

    // true
    println!("{}", s2 == s3);

    // all utf-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("fuck");
    s.push_str(" you");
    println!("{}", s);

    let mut s = "fuck".to_string();
    s.push_str(" you");
    println!("{}", s);

    let mut s = "fuck yo".to_string();
    s.push('u');
    println!("{}", s);

    // + just look like fn add(self, &str) -> String {}
    let s1 = String::from("fuck");
    let s2 = String::from(" you");

    // + "move" s1 and reference s2
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("fuck");
    let s4 = format!("{}{}", s1, s2);
    println!("{}", s4);

    let len = String::from("Hola").len();
    println!("{}", len);

    let len = String::from("Здравствуйте").len();
    println!("{}", len);

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // ok "Зд"
    // let s = &hello[0..1]; // panic

    // for loop a string better use chars()
    for c in hello.chars() {
        println!("{}", c);
    }

    // if you want get bytes
    for c in hello.bytes() {
        println!("{}", c);
    }
}