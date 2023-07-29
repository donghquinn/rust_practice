# Method
- 메소드 Method 는 클래스와 유사한 문법이다.
- 구조체 상에 함수를 정의한다.
- 이 구조체는 impl 키워드로 선언할 수 있다.
- 파라미터로는 &self를 입력해준다.
    - 파라미터로 &self를 갖지 않더라도 정의할 수 있다.

```
struct Square {
    length: u32,
    height: u32
}

impl Square {
    fn calculate_square(&self) -> u32 {
        self.length * self.hegith
    }
}

fn main() {
    let square = Square { length: 53, height: 32};

    if square.length == square.height {
        // 길이와 높이가 같다면 정사각형 판단하여 넓이 계산
        println!("Square: {}", square.calculate_square());
    } else {
        // 길이와 높이가 다르다면 직사각형 판단하여 넓이 계산
        println!("Rectangle: {}", square.calculate_square());
    }
}
```