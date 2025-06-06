//problem with the generic trait and type definition is that we have to specify the type at every usage
struct Container(i32,i32);
trait Contains<A,B>{
    fn contains(&self, _:&A, _:&B)->bool;
    fn first(&self)->i32;
    fn last(&self)->i32;
}

impl Contains<i32,i32> for Container{
    fn contains(&self, number_1:&i32, number_2:&i32)->bool{
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self)->i32{
        self.0
    }
    fn last(&self)->i32{
        self.1
    }
}

fn difference<A,B,C>(container:&C)->i32 where
C:Contains<A,B>{
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
