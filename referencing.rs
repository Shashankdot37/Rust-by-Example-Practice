//Pointers and Referencing

fn main() {
    let reference = &4; //& means that the reference is being assigned.

    //Destructing to obtain the value.
    match reference {
        &val => println!(
            "The destructured value obtained from the reference is {:?}",
            val
        ),
    }

    //Dereferencing to obtain the value.
    match *reference {
        val => println!(
            "The value obtained after derefencing the reference is {:?}",
            val
        ),
    }

    //ref and ref mut keywords are provided to obtain the reference.
    let ref is_ref = 14; //is_ref stores the reference for the value "14";

    let value = 34;
    match value {
        ref r => println!("The reference is obtained for the value: {:?}", value),
    }

    let mut val = 12;
    match val {
        ref mut val => {
            *val = *val * 2;
            println!(
                "The mutable value was deferenced followed by multiplication by 2. Result :- {:?}",
                val
            )
        }
    }
}
