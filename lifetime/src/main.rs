use std::fmt::Display;

fn main() {
    let string1 = String::from("123");
    let string2 = "1234";
    let result = longer(&string1.as_str(), &string2);
    println!("{}", result);

    let string1 = "123";

    {
        let string2 = String::from("1234");
        let result = longer(&string1, &string2.as_str());
        println!("{}", result) // 有效, string1, string2 皆為有效引用
    }

    let string1 = "123";
    let result;
    {
        let string2 = String::from("1234");
        result = longer(&string1, &string2.as_str());
    }
    // println!("{}", result); // 錯誤, 因為 string2 的壽命比 string1 短, 但回傳的 ref 壽命卻比較長

    let mut x:i32 = 3;
    use_ref(&mut x);
    println!("{}", x);
}

fn use_ref(x: &mut i32) {
    *x -= 1;
}

fn foo() {
    let r;                // -------+-- 'a
                          //        |
    {                     //        |
        let x = 5;        // -+-----+-- 'b
        r = &x;           //  |     |
    }                     // -+     |
                          //        |
    // println!("r: {}", r); //     |
}                         // -------+ // 數據比引用的壽命短

fn foo2() {
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+ // 數據比印用的壽命長

// 比較字符串 slice 長度
// 這個函數無效, 因為編譯器不知道將回傳的引用是指向 x ref, 還是 y ref
// 無法確定這兩個 ref 的生命週期
// fn longer(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

/// 因此需加上 lifetime parameter
/// 表示兩個參數的生命週期將綁定
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn bar<'a>(s: &'a str, y: &str) -> &'a str {
    s // y 不用加上 lifetime 因為, 它跟回傳值得生命週期無關
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // result.as_str() // 無法回傳與生命週期無關之值, 因為 result 離開作用域之後會無效

    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

/// 跟下面一樣
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 跟上面一樣
fn first_word_two<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn static_str() {

    // 這是一個 static 的 str, 將在整個程式中存在, 直到程式結束
    let s: &'static str = "123";
}

/// 泛型參數與 lifetime 一起使用
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct Foo<'a> {
//     x: &'a i32,
// }

// fn main() {
//     let f : Foo;
//     let y;
//     // let y = &5; ok
//     {
//         y = &5;
        
//     }; // f 的 

//     f = Foo { x: y };
//     println!("{}", f.x);
// }