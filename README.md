# Rust Leetcode
练习使用Rust语言刷[leetcode](https://leetcode.com/problemset/all/)算法题目

## Environment
- rustc 1.37.0
- cargo 1.37.0

## How to test
```
cargo test  // 测试所有的测试
cargo test p0007_reverse_integer  // 只测试单个模块下的测试
```

## Rust基本数据类型
Rust一切皆表达式, 表达式皆有值, 值皆有类型.

#### 布尔类型
- true
- false

#### 数字类型
- u8, u16, u32, u64, u128
- i8, i16, i32, i64, i128
- usize, isize  // i.e let a = 0usize;
- f32, f64  // i.e. let a = 1.0f64;

#### 字符类型
- char  // i.e. let ch = 't';

#### 数组类型
- [T; N]  // i.e. let arr = [0; 3];

#### 范围类型
- std::ops::Range  // i.e. (1..5)
- std::ops::RangeInclusive  // i.e. (1..=5)

#### 切片类型
- &[T], &mut[T]  // i.e. assert\_eq!(&[1, 2, 3][1..], [2, 3]);

#### 字符串类型
- &' static str  // 静态生命周期字符串,与程序生命一样持久有效

#### 指针
- 引用
    - &T, &mut T
- 原生指针
    - \* const T
    - \* mut T
- 函数指针 // 函数作为参数或者返回值, 带参数枚举体
- 智能指针 // 实现了Deref Drop 两个tarit

#### nenver类型
- ! // 表示永远不可能有返回值的计算类型, 如线程崩溃,break, continue

## Rust复合数据类型

#### 元组(Tuple)
- (T, U, M, N)  // i.e let var = (1, 2, 3);

#### 结构体(Struct)
- 具名结构体(Named-Field Struct)  // i.e. struct People { name: &'static str, age: u8, }
- 元组结构体(Tuple-Link Struct) // i.e. struct Color (i32, i32, i32);
- 单元结构体(Unit-Like Struct) // i.e. struct Empty; or struct Empty {}

#### 枚举体(Enum)
- 无参数枚举体  // i.e. enum Number { Zero, One, Two, }
- 类C枚举体 // i.e. enum Color { Red=0xff0000, Green=0x00ff00, Blue=0x0000ff, }
- 带参枚举体 // i.e. enum IpAddr { V4(u8, u8, u8, u8), V6(String), }
- enum Option<T> { Some(T), None, } //  Option枚举类型有效地避免开发出现None, 不需要自己定义实现,可以直接编程使用Some(T), None两个值.
- enum Result<T, E> { Ok(T), Err(E), } // Result枚举类型常用于错误处理, 不需要自己定义实现,可以直接编程使用Ok(T), Err(E)两个值.

#### 生命周期标志
- &'static str  // Rust把作用域也纳入了类型系统, 与程序生命一样持久有效
- &'a // 与某个作用域一致, 用作函数参数定义时


#### 联合体
- union

## Rust集合类型
#### 线性序列
- Vec
- VecDeque
- LinkedList

#### 映射表
- HashMap
- BTreeMap

#### 集合
- HashSet
- BTreeSet

#### 队列
- BinaryHeap

## 参考资料
- <<Rust编程之道>>
- https://dev.to/cad97/rust-must-know-crates-5ad8
- https://mp.weixin.qq.com/s/XYAc3wMfF50vYNnv_GzFjw
