//binding
fn age() -> u32{
    15
}

fn main(){
    match age(){
        0 => println!("haven't celebrated the first birthday yet."),
        n @ 1..=12 => println!("Yet a child"),
        n @ 13..=19 => println!("Teen"),
        n @ 20..=25 => println!("Young Adult"),
        _=>println!("Adult")
    }
}

//could be use for destructuring too. just need to replace the range .. with specific value. 
