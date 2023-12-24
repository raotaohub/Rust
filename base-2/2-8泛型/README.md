## 1. 泛型 Generics

1. 泛型在使用的时候，要根据其使用特性，来加以限制.

```rust
// T 用于 add ，因此要限制 T 具备该特性
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

2. 泛型可用于结构体

```rust

struct Point<T,U>{
   x:T,
   y:T,
   z:U
}
```

3. 可用于枚举

4. 可用于方法

5. 可为具体的泛型类型，实现特定方法

## const 泛型（重要特性,基于值的泛型参数）

原因：以数组为例，在 rust 中，值类型相同，而长度不同的数组类型，并不是同一个类型。
比如`[i32,2]`和`[i32,5]`就不是同一个类型。那么带来的问题就是，他们的方法需要分别实现。

```rust
fn dispaly_array<T:std::fmt::Debug,const N :usize>(arr:[T,N]){
   printLn!("{:?}",arr)
}

```

---

## 2. 特征 trait (和其他语言中的接口概念类似)

### 1.定义特征 (行为)

```rust
pub trait Summary{
   fn summarize(&self) -> String;
}

```

也可以定义默认实现

```rust
pub trait Summary{
   fn summarize(&self) -> String {
      String::from("阿巴阿巴")
   };
}
```

### 2. 为类型实现特征

使用 `impl 结构体 for 特征`的语法，来实现特征

```rust
struct Weibo{
   title:String,
   content:String
}

impl Weibo for Summary{
   // 1. 可以选择重载这个特征
   fn summarize(&self) -> String{
      String::from("文章的标题是"+self.title)
   }
   // 2. 不重载，则使用默认实现
}

```

### 3. 特征作为函数参数 和特征约束（类似 ts 的类型，但是单独指类型中的函数）

有以下 2 中写法是等价的

```rust

pub fn nofity(&item: impl Summary){
//...
}

pub fn nofity<T: Summary>(item:&T){
//...
}

```

### 4. 多重约束

````rust

pub fn nofity(item: &( impl Summary + Display)){
//...
}

pub fn nofity<T: Summary + Display>(item:&T){
//...
}



### 5. where 约束（rust 特有）

约束一个类型的需要实现多个特征，例如

```rust
fn display_name<T: Add + Clone> (item:&T){
   // ...
}

````

也可以写成如下

```rust
fn display_name<T,U> (item:&T,other:&U) -> i32
   where T: Add + Clone,
         U: Add + Display
{
   // ...
}

```

### 6. 有条件的实现方法或者特征（rust 特有）

```rust

```

### 7. impl Trait 作为函数返回值（rust 特有）

### 8. derive 派生特征，使用 Rust 默认提供的特征（rust 特有）

https://course.rs/basic/trait/trait.html#%E8%B0%83%E7%94%A8%E6%96%B9%E6%B3%95%E9%9C%80%E8%A6%81%E5%BC%95%E5%85%A5%E7%89%B9%E5%BE%81

## 3. 特征对象

https://course.rs/basic/trait/trait-object.html#%E7%89%B9%E5%BE%81%E5%AF%B9%E8%B1%A1%E5%AE%9A%E4%B9%89
