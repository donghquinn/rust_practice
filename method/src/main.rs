struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn calculate(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let size = Rectangle {
        width: 40.0,
        height: 37.0,
    };

    println!("Size Calculate: {}", size.calculate());
}
