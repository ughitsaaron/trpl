#[derive(Debug)]
struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: 40,
        height: 22,
    };

    println!("the area of the rectangle is {}", rect1.area());
}
