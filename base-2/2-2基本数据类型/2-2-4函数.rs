#![allow(unused_variables)]
/**
 * https://course.rs/basic/base-type/function.html#rust-%E4%B8%AD%E7%9A%84%E7%89%B9%E6%AE%8A%E8%BF%94%E5%9B%9E%E7%B1%BB%E5%9E%8B
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

/* 返回i8 */
fn add(a: i8, b: i8) -> i8 {
    let inner_count = 1;
    a + b + inner_count
}

/* 永不反回 */
fn forever(a: i8, b: i8) -> ! {
    loop {}
}
