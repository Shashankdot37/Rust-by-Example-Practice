use std::marker::Copy;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Copy + Into<f64>,
{
    fn distance_from_origin(&self) -> f64 {
        let x_f64 = self.x.into();
        let y_f64 = self.y.into();

        ((x_f64 * x_f64) + (y_f64 * y_f64)).sqrt()
    }
}

fn main() {
    let point1 = Point { x: 3, y: 4 };

    println!("Distance from origin: {}", point1.distance_from_origin())
}
