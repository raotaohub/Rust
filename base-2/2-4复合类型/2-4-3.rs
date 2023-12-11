/* 结构体 */

struct User {
    name: String,
    age: i8,
    active: bool,
    email: String,
    sign_in_count: u64,
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        age: 0,
        active: false,
        email,
        sign_in_count: 0_u64,
    }
}

fn main() {
    /* 初始化实例 */
    let mut user1 = User {
        name: String::from("Rust"),
        age: 10,
        active: false,
        email: String::from("Rust@outlook.com"),
        sign_in_count: 0_u64,
    };
    /* 不能能够单独声明某个字段可修改，只能整个结构体声明可变 */

    /* 访问字段 */
    user1.name = String::from("raotao");

    /* 结构体更新语法 */

    let mut user2 = User {
        name: String::from("xiaoli"),
        ..user1
    };

    /* 此时 user1内部，不具有 Copy 特征的字段，所有权会转移到 user2  */

    /* 此时访问 user 会报错 */
    /* 但是可以访问其他所有权未被转移的字段 */

    println!("{}", user1.age)
}
