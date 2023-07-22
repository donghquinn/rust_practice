fn main() {
    println!("Hello, world!");

    let x: i32 = 32;
    let y: i32 = 64;

    println!("X+Y = {}", x + y);

    tuple();

    arrays();
}

fn tuple() {
    // 서로 다른 타입의 숫자형 튜플
    let tuples: (i32, f64, u32) = (100, 0.5, 2);

    // 각각 튜플에 접근하는 방법
    println!("Each Tuples: {}, {}, {}", tuples.0, tuples.1, tuples.2);

    // 같은 타입의 숫자형 튜플로 연산까지 -> 벡터에 사용하기 좋을 듯
    let t: (f64, f64, f64) = (100., 0.5, 2.);
    let (x, y, z) = t;

    let calculate_result: f64 = (x * y) * z;

    println!("Tuple Calculate Result: {}", calculate_result);
}

fn arrays() {
    let array: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!(
        "Access to Array Values: 0 - '{}', 1 - '{}', 4 - '{}', -1 - '{}'",
        array[0],
        array[1],
        array[4],
        array[array.len() - 1]
    );
}
