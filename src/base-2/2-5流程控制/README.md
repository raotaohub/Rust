## 流程控制

https://course.rs/basic/flow-control.html#%E6%B5%81%E7%A8%8B%E6%8E%A7%E5%88%B6

## 1. if else

和其他的语言无异，但是有一些 rust 独有的语法，就是 if else 可以作为表达式来返回值

```rust

fn main(){
    let num = if true { 10 } else if false { 20 } else { 30 };

    // num 10
}

```

## 2. 循环

### 2.1 for

https://course.rs/basic/flow-control.html#for-%E5%BE%AA%E7%8E%AF

- rust 的 for 循环使用时注意所有权的转移**（rust 特有）**
- rust 的 for 循环可以通过`for (i, v) in [1,2,3].iter().enumerate() `来迭代出索引和值。**（rust 特有）**

  使用方法 等价使用方式 所有权
  for item in collection for item in IntoIterator::into_iter(collection) 转移所有权
  for item in &collection for item in collection.iter() 不可变借用
  for item in &mut collection for item in collection.iter_mut() 可变借用

### continue 单次跳过（常规）

### break 打断 ，但是可以返回值 （rust 特有）

```rust
fn main(){
    let mut count = 0;
    let num = loop{
        count+=1;
        if count >= 20{
            break count;
        }
    };

    // num 20
}

```

### continue

### 2.2 while 循环 (常规)

### 2.3 loop 循环是表达式，可以返回值（rust 特有）
