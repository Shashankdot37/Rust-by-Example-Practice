use std::marker::Copy;
use std::cmp::PartialOrd;

fn find_min<T>(items: Vec<T>) -> T where
T:PartialOrd + Copy,{
    let mut temp = items[0];
    for item in items{
        if item < temp{
            temp = item;
        }
    }
    return temp
}

fn main(){
    let num = vec![34,30,12,34,10,33];
    let min = find_min(num);
    println!("Minimum value: {:?}", min);
}
