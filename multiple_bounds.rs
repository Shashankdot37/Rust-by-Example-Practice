//Multiple bounds are applied using "+"" and types can be separated using ",". 

use std::fmt::{Display, Debug};

fn compare_prints<T: Display + Debug>(t: &T){
    println!("Debug: {:?}", t);
    println!("Display: {}",t);
}

fn compare_types<T:Debug, U:Debug>(t:&T, u:&U){
    println!("{:?}", t);
    println!("{:?}", u);
}

fn main(){
    let str1 = "String";
    let arr1 = [1,2,3,4];
    compare_prints(&str1);
    compare_types(&str1, &arr1);
}
