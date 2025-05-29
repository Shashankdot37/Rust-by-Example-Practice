// Conversion

struct NumberData{
    value: i32;
}

impl From<i32> for NumberData{
    fn from(item: i32) -> Self{
        NumberData{
            value: item;
        }
    }
}

//Into requires type specifying as the program may not be able to do that all the time
impl Into<i32> for NumberData{
    fn into(self) -> NumberData{
        NumberData{
            value: self
        }
    }
}

// Conversion between str and String
fn main(){

    //Conversion from String to &str
    let s1 = String::from("String 1"); //String Type
    let slice1 = s1.as_str(); //&str type
    println!("{}",slice1);
    
    //From Implementation for the custom type -- struct
    let int = 32;
    let printable1 = NumberData::from(int);
    println!("{:?}", printable1);
    
    let num::NumberData = int.into();
    println!("{:?}", num);
}
