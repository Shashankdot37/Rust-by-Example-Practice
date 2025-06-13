// Create a function longer_str<'a>(x: &'a str, y: &'a str) -> &'a str
// that returns the string slice which is longer between x and y.
//  Constraints:
//     You must use an explicit lifetime annotation.
//     You must not clone or move the data (just return one of the references).
//     In main(), test it with a few inputs.

fn longer_str<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len()>y.len() {
        return x
    }
    else{
        return y
    }
}

fn main(){
    let str1 = String::from("Hello");
    let _str2 = String::from("World");
    
    let result = longer_str(&str1, &_str2);
    println!("{:?}", result);
    
    // let result;
    // {
    //     let _str3 = String::from("Sansaar");
    //     result = longer_str(&str1, &_str3);
    // }
    // println!("{:?}", result);
    //This doesn't work because the _str3 is terminated at the end of the block in which it was declared.
}
