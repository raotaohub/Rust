/**
 * 在语法的syntax上的区别在与 是否有分号
 */

fn main() {
    let a = 20;
    println!("{}", add(a, a)); // 有分号

    let y = { 6 + 6 }; // 一个块 也可以有返回值

    println!("{}", y); // 有分号

    let 文本 = if 2 % 2 == 0 { "A" } else { "B" };

    println!("{}", 文本) // 没分号默认返回 ()
}

fn sum(x: i32, y: i32) -> () {
    x + y;
}

fn add(a: i8, b: i8) -> i8 {
    /* 语句，没有返回值 */
    let inner_count = 1;
    /* 表达式，总是有返回值 */
    a + b + inner_count
}
