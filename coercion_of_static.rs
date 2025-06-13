static NUM: i32 = 20; //static constant with the lifetime upto the end of the program.

fn coerced_static<'a>(_a: &'a i32) -> &'a i32{
    &NUM
}

fn main(){
    {
        let lifetime_num = 10;
        let coerced_num = coerced_static(&lifetime_num); //It coerced the lifetime of reference to NUM to the lifetime of lifetime_NUM 
        println!("coerced_num: {}", coerced_num);
    }
    
    println!("NUM:{:?} is still available throughout the lifetime of the program", NUM);
}
