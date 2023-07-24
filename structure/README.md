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
