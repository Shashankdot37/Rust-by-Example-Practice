use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

//TryFrom -- fallible conversion defining how to create itself from other type
impl TryFrom<i32> for EvenNumber{
    type Error = ();
    
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value % 2 == 0{
            Ok(EvenNumber(value))
        }
        else
        {
            Err(())
        }
    }
}

//implementation of TryFrom allows the automatic implementation of TryInto try_into()

fn main(){
    assert_eq!(Ok(EvenNumber(12)),EvenNumber::try_from(12));
    assert_eq!(Err(()),EvenNumber::try_from(13));
    
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(Ok(EvenNumber(8)), result);
    
    let result2: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(Err(()), result2);
}
