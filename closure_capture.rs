//Capturing in the closure
use std::mem;
fn main() {
    //Immutable borrow
    let chef = String::from("Carmy");

    let print = || println!("{}", chef); //println uses immutable reference

    print(); //immutable ref to chef used so this is also immutable var that rep closure

    let _borrowed_chef = &chef; //valid

    print(); //valid

    let moved_chef = chef; //valid. Now the chef is moved to moved_chef

    println!("{}", moved_chef); //valid

    // print(); //Now it is invalid

    //Mutable borrow

    let mut count = 0;

    let mut inc = || {
        count = count + 1;

        println!("{}", count); //mutable borrow
    };

    inc(); //valid

    // let reborrow_count = &count; //Invalid -- Cannot immutable borrow the count as it is still being mutably borrowed by inc.

    // println!("{}",reborrow_count);

    inc();
    
    let movable = Box::new(3);
    
    let consume = || {
        println!("Movable: {:?}", movable);   //it takes the ownership i.e it moves the data so can be called only once
        mem::drop(movable);
    };
    
    consume();
    
}
