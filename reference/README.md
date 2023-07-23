# [참조자와 빌림](https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html)

## 참조자 Reference
- 참조자는 선언된 문자열 변수를 함수 안으로 이동시킨 후에도 해당 변수를 유효하게 하고 싶을 때 사용한다.
- 즉, 소유권을 넘기는 대신 참조자를 넘겨 인자로 활용할 수 있게 만드는 것이다.
- 따라서 소유권을 갖고 있지 않기 때문에, 스코프를 넘겨도 참조자가 가르키는 값의 메모리는 반납되지 않는다.
    - 다시 말하자면 참조자만 사라지는 것.
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s는 String의 참조자입니다
    s.len()
}   // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
    // 때문에, 아무런 일도 발생하지 않습니다.
```

## 빌림 Borrowing
- 함수의 파라미터로 참조자를 받는 것을 빌림이라 한다.
- 실생활에서 빌린 물건을 고치거나 개조하는 것은 예의가 없는 것이다.
- 비슷하게, 빌린 참조자 역시 불변 변수이므로 변경하는 것이 불가능하다.
- 만일 빌린 참조자를 변경하고 싶다면 가변 참조자로 바꾸면 된다.
    - 가변 참조자는 &mut <변수 이름>
```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
- 다만 한 스코프 내에서 특정 데이터 조각에 대한 가변 참조자는 "딱 하나"만 만들 수 있다.
    - 컴파일 타임에 데이터 레이스(data race)를 방지하기 위함
    - 데이터 레이스는 의도하지 않은 동작을 유발함
    - 런타임에 이를 진단하고 추적하고 해결하기가 어려움
- 데이터 레이스 발생 조건
    - 두 개 이상의 포인터가 동시에 같은 데이터에 접근할 때
    - 그 중 적어도 하나의 포인터가 데이터를 사용할 때
    - 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없을 때

- 아래는 에러가 발생할 예시
```
let mut s = String::from("hello");

// 특정 데이터 조각에 대해 가변 참조자는 특정 스코프 내에서 하나만
let r1 = &mut s;
let r2 = &mut s;
```

## 댕글링 참조자(Dangling Reference)
- 댕글링 포인터란, 어떤 메모리를 가르키는 포인터를 보관하는 동안, 그 포인터가 가르키는 메모리가 해제되어 다른 객체에게 이미 사용하도록 줘버린 메모리 포인터를 의미한다.
- RUST는 어떤 참조자가 생성되었다면, 컴파일러는 그 참조자가 스코프 밖으로 벗어나기 전에는 데이터가 스코프 밖으로 벗어나지 않음을 보장한다.
- 아래에는 댕글링 참조자 발생에 대한 예시
```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}   // 이 스코프를 벗어나면 s 는 drop된다. 그런데 s의 참조자는 반환된다.
```