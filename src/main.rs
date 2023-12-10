fn greet_world() {
    let chinese = "你好世界";
    let english = "hello world";
    let regions = [chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn what_are_you_doing() {
    let i_am_a_paragraph = "\
    多少楼台烟雨中,60
    啊情深深雨蒙蒙,33
    锄禾日当午,汗滴禾下土,65
    东临碣石以观沧海,水何澹澹山岛竦峙,36
    一二三四,66
    ";

    let lines = i_am_a_paragraph.lines();

    for (i, record) in lines.enumerate() {
        /* 跳过第1个迭代 || 跳过空字符 */
        // println!("数字{},值:{}", i, record);
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        /* 分割并组成一个string集合 类似js '1,2,3'.split(',').map(t=>t.trim()) 吗🐴 */
        let fields: Vec<_> = record.split(",").map(|f| f.trim()).collect();

        /* 条件编译 */
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        /* 取出集合中的第1个 */
        let name = fields[0];

        /* 在我目前的理解就是正则赋值+运行，若成功则赋值给 length 并执行块中的语句 */

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("名字--{},数字--:{}", name, length)
        }
    }
}

fn main() {
    greet_world();
    what_are_you_doing();
}
