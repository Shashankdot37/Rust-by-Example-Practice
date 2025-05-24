use std::fmt;

struct Location{
    name: &'static str,
    lat:f32,
    lon:f32,
}

impl fmt::Display for Location{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let direction1 = if self.lat >= 0.0 {'N'} else {'S'};
        let direction2 = if self.lon >= 0.0 {'E'} else {'W'};
        
        write!(f, "name: {} position: {:.3}degree {} and {:.3}degree {}", self.name, self.lat.abs(), direction1, self.lon.abs(),direction2)
        
    } 
}

fn main(){
    for city in [
    Location { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    Location { name: "Oslo", lat: 59.95, lon: 10.75 },
    Location { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city)
    }
}
