use std::ptr::slice_from_raw_parts;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //const 声明常量
fn main() {
    let x = 5;
    println!("The value of the x : {x}");
    //x = 6;不可变变量不可重复赋值
    let mut y = 5;
    y = 6; //可变变量可以再次赋值(可变性与不可变性)

    let z = 5;
    //x = x+1;要用关键字let表示重新定义(隐藏)
    let z = z + 1;

    let space = "    ";
    // space = space.len();变量转换类型要加关键字let
    let space = space.len();
}
