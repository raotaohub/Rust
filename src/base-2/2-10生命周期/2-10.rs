use std::collections::HashMap;

fn main() {
    let vec = vec![("中国队".to_string(), 100)];

    println!("{:?}", vec);

    /* 直接将其他类型的数据转成集合，但是需要写类型注解 */
    let map: HashMap<_, _> = vec.into_iter().collect();

    println!("{:?}", map);

    let mut map2 = HashMap::new();
    map2.insert("你好哈希", 200);
    map2.insert("你好集合", 299);

    println!("{:?}", map2);
    /* 查询哈希 */
    println!("{:?}", map2.get("你好哈希")); // get返回值是 Option<&T>的类型

    /* 更新哈希 */
    map2.insert("你好哈希", 999); // 1. 覆盖已有的值
    map2.entry("你好更新").or_insert(333); // 2.1 若不存在则插入新值
    map2.entry("你好更新").or_insert(222); // 2.2 若不存在则插入新值
    println!("{:?}", map2);
}
