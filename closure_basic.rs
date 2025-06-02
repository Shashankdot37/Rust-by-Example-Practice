//Closure

fn main(){
    let outer_var = 50;
    
    let closure_annotated = |x:i32| -> i32 {x + outer_var};
    let closure_inferred = |x| x+outer_var;
    
    println!("closure_annotated = {:?}", closure_annotated(23));
    println!("closure_inferred = {:?}", closure_inferred(22));
    
    //closure with no input value but returns i32 value
    
    let three = || 3;
    
    println!("Three: {:?}", three());
}
