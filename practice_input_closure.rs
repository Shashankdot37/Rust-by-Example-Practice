fn process_list<F>(mut f:F, value:i32)->i32 where
    F: FnMut(i32) -> i32{
        f(value)
    }
fn process_even<F>(f:F,value:i32)->bool where
    F:FnOnce(i32)->bool{
        f(value)
    }


fn main(){
    let numbers = vec![1,2,3,4,5];
    let double = |num| {
        2 * num
    };
    let even_check = |num|
    {
        if num%2 == 0{
            true
        }
        else{
            false
        }
    };
    for number in numbers.iter(){
        let result1 = process_list(double, *number);
        let result2 = process_even(even_check, *number);
        println!("{:?} -- is even? -> {:?}, *2 = {:?}",number, result2, result1);
    }
    
}
