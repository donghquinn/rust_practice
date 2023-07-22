fn main() {
    let result = calculate_numbers(13);

    println!("Result: {}", result);

    if_else(43, 98);

    for_test();

    loop_test();

    while_test();
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

fn loop_test() {
    let mut count = 0;

    loop {
        println!("LOOP!!!");

        count += 1;

        if count == 10 {
            println!("Count 10. Stop the Loop");
            break;
        }
    }
}

fn while_test() {
    let mut count = 0;

    while count < 10 {
        println!("Same Pattern with Loop");

        count += 1;
    }

    println!("Count 10 stop while");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
