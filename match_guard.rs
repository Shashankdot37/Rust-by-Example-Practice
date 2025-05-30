//using a match guard

fn main(){
    let temps = vec![-5, 0, 7, 15, 22, 35];
    
    for temp in temps.iter(){
        match temp{
            t if *t <= 0 => println!("{:?}°C is Freezing", t),
            t if *t <= 10 && *t >= 0 => println!("{:?}°C is Cold", t),
            t if *t <= 20 && *t>= 11 => println!("{:?}°C is Cool", t),
            t if *t <= 30 && *t>=21 => println!("{:?}°C is Warm", t),
            t if *t > 30  => println!("{:?}°C is Hot", t),
            _=>()
        }
    }
}
