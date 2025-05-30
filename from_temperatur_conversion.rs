use std::convert::From;
use std::fmt;

struct Celsius(f64);

struct Fahrenheit(f64);

//To convert celsius to fahrenheit
impl From<&Celsius> for Fahrenheit{
    fn from(temp: &Celsius) -> Self{
        let f_temp = temp.0 * 9.0/5.0 + 32.0;
        Fahrenheit(f_temp)
    }
}

//To convert fahrenheit to celsius
impl From<&Fahrenheit> for Celsius{
    fn from(temp: &Fahrenheit) -> Self{
        let c_temp = (temp.0 - 32.0) * 5.0/9.0;
        Celsius(c_temp)
    }
}

impl fmt::Display for Fahrenheit{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Celsius{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

fn main(){
    let celsius = Celsius(32.33);
    let converted_fahrenheit = Fahrenheit::from(&celsius);
    let fahrenheit = Fahrenheit(100.00);
    let converted_celsius = Celsius::from(&fahrenheit);
    println!("Celsius temperature: {} converted to Fahrenheit temperature: {}", &celsius, &converted_fahrenheit);
    println!("Fahrenheit temperature: {} converted to Celsius temperature: {}", &fahrenheit, &converted_celsius);
}
