/* 所有权原则
理解了堆栈，接下来看一下关于所有权的规则，首先请谨记以下规则：

Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
*/

fn main() {
    println!("所有权与借用");
    let a = String::from("美丽"); // 这个类型是 String ，是可变的 ，
    let b = a; // a 的值的所有权被转移到 b 上了，因此 a变量 不再可用

    let c: &str = "哈啰沃德"; // c 存储的是引用，这里是原始数据类型str的引用
    let d = c; // 这里拿到了 c 的引用 ，c存储的也是引用；

    println!("{b},{c},{d}");
    clone_str();

    let f = String::from("所有权在谁那？");
    move_ownership(f);
    // println!("{f}"); // error 所有权被转移到 move_ownership 函数了，f 不再持有 报错
    borrow()
}

fn clone_str() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn move_ownership(some_string: String) {
    println!("in move_ownership fn inner{some_string}");
}

fn borrow() {
    #[derive(Debug)]
    /* 声明解构体 */
    struct Person {
        name: String,
        age: Box<u8>,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}
