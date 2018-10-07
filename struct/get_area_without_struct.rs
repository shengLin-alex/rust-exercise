fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_func(width1, height1)
    );

    let rect: (u32, u32) = (35, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect)
    );
}

// use function
fn area_func(l: u32, w: u32) -> u32 {
    l * w
}

// use tuple
fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}