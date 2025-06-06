//Bounds -- Generic
use std::fmt::Debug;
trait HasArea{
    fn area(&self) -> i32;
}

#[derive(Debug)]
struct Rectangle{
    length:i32,
    breadth:i32
}

#[allow(dead_code)]
struct Triangle{
    length:i32,
    height:i32
}

impl HasArea for Rectangle{
    fn area(&self) -> i32{
        self.length * self.breadth
    }
}

fn print_debug<T:Debug>(t:T){
    println!("{:?}", t);
}

fn area<T:HasArea>(t:&T)->i32{
t.area()
}

fn main(){
    let rectangle = Rectangle{
        length:12,
        breadth:15
    };
    
    let triangle = Triangle{
        length:12,
        height: 51
    };
    
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}
