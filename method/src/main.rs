struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn calculate(&self) -> f64 {
        self.width * self.height
    }

    fn is_square(&self) {
        if self.width == self.height {
            println!("It's Square: {}", self.calculate());
        } else {
            println!("It's Rectangle: {}", self.calculate());
        }
    }
}

fn main() {
    let size = Rectangle {
        width: 40.0,
        height: 37.0,
    };

    size.is_square();
}
