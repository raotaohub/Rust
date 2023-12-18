/**
 * 数组 array 存在在栈，长度固定
 * 动态数组 Vector，存放在堆，长度可动态伸缩
 * [T;n]语法
 * [value;n]语法,但是，对于非基本类型而言将会报错。
 *
 */
fn main() {
    let arr: [u8; 2] = [1, 2]; // [T;len]语法

    println!("{:?}", arr);

    let arr2 = [33; 2];

    println!("{:?}", arr2);

    let arr3: [String; 2] = [String::from("1"), String::from("2")];
    println!("{:?}", arr3);

    /* 非基本类型，无法使用[value;n]语法直接构造n个value的数组 */
    /* let arr4: [String; 2] = [String::from("hello");2] error*/
    let arr4: [String; 2] = std::array::from_fn(|_i| String::from("hello"));
    println!("{:?}", arr4);

    let one = [1, 2, 3, 4];
    let two: [u8; 4] = [1, 2, 3, 4];
    let blank1 = [1; 4];
    let blank2: [u8; 4] = [8; 4];

    let arrays: [[u8; 4]; 4] = [one, two, blank1, blank2];
    println!("{:?}", arrays);

    for t in arrays {
        println!("{:?}", t);

        /* iter() */
        /*
           [T;n]描述了一个数组的类型，而[T]描述了切片的类型，
           因为切片是运行期的数据结构，它的长度无法在编译期得知，
           因此不能用[T;n]的形式去描述
        */
        for k in t.iter() {
            println!("{:?}", k);
        }
    }
}
