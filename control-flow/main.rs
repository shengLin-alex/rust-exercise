fn main() {
    let cond = true;

    let x = if cond {
        5
    } else {
        6
    };

    println!("{}", x);

    let mut number = 10;

    while number > 0 {
        println!("{}\n", number);

        number -= 1;
    }

    let arr = [1,2,3,4,5,6,7,8];
    for element in arr.iter() {
        println!("{}\n", element);
    }

    for element in (1..9).rev() {
        println!("{}\n", element);
    }
}