//ref pattern matching

#[derive(Clone, Copy)]
struct Point{
    x:i32,
    y:i32
}

fn main(){
    //putting "ref" keyword on the left side is same as putting & on the right side
    let c = 'A';
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2 : {}", *ref_c1 == *ref_c2);
    //ref is valid when destructuring the struct
    
    let point = Point{
        x : 0,
        y : 0
    };
    
    let _copy_of_x = {
        let Point{x: ref ref_to_x, y: _} = point;
        *ref_to_x
    };
    
    //mutable copy of point
    
    let mut mutable_point = point;
    
    {
        let Point{x:_, y: ref mut mut_ref_to_y} = mutable_point;
        
        *mut_ref_to_y = 1;
    }
    
    println!("Point = ({}, {})", point.x, point.y);
    println!("Point = ({}, {})", mutable_point.x, mutable_point.y);
    
    //can be done same with tuple
}
