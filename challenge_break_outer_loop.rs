//Program to inner loop through an array for each value of outer array to find a first pair whole sum is 15.

fn main(){
    let list_a = [1, 3, 5, 7, 9];
    let list_b = [2, 4, 6, 8, 10];
    
    let mut found = false;
    
    'outer: for outer_item in list_a.iter(){
        for inner_item in list_b.iter(){
            if outer_item + inner_item == 15{
                println!("Found pair: ({:?},{:?})", outer_item, inner_item);
                found = true;
                break 'outer;
            }
        }
    }
    
    if !found{
        println!("Values not found");
    }
    
}
