trait MyIterator<T>{
    fn next(&mut self) -> Option<T>;
}

fn my_iter<I,T>(iter: I) where
I: MyIterator{
    ...
}

//using associated item for trait

struct Counter;
trait MyIterator<T>{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl MyIterator for Counter{
    type Item = u32;
    
    fn next(&mut self)->Option<Self::Item>{
        Some(42);
    }
}

fn my_iter<I>(iter:I) where
I:MyIterator
{
    if let Some(val) = iter.next(){
        println!("{:?}", val);
    }
}
