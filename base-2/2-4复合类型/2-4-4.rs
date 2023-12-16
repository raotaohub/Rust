/* 枚举 enum */
#[derive(Debug)]
enum PokerSuit {
    Clubs(u8),
    Spades(u8),
    // 同一个枚举类型下的不同成员还能持有不同的数据类型
    Diamonds(char),
    Hearts(char),
}

enum QuitMessage {
    Quit,
    Move { x: i32, y: i32 },
    Message(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let hearts = PokerSuit::Hearts("A");
    let diamonds = PokerSuit::Diamonds("K");
    let clubs = PokerSuit::Clubs(10);

    println!("{hearts},{diamonds},{clubs}");

    // 使用枚举的好处在于。。。。，我ts这段很溜不用学了 but 这个概念rust叫 同一化类型
    let q = QuitMessage::Quit;
    let m = QuitMessage::Move { x: 3, y: 4 };
    let msg = QuitMessage::Message(String::from("msg"));
    let c = QuitMessage::ChangeColor(128, 255, 120);

    println!("{q},{m},{msg},{c}");

    let five = Some(5);
    let six = push_add(five);
    let none = push_add(None);

    println!("{five},{six},{none}");
}

fn push_add(p: Option<i8>) -> Option<i8> {
    /* 匹配模式，处理不同的情况 */
    match p {
        Some(i) => i + 1,
        None => None,
    }
}

// Option 枚举 Option 枚举用于处理空值 !!特殊的枚举，内置的

enum Option<T> {
    Some(T), // T 允许接收任意类型
    None,
}
