//Closure as input parameter

fn closure_func<F>(f: F) where
    F: FnOnce(){
        f();
    }

fn apply_to_3<F>(f:F)->i32 where
    F:Fn(i32)->i32{
        f(3)
    }
    
fn main(){
    use std::mem;
    let str1 = "Hello";
    let mut str2 = "Worl".to_owned();
    
    let closure1 = || {
        println!("{}", str1);
        
        str2.push_str("d");
        println!("{:?}", str2);
        
        mem::drop(str2);
    };
    
    closure_func(closure1);
    
    let double = |num|{
        num * 2
    };
    
    println!("{}",apply_to_3(double));
    
}
