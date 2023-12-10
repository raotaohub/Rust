/**
 * & 符号表示取变量的引用。
 * 在 Rust 中，这被称为“引用（reference）”，
 * 它允许你创建一个指向值的引用而不是将值本身传递到函数或其他代码块中。
 * 它支持通过借用（borrowing）的方式访问值，而不会转移所有权。
 *
 * std::mem::size_of_val 是标准库中的一个函数，用于返回给定值所占用的内存大小，以字节为单位。
 * 在你的例子中，std::mem::size_of_val(&x) 计算的是变量 x 所指向的值的大小，即字符 '中' 所占用的内存大小。
 */

fn main() {
    /* 字符 */
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
    /* 布尔 */
    let _t: bool = true;
    let f: bool = true;

    if f {
        println!("这是段毫无意义的代码");
    }

    /* 单元 不占用内存*/
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    // 修改一行让代码正常打印
    let c1 = "中";
    print_char(c1);
}

fn print_char(c: &str) {
    println!("{:?}", c);
}
