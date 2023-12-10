// 修复下面代码的错误并尽可能少的修改
fn main() {
    /*  */
    let x: i32 = 0; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x);

    /*  */
    let mut x = 1;
    x += 2;

    println!("x = {}", x);

    /*  */
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y);

    println!("{}, world", define_x());
    println!("{:?}, world", define_x2());
}

fn define_x<'a>() -> &'a str {
    let x = "hello";
    x
}

fn define_x2() -> &'static str {
    let x = "hello";
    x
}
