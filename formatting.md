# Formatting
## println!
- 다양한 종류의 포맷 출력 기능
### {}
- 문자열 내의 {}는 Display라는 포매팅을 이용하라는 의미
    - 직접적인 최종 사용자가 사용하도록 의도된 출력

```
    let name = "donghquinn";
    println!("HI my name is {}", name);
```
### {:?}
- :?는 debug 포맷을 출력하는 의미
    - 구조체를 출력할 뗴 그냥 {}를 사용해서는 에러가 발생함
    - {:?}는 구조체를 출력 문구 상에서 확인 가능
        - 다만 이를 위해선 어노테이션을 통해 미리 디버깅 동의를 해 주어야 함
    ```
        #[derive(Debug)]
        struct Rectangle {
            length: u32,
            width: u32,
        }

        let rect = Rectangle { length: 80, width:30 };
        println!("rect is {:?}", rect);
    ```