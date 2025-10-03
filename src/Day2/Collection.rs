// 引入标准库中的collections模块
use std::collections::HashMap;

fn main() {
    // Vector 示例
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
    
    // HashMap 示例
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
    
    // 字符串作为集合类型
    let mut s = String::new();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    
    // 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    
    // 字符串连接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{}-{}-{}", s1, s2, s3);
}