fn main() {

    // must use mut let x can be reassign.
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;
    println!("The value of x is: {}", x);

    let y = 9;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // this is not allow.
    // let mut spaces = "    ";
    // spaces = spaces.len();

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces: {}", spaces);
}