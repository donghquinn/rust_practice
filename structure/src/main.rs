struct User {
    user: String,
    email: String,
    is_active: bool,
}

struct Rectangle {
    width: f64,
    hight: f64,
}

fn main() {
    let user_info = set_user(
        String::from("ehdgus1524@gmail.com"),
        String::from("donghquinn"),
    );

    // println!("User Info: {:#?}", user_info);
    println!("User Email: {}", user_info.email);

    println!("Square Value: {}", area((32., 54.)));

    let rectangular = Rectangle {
        width: 32.,
        hight: 54.,
    };

    // rectangular 복사
    println!("Rectangular Area Value: {}", area_2(&rectangular));
}

fn set_user(email: String, user: String) -> User {
    User {
        user,
        email,
        is_active: true,
    }
}

// 튜플을 이용한 코드
fn area(dimensions: (f64, f64)) -> f64 {
    dimensions.0 * dimensions.1
}

fn area_2(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.hight
}
