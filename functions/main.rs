fn main() {
    println!("Hello");

    another_fn();

    let x = 5;

    // a assignment expression
    let y = {
        
        let mut x = 4;

        if (x > 3) {
            x = 1;
        }

        // assign
        x + 1
    };

    println!("y is: {}", y);

    println!("ten() : {}", ten_with_return());
    println!("ten() : {}", ten_without_return());
    println!("plus_one {}", plus_one(56));
}

// rust use snake_case_style
fn another_fn() {
    println!("Hello2");
}

fn ten_with_return() -> i32 {

    // return can be left out, see below
    return 5;
}

fn ten_without_return() -> i32 {

    // dot not write semicolon
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}