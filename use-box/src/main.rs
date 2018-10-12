/// 建立一個遞迴數據類型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

fn main() {
    boxing();

    create_recusive_type();
}

fn boxing() {
    let b: Box<u32> = Box::new(32);

    println!("{}", b);
}

fn create_recusive_type() {
    let list = Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(Nil)
            ))
        ))
    );

    println!("{:?}", list);
}