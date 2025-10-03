//包的规则：
//一个包中至多/只能包含一个库 crate（library crate）；
// 包中可以包含任意多个二进制 crate（binary crate）；
// 包中至少包含一个 crate，无论是库的还是二进制的。
use std::collections::HashMap;//引入std库中的collections模块，直接引入它本身
mod rest_Front{
    pub mod hosting{
        pub(crate) fn add_to_waitlist(){

        }
        fn seat_at_table(){

        }
    }
    mod serving{
        fn take_order(){

        }
    }
    pub struct Eat{
        //使用pub可以让struct变成公有的，但是不会改变字段的 publicity
       pub name:String,
        //字段需要单独使用 pub
        pub age:i32
    }
}
pub use crate::test_Lib::rest_Front::hosting;//引入一个模块引入他的父模块
pub use crate::test_Lib::rest_Front::Eat as  Son_Eat;//as可以改变模块的名字
fn main(){
    rest_Front::hosting::add_to_waitlist();
    //这个是相对路径
    crate::test_Lib::rest_Front::hosting::add_to_waitlist();
    //这个是绝对路径
    //一个模块默认是私有的，如果想访问，需要pub
    //模块上的 pub 关键字只允许其父模块引用它
    //super表示父模块
    super::test_Lib::hosting::add_to_waitlist();
}