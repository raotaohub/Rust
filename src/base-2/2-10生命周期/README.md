## 生命周期

生命周期的概念就是一个引用能存活是时间

## 函数中的生命周期标注语法

- 标记的生命周期只是为了取悦编译器，让编译器不要难为我们

```rust

&i32 // 一个引用
&'a i32 // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用

```

- 通过一个例子来理解

```rust

fn longest<'a> (x:&'a str , y:&'a str) ->&'a str{

    if x.len() > y.len() {
        x
    }else{
        y
    }

}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        // result 的生命周期 是 string1.as_str() 和 string2.as_str()中较小的那个
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```
