struct Student<'a>{
    name: &'a str,
    age :i32,
    id: i32
}

fn main(){
    let s1 = Student{
        name: "Marc",
        age: 21,
        id: 00133
    };
    
    match s1{
        // Student{name, ..} => println!("The name of the student is {:?}", name),
        Student{name, id, age: 22} => println!("Name: {:?} Age: 22", name),
        _ =>println!("The student is not of age 22.")
    }
    
    let s2 = Student{
        name: "Pablo",
        age: 20,
        id: 22323
    };
    
    let Student{name: name1, age: age1, id:id1} = s2;
    println!("Name of 2nd student: {:?}. Age: {:?}. Id:{:?}", name1, age1, id1);
}
