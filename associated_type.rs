//problem with the generic trait and type definition is that we have to specify the type at every usage.
//use of associated types moves the inner types locally into traits as output types.
struct Container(i32,i32);
trait Contains{
    type A;
    type B;
    fn contains(&self, _:&Self::A, _:&Self::B)->bool;
    fn first(&self)->i32;
    fn last(&self)->i32;
}

impl Contains for Container{
    type A = i32;
    type B = i32;
    fn contains(&self, number_1:&Self::A, number_2:&Self::B)->bool{
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self)->i32{
        self.0
    }
    fn last(&self)->i32{
        self.1
    }
}

fn difference<C>(container:&C)->i32 where
C:Contains{
    container.first() - container.last()
}

fn main(){
    let n1 = 45;
    let n2 = 40;
    let container = Container(45,40);
    println!("Does container contains {} and {}: {}",n1, n2, container.contains(&n1,&n2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("Difference: {:?}", difference(&container));
}
