use std::rc::Rc;

enum List {
    ConsBox(i32, Box<List>),
    NilBox,
}

enum RcList {
    ConsRc(i32, Rc<RcList>),
    Nil,
}

use self::List::{ConsBox, NilBox};
use self::RcList::{ConsRc, Nil};

fn main() {
    let a = ConsBox(
        5,
        Box::new(ConsBox(
            10,
            Box::new(NilBox)
        ))
    );

    let b: List = ConsBox(3, Box::new(a));

    // let c = Cons(4, Box::new(a)); // 錯誤 a moved

    let a: Rc<RcList> = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(Nil)))));

    let b: RcList = ConsRc(4, Rc::clone(&a)); // 增加一次引用計數
    let c: RcList = ConsRc(5, Rc::clone(&a)); // 在增加一次引用計數

    print_count();
}

fn print_count() {
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(Nil)))));
    println!("count after create a {}", Rc::strong_count(&a));

    let b = ConsRc(3, Rc::clone(&a));
    println!("count after create b {}", Rc::strong_count(&a));

    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("count after create c {}", Rc::strong_count(&a));
    }

    println!("count after c leave scope {}", Rc::strong_count(&a));
}