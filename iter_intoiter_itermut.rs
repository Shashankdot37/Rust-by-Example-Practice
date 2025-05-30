fn main() {
    let players = vec!["Lewa", "Inaki", "Victor", "Ferran"];
    let mut def = vec![
        String::from("Araujo"),
        String::from("Cubarsi"),
        String::from("Inigo"),
        String::from("Kounde"),
        String::from("Balde"),
        String::from("Christensen"),
    ];
    let coaches = vec!["Hansil Flick", "Marcus Sorg", "Toni Tapalovic","Heiko Westermann","José Ramon De la Fuente"];
    //iter
    for player in players.iter() {
        match player {
            &"Inaki" => println!("There is a Goalkeeper"),
            _ => println!("Attacker: {:?}", player),
        }
    }

    //iter_mut
    for d in def.iter_mut() {
        match d.as_str() {
            "Balde" => d.push_str(" --injured"),
            "Christensen" => d.push_str(" --transfer listed"),
            _ => {}
        };
        println!("{:?}", d)
    }
    
    //for by default used into_iter. it takes the ownership rather than reference.
    for coach in coaches.into_iter(){
        match coach{
            _ => println!("{:?}", coach)
        }
    }
}
