fn main() {
    println!("引用与解引用 & *");

    let x = 5;
    let y = &x; // y 变量借用了 x 的引用； & 操作符

    assert_eq!(5, *y); // y 变量解了 x 的引用；* 操作符
    assert_eq!(5, x);

    println!("不可变引用");

    let s1 = String::from("hello");
    let len = get_length(&s1);

    println!("s1:{s1},len:{len}");

    println!("可变引用 mut");

    let mut s2 = String::from("hello"); // 1. 声明可变类型

    write_str(&mut s2); // 2. 创建可变引用 s2
    let len2 = get_length(&s2);
    println!("s2:{s2},len2:{len2}");

    {
        println!("特定作用域可变引用同时只能存在一个");

        /*

        let mut mutable = String::from("aaaaaaaaa");
        let m1 = &mut mutable;

        println!("{}", m1);

        let m2 = &mut mutable; // error

        println!("{},", m2);

        */
    }

    {
        println!("可变引用与不可变引用不能同时存在，也不能同时存在2个可变引用");

        let mut good = String::from("good");
        let g1 = &good;
        let g2 = &good;

        println!("{g1}, {g2}");
        let g3 = &mut good; // error

        println!("{g3}");
    }
}

fn get_length(str: &String) -> usize {
    str.len()
}

fn write_str(str: &mut String) //3. 声明接受可变引用
{
    str.push_str(" world");
}
