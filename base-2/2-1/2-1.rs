fn main() {
    println!("第一章");

    /* 声明数值，以及类型 */
    let a: i32 = 10;
    let b = 30i32;
    let c = 99_i32;
    let d = 120;

    /* 值可变，值类型不可变 */
    let mut e = 100;

    let res = add(add(a, b), add(c, d));
    println!("{res}")
}

fn add(a: i32, b: i32) -> i32 {
    /* 注意这里表达式的运用，（没有分号;）隐式代表retrun a + b 的值 */
    a + b
}

/* rust概念所有权，变量绑定，而非赋值，通过将一个对象绑定到一个对象上 */

/* 可变 与 不可变 */
fn mut_and_disMut() {
    /* error */
    let a = 9;
    a = 90;

    /* success */
    let mut b = 10;
    b = 100
}

/* 下划线变量，来忽略编译器警告，同ts */

fn variable() {
    let _nibaba = 10; // success
    let nimama = 20; // warning
}

/* 变量解构绑定 */
fn rest() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("{a}-{b}");
    b = true;

    assert_eq!(a, b);
}

/* 解构式赋值 */
struct Struct {
    e: i32,
}

/* https://course.rs/basic/variable.html#%E8%A7%A3%E6%9E%84%E5%BC%8F%E8%B5%8B%E5%80%BC */
fn rest_assign() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);

    [c, .., d, _] = [1, 2, 3, 4, 5];

    Struct { e, .. } = Struct { e: 5 };
}

/* 常量 */

fn const_variable() {
    const TITLE: str = "编辑";
}

/* 变量遮蔽 */
fn variable_shadow() {
    let a = 10;
    let a = a + 20;
    {
        let a = a + 30; // success a = 10 + 20 + 30

        a = 999; // error
    }
}
