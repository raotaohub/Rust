struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn width(&self) -> i32 {
        self.width
    }
}
fn main() {
    let rect = Rect {
        width: 30,
        height: 20,
    };

    let width = rect.width();

    println!("{width}");
}
