# 复合类型

## 字符串

### 1. 字符串与字符串切片

- `String` 字符串类型 通过 String 创建的 （String 是标注库中的其中之一，使用最为广泛）

- `&str` 字符串切片类型，通过`字面量方式`创建的也属于字符串切片类型

```rust
let string = String::from("你好"); // 类型为 String

let string = "阿波".to_string(); // 类型为 String

let slice = "哈啰"; // 类型为 &str

let slice2 = &string[..]; // 类型为 &str

let slice3 = &string // 类型为 &str

let slice4 = string.as_str() // 类型为 &str

```

### 深入了解 String

在 Rust 中，当你使用 `String::from("abc")` 创建一个字符串时，实际上它的底层是使用了堆内存来存储这个字符串的内容。具体来说，Rust 的 `String` 类型是一个指向堆上分配的字符序列的指针，这个字符序列存储了字符串的内容。

#### `String` 类型内部结构如下：

```rust
pub struct String {
    vec: Vec<u8>,
}
```

这里的 `Vec<u8>` 是一个动态数组，用于存储字符串的 UTF-8 编码字节序列。这意味着对于简单的字符串如 "abc"，它们将按照 UTF-8 编码被存储在内存中。UTF-8 是一种针对 Unicode 的变长度字符编码方案，因此对于 ASCII 字符（比如 "a", "b", "c"），它们的编码实际上使用一个字节（即 ASCII 编码）来存储。

因此，当你创建 `String` 时，"abc" 实际上被存储为三个连续的字节，分别是 `0x61`、`0x62` 和 `0x63`，分别对应 ASCII 编码下的 "a"、"b" 和 "c"。在堆上分配的内存中，这些字节将按照顺序存储，这个内存区域的起始地址将会被 `String` 结构体中的指针所记录。

需要注意的是，Rust 的 `String` 类型会负责对这个内存进行分配和释放，这保证了在对字符串进行操作时内存的正确管理，从而避免了产生一些常见的内存安全问题。

> 1 bit 比特 = 1 个 0，或者 1 个 1
> 1 byte 子节 = 8 个 bit
> 1 kb 千字节 = 1024 个 byte

#### String 与 Chat

- Chat 使用 Unicode 编码，因此每个字符占据 4 个字节内存空间。
- String 使用 UTF-8 编码，因此每个字符所占的字节数是变化的(1 - 4)

### 2.字符串索引

基于`[深入了解 String]` 我们可以知道，在 Rust 中，通过索引访问字符串可能会引发错误。

```rust
fn main(){

    let str = String::from("中国");

    //底层存储为 [xx1,xx2,xx3,xx4,xx5,xx6] 类型vec<u8>[]

    let slice =&str[0] // 假设取到 xx1 ， 1/3个"中"字并没有意义。
}
```

**因此 Rust 不允许通过索引访问，同样的对于&str 类型也最好不要通过这种方式**

```rust

fn main(){
    let s = String::from("中ello world");
    let word = first_word(&s); // 切片操作
}

fn first_word(s: &String) -> &str {
    &s[..1]
}


byte index 1 is not a char boundary; it is inside '中' (bytes 0..3) of `中ello world`

```

一般来说可以使用 range 语法来取

### 操作字符串 String 类型

```rust
fn main() {
    let mut s = String::from("Hello ");

    s.push_str("rust");

    s.inster("2");
    s.inster_str("3");

    s.replace("2","t")
    s.replacen("2","t",99/*要替换的个数*/);
    s.replace_range(0,2,"age");

}

```

### 字符串转义输出

- `\`+ `unicode字符`

```rust
"i am \x52\x75\x73\x74!" //输出 i am rust
```

- `\u`+`{`+`211D`+`}`

```rust
"\u{211D}" // ℝ


```

- 保持原样 `\`

```rust

println!("hello \\x52\\x75\\x73\\x74"); // hello \x52\x75\x73\x74

```

## 元祖 tup

### 声明语法

```rust
let tup:(i32, f64, u8) = (12, 4.2, 1)


```

### 模式匹配解构语法

```rust
let tup:(i32, f64, u8) = (12, 4.2, 1);

let (x, y, z) = tup;
```

### `.`操作符访问

```rust
let tup:(i32, f64, u8) = (12, 4.2, 1);

let x = tup.0;
let y = tup.1;
let z = tup.2;
```

### 函数返回值中使用元祖

```rust
let tup:(i32, f64, u8) = (12, 4.2, 1);

fn calculate_length(s:String) ->( String, usize){
    let len = s.len();
    (s, len)
}
```

## 结构体 struct

```rust

struct User {
    name: String,
    age: u32,
}

let pos = (100,300)

```

### 元组结构体(Tuple Struct)

元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用

```rust

struct Position (i32,f32)

let pos = (100,300)

```

### 单元结构体(Unit-like Struct)

TODO 不懂咋个意思
https://course.rs/basic/compound-type/struct.html#%E5%8D%95%E5%85%83%E7%BB%93%E6%9E%84%E4%BD%93unit-like-struct

```rust

```

### 结构体的所有权

如果要在结构体中使用引用，则需要在使用引入声明周期的概念，否则报错。

```diff
struct User {
-    name: String,
+    name: &str,
}
```
