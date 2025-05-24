//using struct as a field for another struct

#![allow(dead_code)]

#[derive(Debug)]
struct Point{
    x: f64,
    y:f64,
}

#[derive(Debug)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn calc_area(rect: Rectangle) -> f64{
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;
    width * height
}

fn main(){
    let point: Point = Point{x:3.2, y:8.9};
    let another_point: Point = Point{x:6.5, y:5.6};
    
    println!("First point = ({:?},{:?})", point.x, point.y);
    println!("Second point = ({:?},{:?})", another_point.x, another_point.y);
    
    let bottom_right = Point{x:4.5, ..another_point}; //It uses the another_point.y as the y value
    
    //Destructuring the points using let binding
    
    let Point {x: point1, y: point2} = point;
    
    let top_left = Point {x:point1, y:point2};
    
    let _rectangle = Rectangle{
        top_left: top_left,
        bottom_right: bottom_right,
    };
    
    println!("{:?}",_rectangle);
    
    let area = calc_area(_rectangle);
    
    println!("Area of Rectangle: {:?} unit square", area);
    
    
}
