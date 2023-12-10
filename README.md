# $ shell

```shell
cargo run

cargo build
```

## ohter args

```shell

cargo run --release
cargo build --release

```

- cargo check 检查代码

## Cargo.toml 和 Cargo.lock

# syntax

1. for

2. continue

3. 声明变量 声明类型

4. if let Ok(length) 判断+正则声明赋值

5. 类型标注 xxx::<f32>

6. 条件编译 if(debug_asserions) { ... }

# 命令行代理

```shell
export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7891

```

# cargo 源头

```shell
crm test 测试
crm best：自动换成国内最快的镜像（我这里是sjtu）
crm publish：用官方源执行cargo publish（对于crate贡献者很有用，在开着镜像的时候不能publish）
```

# 编译器属性标记

https://course.rs/profiling/compiler/attributes.html

- #![allow(unused_variables)]

  > 属性标记，该标记会告诉编译器忽略未使用的变量，不要抛出 warning 警告,具体的常见编译器属性你可以在这里查阅
  > !表示整个文件有效，可以去除表示只对当下块有效

- #[allow(dead_code)]
  ```rust
  fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
  unimplemented!()
  }
  ```
