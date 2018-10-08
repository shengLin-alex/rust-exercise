fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    // this will cause run time error.
    // let element = a[index];

    // println!("The value of element is: {}", element);

    let tuple: (u32, u64, f32) = (78, 1993902, 0.4);
    println!("{}{}{}", tuple.0, tuple.1, tuple.2);

    // a slice
    let x: [i32;5] = [1,3,4,5,3];
    for i in x.iter() {
        println!("{}", i);
    }
}