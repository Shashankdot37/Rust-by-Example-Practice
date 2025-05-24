fn reverse(pair:(i32,bool)) -> (bool, i32){
    let (int_parameter, bool_parameter) = pair;
    (bool_parameter, int_parameter)
}

fn main(){
    let pair = (32, false);
    println!("The pair is {:?}", pair);
    println!("The reverse pair is {:?}", reverse(pair));
}
