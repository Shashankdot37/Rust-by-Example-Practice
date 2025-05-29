enum Barcelona<'a>{
    Pedri,
    Gavi,
    Coach(&'a str),
    WingersNum{left_wing : i32, right_wing: i32}
}

// type Barca = Barcelona; //type alias for cases when enum name is too long or generic

fn print_enum(item: Barcelona){
    match item{
        Barcelona::Pedri => println!("Pedriiiii"),
        Barcelona::Gavi => println!("Gaviraaaaa"),
        Barcelona::Coach(s)=>println!("Current coach of FC Barcelona: {:?}", s),
        Barcelona::WingersNum{left_wing, right_wing} => println!("Left Winger's shirt no. = {:?} and Right Winger's shirt no. = {:?}", left_wing, right_wing),
    }
}

fn main(){
    let player_1 = Barcelona::Pedri;
    let player_2 = Barcelona::Gavi;
    let coach = Barcelona::Coach("Hansil Flick");
    let wingers = Barcelona::WingersNum{left_wing: 11, right_wing: 19};
    
    print_enum(player_1);
    print_enum(player_2);
    print_enum(coach);
    print_enum(wingers);
    
}
