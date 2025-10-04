# Rust 学习笔记 Day2

作者：佟雨泽
创作时间：2025-10-03

## 结构体 (Struct)

结构体是 Rust 中自定义数据类型的一种方式，允许我们命名和包装多个相关的值。

### 定义结构体

```rust
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: i32,
    is_verified: bool
}

struct Rectangle {
    width: usize,
    height: usize
}
```

使用 `#[derive(Debug)]` 可以让结构体支持调试打印。

### 创建结构体实例

```rust
// 创建一个不可变的结构体
let User1 = User {
    name: "".to_string(),
    email: String::from("<EMAIL>"),
    age: 18,
    is_verified: true
};

// 创建一个可变结构体
let mut User2 = User {
    name: String::from("Rahul"),
    email: String::from("<EMAIL>"),
    age: 18,
    is_verified: true
};
```

### 更新结构体语法

使用 `..` 语法可以从其他实例创建新实例：

```rust
let User3 = User {
    name: String::from("Rahul"),
    email: User1.email.clone(),
    ..User1 // 表示除了已明确指定的字段外，其他字段都从User1复制
};
```

### 元组结构体 (Tuple Struct)

元组结构体看起来像元组，但有结构体的名称：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### 类单元结构体 (Unit-like Struct)

类单元结构体没有任何字段：

```rust
struct AlwaysEqual;
```

### 结构体数据所有权

当结构体字段使用 `String` 类型时，结构体会获得该数据的所有权：

```rust
// 这里 email 字段会获得字符串的所有权
let User1 = User {
    email: String::from("<EMAIL>"),
    // ...
};
```

## 方法语法

在 Rust 中，可以使用 `impl` 块为结构体定义方法。

### 定义方法

```rust
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

方法的第一个参数总是 `self`，表示调用该方法的实例。`&self` 表示不可变借用实例。

### 关联函数

关联函数是定义在结构体上的函数，但不以 `self` 作为参数：

```rust
impl Rectangle {
    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

// 调用关联函数
let sq = Rectangle::square(3);
```

关联函数通常用于构造器，例如创建特定类型的实例。

## 示例：计算矩形面积

有多种方式可以计算矩形面积：

### 使用普通函数和数值参数

```rust
fn area_By_Num(width: usize, height: usize) -> usize {
    width * height
}
```

### 使用结构体

```rust
fn area_By_Struct(rect: &Rectangle) -> usize {
    rect.width * rect.height
}
```

### 使用元组结构体

```rust
struct Tuple_Rect(usize, usize);

fn area_By_TupleStruct(rect: &Tuple_Rect) -> usize {
    rect.0 * rect.1
}
```

## 构建结构体的函数

可以创建函数来构建结构体实例：

```rust
fn build_User(name: String, email: String, age: i32, is_verified: bool) -> User {
    User {
        name,
        email,
        age,
        is_verified,
    }
}
```

当函数参数名与结构体字段名相同时，可以使用字段初始化简写语法。

## 枚举 (Enum)

枚举允许你通过定义一个类型，这个类型有多种可能的变体。

### 定义枚举

```rust
enum IP {
    //这样需要额外的struct
    IPv4,
    IPv6
}

enum IPaddr {
    //这样不需要额外的struct
    IPv4(u8, u8, u8, u8),
    IPv6(String)
}

struct IPAddr {
    kind: IP,
    address: String
}
```

### 枚举变体中嵌入数据

枚举的每个变体可以包含不同类型和数量的数据：

```rust
// enum里面可以内嵌了多种多样的类型
enum Message {
    Quit,                          // 没有关联数据
    Move { x: i32, y: i32 },       // 包含匿名结构体
    Write(String),                 // 包含一个String
    ChangeColor(i32, i32, i32),    // 包含三个i32值
}
```

### 枚举实例创建

```rust
// 枚举Struct的实例，告诉我们这个的类型是IPv4，address是127.0.0.1
let home = IPAddr {
    kind: IP::IPv4,
    address: String::from("127.0.0.1")
};

// 枚举的实例，这个直接告诉类型是IPv4，address是127.0.0.1，不需要额外的struct
let loopback = IPaddr::IPv4(127, 0, 0, 1);
let loopback_v6 = IPaddr::IPv6(String::from("::1"));
```

## Option 枚举与空值处理

Rust 中没有 Null 概念，而是使用 Option 枚举来处理可能为空的值：

```rust
// Option<T> 定义
// enum Option<T> {
//     Some(T),
//     None,
// }

let some_number = Some(5);        // Some(5)
let some_string = Some("a string"); // Some("a string")
let absent_number: Option<i32> = None; // None
```

Option 和 T 是不同类型，不能直接运算：

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
// let sum = x + y; // 这会报错，因为 Option<i8> 和 i8 是不同类型
```

## 使用 match 处理枚举

match 表达式允许你对枚举的每个变体执行不同的代码：

```rust
impl Level {
   fn grade_By_Level(&self) -> usize {
       match self {
           Level::High => 10,
           Level::Medium => 5,
           Level::Low => 1
       }
   }
}
```

### 从枚举中提取值

使用模式匹配可以从枚举变体中提取数据：

```rust
enum Address {
    test(Province)
}

#[derive(Debug)]
enum Province {
    Liaoning,
    Jiangsu,
}

fn Address_Province(address: Address) {
    match address {
        Address::test(province) => {
            // 可以绑定匹配的模式的部分值
            println!("{:#?}", province);
        }
    }
}

// 使用示例
let address = Address::test(Province::Liaoning);
Address_Province(address);
```

### 匹配 Option<T>

使用 match 处理 Option 值：

```rust
fn plus_One(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}
```

### 使用 _ 通配符

match 必须覆盖所有可能的情况，可以使用 _ 通配符处理其余情况：

```rust
fn printt(x: i32) {
    match x {
        1 | 2 => println!("one or two"),  // 匹配1或者2
        3 => println!("three"),           // 匹配3
        _ => println!("anything")         // 忽略其他所有情况
    }
}
```

### if let 简化匹配

当只关心一种情况时，可以使用 if let：

```rust
fn only_One_Printt(x: i32) {
    if let 1 = x {           // 只匹配1
        println!("one")
    } else {                 // 处理其他所有情况
        println!("anything")
    }
}
```

::## 集合类型 (Collections)

Rust 标准库提供了多种集合类型，用于存储多个值。这些集合类型可以存储在堆上，这意味着数据的数量不需要在编译时知道，并且可以在程序运行时增长或缩小。

### Vector (Vec<T>)

Vector 允许我们在单个数据结构中存储多个值，所有值必须是相同类型：

```rust
// 创建一个新的空Vector
let mut v: Vec<i32> = Vec::new();

// 使用 vec! 宏创建包含初始值的Vector
let v = vec![1, 2, 3];

// 添加元素到Vector
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// 读取Vector中的元素
let v = vec![1, 2, 3, 4, 5];

// 通过索引引用
let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

// 使用 get 方法
match v.get(2) {
    Some(third) => println!("第三个元素是 {}", third),
    None => println!("没有第三个元素"),
}

// 遍历Vector中的元素
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// 遍历可变Vector并修改元素
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

### HashMap<K, V>

HashMap 类型允许我们将值与特定的键相关联，而不是通过索引：

```rust
use std::collections::HashMap;

// 创建一个新的HashMap
let mut scores = HashMap::new();

// 插入键值对
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 通过 vec! 宏和 collect 方法创建HashMap
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// 访问HashMap中的值
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);

// 遍历HashMap
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新HashMap中的值
scores.insert(String::from("Blue"), 25);

// 仅在键没有值时插入
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Green")).or_insert(50);
```

## 关键概念总结

1. **结构体**：自定义数据类型，可以将相关数据打包在一起
2. **方法**：定义在结构体上的函数，通过 `impl` 块实现
3. **关联函数**：定义在结构体上但不使用 `self` 参数的函数
4. **所有权**：结构体字段拥有其数据的所有权
5. **借用**：方法可以通过 `&self` 或 `&mut self` 借用实例
6. **元组结构体**：没有命名字段的结构体，类似元组
7. **类单元结构体**：没有任何字段的结构体
8. **枚举**：定义一个类型，该类型有多种可能的变体
9. **Option 枚举**：用于处理可能为空的值，替代其他语言中的 null
10. **match 表达式**：处理枚举的每个变体，必须穷举所有可能性
11. **模式匹配**：从枚举变体中提取数据
12. **if let**：简化只处理一种情况的匹配
13. **Vector**：存储相同类型值的动态数组
14. **HashMap**：存储键值对的数据结构

结构体和枚举是 Rust 中构建复杂数据类型的基础，通过将数据和操作数据的方法组织在一起，使代码更加清晰和模块化。枚举特别适合表示具有多种变体的数据，而 Option 枚举则提供了一种安全处理可能为空值的方式。Vector 和 HashMap 等集合类型则提供了存储和操作多个值的方法。