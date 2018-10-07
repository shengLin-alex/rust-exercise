fn main() {
    let s1 = String::from("fuckyou");

    take_ownship(s1);

    // this won't work, cause s1 "move" to take_ownship(s1: String)
    // println!("{}", s1);

    let x: u32 = 4;

    // this will work, cause x is a stack var, just copy
    // stack type
    // all int
    // all float
    // all bool
    // tup (but all type must be stack) eg. (u32, f64, bool)
    make_copy(x);
}

fn take_ownship(s1: String) {
    println!("{}", s1);
} // drop s1 here

fn make_copy(x: u32) {
    println!("{}", x);
}