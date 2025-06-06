//Implementation -- generics

struct Val{
    val:f64
}

struct GenVal<T>{
    g_val:T
}

impl Val{
    fn value(&self) -> &f64{
        &self.val
    }
}

impl<T> GenVal<T>{
    fn g_value(&self) -> &T{
        &self.g_val
    }
}

fn main(){
    let var1= Val{val:6.0};
    let var2= GenVal{g_val:'B'};
    
    println!("{}, {}", var1.value(), var2.g_value());
}
