## 1. Vector 动态数组集合

### 1. Vector 的创建方式

```rust
// 1.关联函数创建
let mut v = Vector::new();
v.push(1);
v.push(2);
v.push(3);

// 2.创建固定长度的
let mut v1 = Vector::with_capacity(3);
v1.push(1);
v1.push(2);
v1.push(3);

// 3.宏，创建时初始化
let mut v2 = vec![1, 2, 3];
```

### 2. 读取 Vector

1. `[index]` 下标读取，有越界风险
2. `.get(index)` 方法读取 ，返回的是 Some(T)

### 集合的借用案例

```rust

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

// 这段代码会报错，原因在于，v.push之后，可能由于容量不够，重新分配了新的内存，那么此时first就会指向旧引用
// 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。
println!("The first element is: {first}");
```

## 2. HashMap 哈希表集合
