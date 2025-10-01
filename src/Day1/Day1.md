# Rust 学习笔记 Day1

作者：佟雨泽
创作时间：2025-10-01



## 基础语法

### 变量声明
```rust
let a = 5;       // 自动推断类型
let b = 5isize;  // 指定isize类型
let b = 5usize;  // 指定usize类型
```

### 数据结构

#### 元组 (Tuple)
```rust
let tuple1 = (1, 2.2, 3isize);  // 声明元组
let (x, y, z) = tuple1;         // 解构元组
```

#### 数组 (Array)
```rust
let mut a: [isize; 5] = [1, 2, 3, 4, 5];  // 声明可变数组
let b = [3; 5];                           // 声明包含5个3的数组
```

## 循环控制

### for 循环
```rust
// 基本for循环
for i in 0..5 {
    println!("{}", i);
}

// 使用迭代器
for i in a.iter() {
    println!("{}", i);
}

// 带索引的迭代
for (i, &item) in a.iter().enumerate() {
    println!("{} {}", i, item);
}
```

### while 循环
```rust
while a[0] < 5 {
    println!("{}", a[0]);
    a[0] += 1;
}
```

### loop 循环
```rust
// 基本无限循环
let mut i = 0;
loop {
    println!("loop");
    i += 1;
    if i == 5 {
        break;
    }
}

// 带标签的循环（用于嵌套循环跳出）
'outer: loop {
    'inner: loop {
        println!("inner");
        break 'outer;  // 跳出外层循环
    }
}

// 带返回值的循环
let loop_result = loop {
    println!("loop");
    break 5;  // 返回值为5
};
println!("loop_result:{}", loop_result);
```

## 所有权系统 (Ownership)

Rust 的核心概念之一是所有权系统，它在编译时确保内存安全。

```rust
// String 类型存储在堆上
let s1 = String::from("hello");
let s2 = s1;  // 浅拷贝，s1的所有权转移给s2，s1失效

// 克隆数据而非转移所有权
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝，s1仍然有效

// 栈上的数据（如整数）直接复制
let x = 5;
let y = x;  // x仍然有效，因为整数实现了Copy trait
```

## 引用和借用 (References & Borrowing)

```rust
let mut s1 = String::from("hello");
let s2 = &s1;          // 不可变引用
let s1_mut = &mut s1;  // 可变引用

// 同一作用域内只能有一个可变引用，避免数据竞争
```

### 函数参数传递
```rust
// 获取所有权，传入后原变量失效
fn testFunction(x: String) {
    println!("{}", x);
}

// 借用，不获取所有权
fn testFunction2(x: &String) {
    println!("{}", x);
}

// 可变借用，可以修改数据
fn testFunction3(x: &mut String) {
    x.push_str(" world");
    println!("{}", x);
}
```

## 字符串切片 (String Slices)

```rust
let s1 = String::from("hello world");
let s2 = &s1[0..];   // 整个字符串
let s3 = &s1[..5];   // 前5个字符
let s4 = &s1[..];    // 整个字符串
let s5 = "asd";      // 字符串字面量，实际是&str类型
```

## 示例：查找第一个单词

```rust
fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    
    // 遍历字符串的每个字节，查找第一个非空格字符
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            continue;
        } else {
            return i;  // 返回第一个非空格字符的位置
        }
    }
    
    s.len()  // 如果全是空格，返回字符串长度
}
```

## 猜数字游戏示例

这是一个完整的猜数字游戏实现：

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn init_num() -> i32 {
    rand::thread_rng().gen_range(1, 100)
}

fn str_to_num(a: String) -> Result<i32, std::num::ParseIntError> {
    a.trim().parse::<i32>()
}

pub(crate) fn main() {
    let init_num = init_num();
    loop {
        println!("请输入数字");
        let mut enter_num = String::new();
        io::stdin().read_line(&mut enter_num).expect("读取输入失败");
        
        match str_to_num(enter_num.clone()) {
            Ok(enter_num) => {
                match enter_num.cmp(&init_num) {
                    Ordering::Less => println!("数字太小"),
                    Ordering::Greater => println!("数字太大"),
                    Ordering::Equal => {
                        println!("恭喜你猜对了");
                        break;
                    }
                }
            },
            Err(_) => {
                println!("输入格式不正确，请输入一个有效的数字！");
                continue;
            }
        }
    }
}
```

这个程序展示了：
- 随机数生成
- 用户输入处理
- 错误处理（Result类型）
- 模式匹配（match表达式）
- 模块引入和使用

## 关键概念总结

1. **所有权**：每个值都有一个所有者，同一时间只能有一个所有者
2. **借用**：通过引用使用值而不获取所有权
3. **生命周期**：确保引用始终有效
4. **切片**：引用集合中的一部分数据

这些概念构成了Rust内存安全保证的基础，无需垃圾回收机制即可防止悬挂指针和数据竞争。