use std::cmp::Ordering;

mod chapter3;
mod numGuess;

fn main() {
    let a=5;
    let b=5isize;
    let b=5usize;
    //元组
    let tuple1=(1,2.2,3isize);
    let (x,y,z)=tuple1;
    //声明数组的形式
    let mut a:[isize;5]=[1,2,3,4,5];
    let b=[3;5];
    //循环
    for i in 0..5{
        println!("{}",i);
    }

    for i in a.iter(){
        println!("{}",i);
    }
    //for还可以调用iter实现迭代器
    for (i,& item) in a.iter().enumerate(){
        println!("{} {}",i,item);
    }
    //while循环
    while a[0]<5{
        println!("{}",a[0]);
        a[0]+=1;
    }
    let mut i=0;
    loop{
        println!("loop");
        i+=1;
        if i==5{

            break;
        }
    }
    //loop可以有标签
    'outer: loop{
        'inner: loop{
            println!("inner");
            break 'outer;
        }
    }
    //loop还可以有返回值
    let loop_result=loop{
        println!("loop");
        break 5;
    };
    println!("loop_result:{}",loop_result);
    //所有权
    //String 类型是在堆上，默认是浅拷贝，所以s2会得到s1的所有权，s1会失效
    let s1=String::from("hello");
    let s2=s1;
    //这个时候，s1已经失效了，再想输出s1就会报错
    println!("{}",s2);
    //如果是用clone,则s1不会失效
    let s1=String::from("hello");
    let s2=s1.clone();
    println!("{}",s1);
    //如果是数字，因为实在栈上，所以不会失效
    let x=5;
    let y=x;
    println!("{}",x);
    //引用
    let mut s1 =String::from("hello");
    let s2=&s1;
    println!("{}",s1);
    println!("{}",s2);
    //函数传递参数也是类似的，传递的是引用，而不是值
    //testFunction(s1);
    //这里s1失效了
    //如果传递&str，则s1不会失效
    testFunction2(&s1);
    //但是引用是不可变的
    //想使用可变引用，则需要使用&mut
    //注意，一个作用域内只能有一个可变引用
    let s1=&mut s1;
    //这样会报错
    //let s2=&mut s1;
    testFunction3(s1);
    let s1=String::from(" he   l  lo  ");
    let s2=first_word(&s1);
    match s2.cmp(&s1.len()) {
        Ordering::Less => {
            println!("{}", &s1[..s2]);
        }
        Ordering::Equal => {
            println!("{}", s2);
        }
        Ordering::Greater => {
            println!("代码错误")
        }
    }
    let s1=String::from("hello world");
    let s2=first_word(&s1);
    println!("{}",s2);
    match s2.cmp(&s1.len()) {
        Ordering::Less => {
            println!("第一个单词是: {}", &s1[s2..s2+1]);
        }
        Ordering::Equal => {
            println!("整个字符串是一个单词: {}", &s1[..]);
        }
        Ordering::Greater => {
            println!("代码错误")
        }
    }
    //切片
    let s1=String::from("hello world");
    let s2=&s1[0..];
    let s3=&s1[..5];
    let s4=&s1[..];
    let s5="asd";
    //s5其实是&str
}

//testFunction
fn testFunction(x:String){
    println!("{}",x);
}
//testFunction2
fn testFunction2(x:&String){
    println!("{}",x);
}
//testFunction3
fn testFunction3(x:&mut String){
    x.push_str(" world");
    println!("{}",x);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    // 遍历字符串的每个字节，查找第一个空格字符后面的第一个字符
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 找到第一个空格时，返回其位置
            continue;
        }else{
            return i;
        }
    }

    // 如果没有找到空格，返回字符串的长度
    s.len()
}