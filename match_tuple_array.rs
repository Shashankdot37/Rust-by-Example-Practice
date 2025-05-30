//match and destructuring it's types
#[allow(unreachable_patterns)]
fn main(){
    let tuple1 = (4,5,7,"chill");
    let array1 = [3,4,5,6,7,8,9];
    match tuple1{
        // (.., x) => println!("The last value is {:?}", x),
        // (x, ..) => println!("The first value is {:?}", x),
        (4, a, b,c) => println!("The first value is 4, second is {:?}, third is {:?} and fourth is {:?}", a,b,c),
        _ => println!("Whatever")
    }
    
    match array1{
        // [first, second, rest @ ..] => println!("First value: {}, Second Value: {}, Rest of the values: {:?}", first, second, rest),
        [first, middle @ .., last] => println!("First value: {}, Middle Values :{:?}, Last value: {}", first, middle, last)
    }
    //similar for enums
    
}
