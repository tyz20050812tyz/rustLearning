#[derive(Debug)]
struct User{
    name: String,
    email: String,
    age: i32,
    is_verified: bool
}
struct Rectangle{
    width: usize,
    height: usize
}
//想在结构体中添加方法,使用impl
impl Rectangle{
    fn area(&self)-> usize{
        self.width*self.height
    }
    //有别的参数，要用other表示
    fn can_hold(&self, other: &Rectangle)-> bool{
        self.width>other.width && self.height>other.height
    }
    //创建一个关联函数,不需要&self
    fn square(size: usize)-> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}
struct Tuple_Rect(usize,usize);
fn main() {
    //创建一个不可变的结构体
    let User1 = User{
        name: "".to_string(),
        email: String::from("<EMAIL>"),
        age: 18,
        is_verified: true
    };
    //创建一个可变结构体
    let mut User2 = User{
        name: String::from("Rahul"),
        email: String::from("<EMAIL>"),
        age: 18,
        is_verified: true
    };
    //更新的时候，如果没有变化，可以简写
    let User3=User{
        name: String::from("Rahul"),
        email: User1.email.clone(),
        //这里email是String类型，如果直接赋值就会move，User1失去所有权
        ..User1 //表示除了name更新，其他不变
        //或者用全克隆
        //..User1.clone()
    };
    //TupleStruct,外表观和struct相同，但是内部元素没有名字
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
    //UnitLikeStruct
    struct AlwaysEqual;
    //输出结构体
    println!("{:#?}", User1);
}
fn build_User(name: String, email: String,age: i32,is_verified: bool)-> User{
    //可以简写，如果命名相同
    User{
        name,
        email,
        age,
        is_verified,
    }
}
//计算长方形面积(输入数值)
fn area_By_Num(width: usize, height: usize)-> usize{
    width*height
}
//计算长方形面积(输入结构体)
fn area_By_Struct(rect: &Rectangle)-> usize{
    rect.width*rect.height
}
//计算长方形面积(输入元祖结构体)
fn area_By_TupleStruct(rect:&Tuple_Rect)-> usize{
    rect.0*rect.1
}