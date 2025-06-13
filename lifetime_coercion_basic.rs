//Coercion of the lifetime. A longer lifetime can be coerced into the shorter one
fn choose_first<'a:'b,'b>(a: &'a u32, _b:&'b u32)->&'b u32{
    a
}

fn main(){
    let first_var = 12;
    
    {
        let second_var = 14;
        let result = choose_first(&first_var, &second_var);
        println!("{:?}", *result)
    }
}

//here the lifetime of "result" is 'b but the value returning from inside the function had lifetime 'a which was coerced into 'b to
//be fitted inside the scope.
