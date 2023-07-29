# 열거 Enumerations
- 하나의 타입이 가질 수 있는 값들을 열거하여 타입을 정의한다.

## 열거형 데이터 타입 선언
- IP 주소를 예로, 사용할 수 있는 표준은 v4와 v6 두가지가 있다. 이를 아래와 같이 타입 선언할 수 있다.
```
enum IpTypes{
    V4,
    V6
}
```
- 이렇게 데이터 타입을 선언할 수 있다.

## 인스턴스 화
- 선언된 데이터 타입을 인스턴스 화 시키는 방법은 아래와 같다.
```
    let ipv4 = IpTypes::V4;
    let ipv6 = IpTypes::V6;
```

## 함수 인자로
- 이러한 타입을 인자로 받게 하는 함수도 역시 간단하다.

```
fn router(ip: IpTypes){}
```

## 구조체 활용
- 구조체를 활용한다면 실제 데이터를 저장할 수도 있다.

```
enum IpTypes {
    V4,
    V6
}

struct IpAddress {
    kind: IpTypes,
    address: Strings
}

fn main() {
    let ip_address = IpAddress {
        kind: IpTypes::V4,
        address: String::from("192.102.131"),
    };
}
```