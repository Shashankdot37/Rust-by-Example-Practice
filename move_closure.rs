//move keyword

fn main(){
    let fighters = vec!["Perriera", "Jon Jones", "Holloway", "Topuria"];
    
    let contains = move |f| fighters.contains(f); //moves the vector into "contains" variable
    
    println!("{}", contains(&"Jon Jones"));
    println!("{}", contains(&"Tyson"));
}
