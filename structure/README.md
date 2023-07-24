# 구조체 & 튜플 Struct & Tuple

## 구조체 Struct
- 연관된 여러 값들을 묶어서 데이터 단위 정의
- 객체의 데이터 속성과 같은 의미
- 구조체 이름과 중괄호 안에 필드라고 불리는 각 구성 요소들의 타입과 이름 정의
```
struct User {
    user: string,
    email: string,
    active: bool,
}

// 인스턴스 생성
let user = User {
    user: String::from("donghquinn),
    email: String::from("ehdgus1524@gmail.com"),
    active: true,
};
```

### 구조체 읽어오기
- 구조체 인스턴스에서 값에 접근은 . 으로 접근하면 된다.
```
// 변경 가능 인스턴스
let mut user = User {
    user: String::from("donghquinn),
    email: String::from("ehdgus1524@gmail.com"),
    active: true,
};

// 인스턴스 내 필드 값 변경
user.email = String::from("kdh970630@gmail.com");
println!(user.email);
```

## 튜플 구조체 Tuple Struct
- 구조체명을 통해 의미를 부여하나, 필드 타입만 정의할 수 있음
```
struct Color (i32, i32, i32);

let black = Color(0,0,0);
```

## 구조체 데이터의 소유권
- 구조체 필드값의 타입을 &str이 아닌 String을 사용한 것은 구조체의 소유를 위해서이다.
- 구조체 전체가 유효한 동안 구조체가 그 데이터를 소유하게 하기 위함
    - 이는 라이프타임(LifeTimes)의 사용에 의한 것
    - 구조체가 존재하는 동안 참조하는 데이터를 계속 존재할 수 있게 함.