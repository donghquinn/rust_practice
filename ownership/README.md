# [소유권](https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html)
- RUST의 핵심 기능
- 기존의 프로그래밍 언어들은 메모리 관리에 애를 먹는다.
    - 더 이상 사용하지 않는 메모리를 찾는 가비지 콜렉션을 가지고 있다.
    - 메모리를 할당하고 해제시켜야 한다.
- 러스트는 소유권 시스템을 통해 메모리를 관리한다.