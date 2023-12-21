## 模式匹配 match (rust 特有)，match 本身也是表达式,rust 中有许多的模式匹配

1. match 的匹配必须要穷举出所有可能，因此这里**用 \_ 来代表未列出的所有可能性**
2. match 的每一个分支都必须是一个表达式，且所有分支的表达式**最终返回值的类型必须相同**
3. **X | Y**，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
4. 只想匹配单个情况则使用`if let 匹配`
5. 条件循环`while let`
6. 忽略模式中的值，忽略单个`_`，忽略剩余`..`

## Option （rust 特有）

https://course.rs/basic/match-pattern/option.html

## if let （rust 特有）

## while let 条件循环 (rust 特有)

```rust
fn main(){

    enum Match{
        One=1,
        Two=2,
    }

    let m = Match::One;
    let num = match m {
        Match::One => 10,
        Match::Two => 20,
        _=>-10,
    }

    // num ? 10


}

```

## 模式的使用场景

1. 考虑匹配所有情况

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

```

2. 只匹配单个情况

```rust
if let PATTERN = SOME_VALUE {

}

```

3. 只要允许就一直匹配

```rust

let stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(p) = stack.pop(){
    printLn!("{:?}",p);
}

```

4. for 循环中匹配

`(x,y) in [any,any,any]` 也是一种匹配,只要存在就一直匹配

```rust
for (index, value) in v.iter().enumerate(){
    // something...
}

```

5. let 语句也是一种模式匹配

```rust
let a = 1; // 稀松平常把？

let (x,y,z) = (1,2,3); // 是否有些许感觉了呢？

```

6. 同理函数参数也是一种模式匹配

```rust
fn start(&(x,y):&(i32，i32)){

}

```

7. 多分支，以及 range 语法

```rust
let any_thing = 5;

let x = match any_thing{
    1=>10,
    2|4 => 20,
    5..20 => 30,
    'a'..'f' => 99,
    _ => -1,
}

```

8. 解构结构体

```rust
struct Point{
    x:i32,
    y:i32
};


let p = Point{
    x:20,y:39
};

let Point{x:a,y:b} = p;

```

9. 解构枚举

10. 解构嵌套结构体和枚举

https://course.rs/basic/match-pattern/all-patterns.html#%E8%A7%A3%E6%9E%84%E5%B5%8C%E5%A5%97%E7%9A%84%E7%BB%93%E6%9E%84%E4%BD%93%E5%92%8C%E6%9E%9A%E4%B8%BE

11. 解构结构体和元祖

12. 解构数组

13. 匹配守卫

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x), // 进一步匹配

    Some(x) => println!("{}", x),

    None => (),
}

```
