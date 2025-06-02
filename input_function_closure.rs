fn call_this<S>(f:S) where
    S: Fn(){
        f()
    }

fn print_func(){
    println!("-hello there.");
}

fn main(){
    let closure1 = ||{
        println!("-hiya.")
    };
    call_this(closure1);
    call_this(print_func);
}
