/**
 *  https://course.rs/basic/base-type/numbers.html
 *  整数
 *  有符号类型 i 8 16 32 64 128 size
 *  无符号类型 u 8 16 32 64 128 size
 *
 *  有符号 存储范围公式  -(2n - 1) ~ 2n - 1 - 1
 *  因此 i8 可存储  -(2n7) ~ 2n7 - 1  得出范围 -127~128
 *  无符号 存储方位公式 0 ~ 2n - 1
 *  因此 u8 可存储 0~（2n8）-1 得出方位 0~255
 *  
 */

/**
 *  https://course.rs/basic/base-type/numbers.html
 *  浮点数 f 32 64
 *  f32 类型是单精度浮点型，f64 为双精度。
 *  
 */

/**
 * 位操作符
 * https://course.rs/basic/base-type/numbers.html#%E4%BD%8D%E8%BF%90%E7%AE%97
 *  
 * 位或 |
 * 位与 &
 * 异或 ^
 * 位非 !
 * 左移 <<
 * 右移 >>
 */

/**
 * 序列(Range) Rust 典中典
 * (1..=5) 1 到 5 ，包含5
 *
 */
fn main() {
    for i in (1..=5) {
        println!("{i}")
    }
}

/* 有理数和复数 */
