#[warn(dead_code)]
enum Position {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
}

fn main() {
    let top = Position::Top;

    match top {
        Position::Top => {
            println!("top");
        }
        Position::Bottom => {
            println!("Bottom");
        }
        Position::Left => {
            println!("Left");
        }
        Position::Right => {
            println!("Right");
        }
        _ => {
            println!("Unknow");
        }
    }
}
