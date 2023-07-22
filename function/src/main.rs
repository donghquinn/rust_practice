fn main() {
    let result = calculate_numbers(13);

    println!("Result: {}", result);

    if_else(43, 98)
}

fn calculate_numbers(x: i32) -> i32 {
    x + 54
}

fn if_else(x: i32, y: i32) {
    if x > y {
        println!("X ({}) is Bigger than Y ({})", x, y);
    } else if x < y {
        println!("Y ({}) is Bigger than X ({})", y, x);
    } else {
        println!("X ({}) and Y ({}) has the same value", y, x);
    }
}
