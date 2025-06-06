use std::fmt::Debug;

#[derive(Debug)]
struct Container<T> {
    data: T,
}
trait Summarizable {
    fn summary(&self);
}

impl<T> Summarizable for Container<T>
where
    T: Debug,
{
    fn summary(&self) {
        println!("Summary: {:?}", self.data);
    }
}

fn main() {
    let data1 = Container {
        data: vec![1, 2, 3],
    };
    data1.summary();
}

