enum IP{
    //这样需要额外的struct
    IPv4,IPv6
}
enum IPaddr{
    //这样不需要额外的struct
    IPv4(u8,u8,u8,u8),
    IPv6(String)
}
struct IPAddr{
    kind:IP,
    address:String
}
//enum里面可以内嵌了多种多样的类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//Rust中没有Null这一说，所以，Rust使用Option枚举来表示一个值可能为null。
//Option<T>
// enum Option<T> {
//     Some(T),
//     None,
// }
//这个表示如果值存在，那么Some(T)，不存在就None
enum Level{
    High,
    Medium,
    Low
}
impl Level{
    fn grade_By_Level(&self) ->usize{
        match self{
            Level::High => 10,
            Level::Medium => 5,
            Level::Low => 1
        }
    }

}
enum Address{
    test(Province)
}
#[derive(Debug)]
enum Province{
    Liaoning,
    Jiangsu,
}
fn Address_Province(address:Address){
    match address{
        Address::test(province)=>{
            //可以绑定匹配的模式的部分值
            println!("{:#?}",province);
        }
    }
}
//可以匹配Option<T>
fn plus_One(x:Option<i32>)->Option<i32>{
    match x{
        Some(i)=>Some(i+1),
        None=>None
    }
}
//必须穷举，可以使用_来忽略
fn printt(x:i32){
    match x{
        1 | 2 => println!("one or two"),//匹配1或者2
        3 => println!("three"),//匹配3
        _ => println!("anything")//忽略
    }
}
//如果只需要匹配一个值，可以使用if  let
fn only_One_Printt(x:i32){
    if let 1 = x{//匹配1
        println!("one")
    }//忽略,可以用else
    else{
        println!("anything")
    }
}
fn main() {
    //枚举Struct的实例，告诉我们这个的类型是IPv4，address是127.0.0.1
    let home = IPAddr{
        kind:IP::IPv4,
        address:String::from("127.0.0.1")
    };
    //枚举的实例，这个直接告诉类型是IPv4，address是127.0.0.1，不需要额外的struct
    let loopback =IPaddr::IPv4(127,0,0,1);
    let loopback =IPaddr::IPv6(String::from("::1"));
    let some_number = Some(5);//5
    let some_string = Some("a string");//"a string"

    let absent_number: Option<i32> = None;// None，表示这个值不存在
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y；这个会报错，因为Option<T>和T是不同的类型，所以不能相加
    let level = Level::High;
    println!("{}",level.grade_By_Level());
    let address = Address::test(Province::Liaoning);
    Address_Province(address);
}