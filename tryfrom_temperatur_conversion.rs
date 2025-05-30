use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Fahrenheit(f64);

struct TemperatureError;

impl fmt::Display for TemperatureError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Error: Temperature is below the absolute temperature")
    }
}

//To convert celsius to fahrenheit
impl TryFrom<&Celsius> for Fahrenheit {
    type Error = TemperatureError;

    fn try_from(temp: &Celsius) -> Result<Self, Self::Error> {
        if temp.0 >= -273.15 {
            let f_temp = temp.0 * 9.0 / 5.0 + 32.0;
            Ok(Fahrenheit(f_temp))
        } else {
            Err(TemperatureError)
        }
    }
}

//To convert fahrenheit to celsius
impl TryFrom<&Fahrenheit> for Celsius {
    type Error = TemperatureError;
    fn try_from(temp: &Fahrenheit) -> Result<Self, Self::Error> {
        if temp.0 >= -459.67 {
            let c_temp = (temp.0 - 32.0) * 5.0 / 9.0;
            Ok(Celsius(c_temp))
        } else {
            Err(TemperatureError)
        }
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°F", self.0)
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}

fn main() {
    let celsius = Celsius(32.33);
    let converted_fahrenheit = Fahrenheit::try_from(&celsius);
    let fahrenheit = Fahrenheit(100.00);
    let converted_celsius = Celsius::try_from(&fahrenheit);
    
    match converted_fahrenheit{
        Ok(f) => println!("{} converted to {}°F", celsius, f.0),
        Err(_) => println!("Invalid Celsius temperature!"),
    };
    match converted_celsius{
        Ok(f) => println!("{} converted to {}°C", fahrenheit, f.0),
        Err(_) => println!("Invalid Fahrenheit temperature!"),
    };
}
