mod data;

///
/// 基础的方法
///
fn main() {

    let   a = "aa";
    const B: i32 =12;
    let flag:bool = false;
    let people:(&str,&str,i32) = ("name","boy",12);
    let mut  array = [0;1];

    array[0] =12;

    println!("a:!{}",a);
    println!("b:!{}",B);
    println!("flag:!{}",flag);
    println!("people:!{}", people.0);

    for x in array.iter() {
        println!("array:!{}", x);
    }

    another_function(B);

}

fn another_function(x:i32) {

    println!("Hello, runoob!:{}",x);
}