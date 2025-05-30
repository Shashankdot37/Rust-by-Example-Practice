//Breaking the outer loop from inner loop
//Requires 'label labelling

fn main(){
    'outer: loop{
        println!("Entered the outer loop.");
        
        'inner: loop{
            println!("Entered the inner loop.");
            
            break; //break the inner loop
            
            break 'outer; //break the outer loop
        }
        
        println!("This point will never be reached");
    }
    println!("Got out of outer loop")
}
