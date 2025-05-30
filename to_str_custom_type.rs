use std::str::FromStr;
use std::num::ParseIntError;

#[allow(dead_code)]
#[derive(Debug)]
struct Circle{
    radius: i32
}

impl FromStr for Circle{
    type Err = ParseIntError; 
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s.trim().parse(){
            Ok(num) => Ok(Circle{radius: num}),
            Err(e) => Err(e)
        }
    }
}

fn main(){
let radius = "32";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}
