fn main(){
    let mut my_num = Vec::new();
    my_num.push(10);
    my_num.push(20);
    my_num.push(30);
    my_num.push(40);
    my_num.push(50);
    my_num.pop();
    my_num.push(60);
    
    for (i,v) in my_num.iter().enumerate(){
        println!("({}, {})", i, v)
    };
}
