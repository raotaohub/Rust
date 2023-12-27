use utf8_slice;

fn slice_str_test() {
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // Will equal "🚀"
}

fn main() {
    let my_name = "Pascal";
    greet(my_name);

    let s = String::from("hello world");

    let word = first_word(&s); // 切片操作

    // s.clear(); // let mut s 然后操作可变，就会报错！ error!

    println!("the first word is: {}", word); //

    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");

    slice_str_test();
}

fn greet(name: &str) {
    println!("Hello, {}!", name);

    /* 遍历 Unicode 的方式使用 chars方法 */
    for c in "中国人".chars() {
        println!("{}", c);
    }

    /* 遍历子节 */

    for b in "中国人".bytes() {
        println!("{}", b);
    }
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
