//Converting to the string
//fmt::Display provides the ToString automatically so we can directly implement to_string()

use std::fmt;

struct Circle{
    radius: i32
}

impl fmt::Display for Circle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Radius of Circle: {}", self.radius)
    }
}

fn main(){
    let circle = Circle{radius: 44};
    println!("{}", circle.to_string());
}
