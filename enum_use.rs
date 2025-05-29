//use declaration to avoid manual scoping

#![allow(dead_code)]

enum Players{
    Raphinha, 
    Lewa,
    Yamal,
    Pedri,
    Casado,
    Bernal,
    Balde
}

// type P = Players; //type alias

fn main(){
    use crate::Players::*; //* applies for all the elements
    let selected_player = Pedri;//same as Players::Raphinha
    match selected_player{
        Raphinha => println!("Raphaaaa"),
        Lewa => println!("Lewaaaaa"),
        Pedri => println!("Pedriiii"),
        Balde => println!("Baaaaldeeeee"),
        _ => println!("Others")
        
    }
}
