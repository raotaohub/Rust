# 所有权和借用

## 所有权核心概念

1. 一个值同时只能被一个变量拥有，但该值拥有 Copy 特征，则可以自动拷贝并赋值给另一个变量，否则需要手动深拷贝，否则编译失败。
2. 一个变量离开他的作用域时就会被 drop 。
3. 所有权也可以部分转移

```rust

fn main() {
    #[derive(Debug)]
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

```

## 引用与借用核心概念

https://course.rs/basic/ownership/borrowing.html#%E5%BC%95%E7%94%A8%E4%B8%8E%E8%A7%A3%E5%BC%95%E7%94%A8

1. 基于所有权，尽管一个值不可以被多个变量拥有，但是可以借用它的引用。 (ref | & 操作符)

> 应该说，获取一个值的引用，被称为借用，这是一种 rust 提出的概念

```rust
let &str school = "中国大学";
let ohter = school;

```

## 不可变引用

## 可变引用同时只能存在一个

- 以下代码报错

```rust

let mut mutable = String::from("aaaaaaaaa");
let m1 = &mut mutable;
let m2 = &mut mutable; // error
println!("{m1},{m2}");

```

- 以下通过编译 NLL 提前找到不再使用的变量

```rust

let mut mutable = String::from("aaaaaaaaa");
let m1 = &mut mutable;

println!("{}", m1); // 在这里 m1 已经释放了

let m2 = &mut mutable;  // 再引用一次可变

println!("{},", m2); // 继续使用

```

## 可变引用和不变引用不能同时存在

## 悬垂引用

https://course.rs/basic/lifetime.html

```rust

fn main(){
    let str = return_str_ref()
}

fn return_str_ref() -> &String{
    let s = String::from("值");
    &s
} // // 这里 s 离开作用域并被丢弃。其内存被释放，但是此时我们又尝试去返回它的引用。

```

> this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
> 该函数返回了一个借用的值，但是已经找不到它所借用值的来源

# 借用规则总结

- 总的来说，借用规则如下：

1. 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
2. 引用必须总是有效的
