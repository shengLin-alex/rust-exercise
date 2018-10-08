struct Foo {
    f: Box<i32>,
}

struct Bar<'a, 'b> {
    // foo: &Foo, // 錯誤, 必須加上 lifetime
    foo: &'a Foo,
    doo: &'b Foo,
}

struct Link<'a> {
    link: &'a Foo,
}

fn main() {
    let v = vec![1,3,4];

    let v2 = v;

    // 這邊會出錯, 因為按照 rust 的記憶體參照機制, v 賦予給 v2, v 與 v2 指向同一個 vector,
    // 離開作用域後為了避免重複回收記憶體, 編譯器已經先把 v 回收掉了
    // println!("{}", v);

    let v = 1;

    let v2 = v;

    // 這邊沒有問體, 因為 1 的型別是 i32, 已經實現了 Copy trait 所以會把記憶體指向的值深度複製一份到 v2
    println!("{}", v);

    // !! 所以實現了 Copy trait 的類型會被 Copy
    // 沒有實現的 會被 move!!

    // 所以如果寫了一個 function 傳遞了參數, 該 funciton 用完之後要歸還參數
    let v1 = vec![1,3];
    let v2 = vec![3,5];
    let (v1, v2, bar) = foo(v1, v2);
    println!("{:?} {:?}", v1, v2);

    // 這種特性簡直是太噁心了, 所以 rust 提供了 borrowing 的概念

    // 使用 reference 之後這樣呼叫

    // reference 字面為引用之意, 因此引用就不具有所有權, 合情又合理
    let v1 = vec![1,3];
    let v2 = vec![3,5];
    let bar = foo_use_ref(&v1, &v2);
    println!("{:?} {:?}", v1, v2);

    // reference 默認為不可修改
    // 如果要修改值, 必須使用 mut 以及 &mut
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    // reference 的規則
    // 1. 所有引用的作用域必須小於所有者的作用域
    // 2. 可以同時有多個不可變引用
    // 3. 但只能同時有一個可變引用

    // 根據上例, 如果把括號拿掉, 將會發生錯誤
    // let mut x = 5;
    // let y = &mut x; // 可變引用
    // *y += 1;
    // println!("{}", x); // 這裡試圖使用原本的可變引用綁定

    // 考慮以下情況
    // 1. 我有一個資源
    // 2. 我將資源的引用借出
    // 3. 我釋放這個資源
    // 4. 你使用這個資源

    let y: &Foo;
    // 故意加入一個作用域
    {
        let mut x = Foo {
            f: Box::new(18)
        };

        // y = &x; // 錯誤 借出 x, 然後離開作用域
    }

    // println!("{}", y.f); // 錯誤, 因為編譯器發現 reference 的 lifetime 比 owner 的 lifetime 長
    // rust 通過保證 owner 活的比它的任何一個引用活的久來確保記憶體安全
    // 因此有了 lifetime 的概念

    // 引用與 lifetime 互相關聯!
    let mut a = Foo {
        f: Box::new(14)
    };
    let d: &Foo;

    { // block start
        let mut b = Foo {
            f: Box::new(13)
        };
        let bar = Bar {
            foo: &a,
            doo: &b,
        };

        println!("{}", bar.foo.f);
        // d = bar.doo; // 錯誤, 一個 struct 有多個引用的話, 它的實例壽命只能跟最短壽命的那個引用一樣長
        // d 不能是壽命只跟 b 一樣長的 doo, 因為 d 的壽命長度為整個 main()
        d = bar.foo; // 所以這樣 ok, 因為壽命長度一樣
    } // block end

    println!("{}", d.f);

    let a = Foo {
        f: Box::new(1),
    };

    let x = &mut Link {
        link: &a,
    };

    {
        let b = Foo {
            f: Box::new(3),
        };

       // store(x, &b); 錯誤
    } // 離開後 b 被已經無效
}

/// 不使用 reference
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {

    // 在這邊使用 v1 v2
    // 然後手動歸還 v2 v2
    (v1, v2, 1)
}

/// 使用 reference 的版本
fn foo_use_ref(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {

    1
}

// fn test(a: &Foo, b: &Foo) -> &Foo { // 錯誤, 沒有加上 lifetime parameter
//     println!("a: {} - b: {}", a.f, b.f);

//     b
// }

/// 由於返回為一個 reference, 而這個 method有兩個 reference arugments, 
/// 
/// 編譯器為了避免錯誤, 認不出是從哪一個參數借用, 因此指定 lifetime parameter.
fn test<'a>(a: &Foo, b: &'a Foo) -> &'a Foo { // 加上 lifetime parameter
    println!("a: {} - b: {}", a.f, b.f);

    b
}

fn test_two<'a, 'b>(a: &'b Foo, b: &'a Foo) -> &'b Foo {
    println!("a: {} - b: {}", a.f, b.f);
    
    // 錯誤, 沒有 lifetime
    // &Foo {
    //     f: Box::new(12),
    // }
    
    // b // 錯誤, 不同的 lifetime

    a // ok
}

/// 創造一個函數來轉移延長 y 這個 reference 的 lifetime
fn store<'a> (x: &mut Link<'a>, y: &'a Foo) {
    x.link = y;
}

/// 一個簡單的 lifetime 語法例子
fn show<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}