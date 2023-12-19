fn main() {
    /* 1. 流程控制 */
    /* 1. 可以配合赋值语句,但是返回值类型要一致 */

    let number = if true { 2 } else { -1 };

    /* 2. 标准的分支流程处理 */

    if number == 2 {
        2
    } else if number == -1 {
        -1
    } else {
        99
    };

    println!("{}", number);

    /* 2.循环 for while loop */

    for t in &[1, 2, 3] {
        println!("{}", t);
    }

    for t in 4..7 {
        println!("{}", t);
        if t == 4 {
            continue;
        }

        if t == 5 {
            break;
        }
    }

    /* 在循环中改变，并且保留所有权 */
    let mut arr = [6];

    for t in &mut arr {
        *t = &*t + 1;
    }
    println!("{:?}", arr)
}
