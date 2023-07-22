fn main() {
    println!("Hello, world!");

    let x = 32;
    let y = 64;

    println!("X+Y = {}", x + y);

    tuple()
}

fn tuple() {
    // 서로 다른 타입의 숫자형 튜플
    let tuples: (i32, f64, u32) = (100, 0.5, 2);

    // 같은 타입의 숫자형 튜플로 연산까지 -> 벡터에 사용하기 좋을 듯
    let t: (f64, f64, f64) = (100., 0.5, 2.);
    let (x, y, z) = t;

    let calculateResult = (x * y) * z;

    println!("Tuple Calculate Result: {}", calculateResult);
}
