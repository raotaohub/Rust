https://course.rs/basic/method.html

## 为结构体定义方法使用 impl 关键字

1. 对象定义和方法声明是分开的
2. 声明与属性名相同的方法，视为实现该属性名的 getter
3. 关联函数 ，意思为和这个结构体相关联的函数，可以细分为 2 种
   1. 有 self 参数的方法，用`.`调用
      1. self
      2. &self
      3. &mut self
   2. 无 self 参数的方法，用`::`调用

## 枚举也可以实现方法
