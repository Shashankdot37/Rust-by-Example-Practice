// Challenge: Filter & Display Printable Items in a Collection
// Write a generic function called print_filtered<T> that:
//     Takes a list of values (as a Vec<T>).

//     Accepts a predicate function F as a closure (e.g., |item| item > 10) to filter the list.

//     Prints only the items that satisfy the predicate, one per line.

//     Use a where clause to ensure:

//         T implements Debug (because you'll print it using println!("{:?}", ...))

//         F is a closure that takes a &T and returns a bool.


use std::fmt::Debug;

fn print_filtered<T, F>(items:Vec<T>, predicate: F) where
T: Debug,
F: Fn(&T) -> bool,{
    for item in items{
        if predicate(&item) {
            println!("{:?}", item)
        }
    }
}

fn main(){
    let items = vec![5, 12, 8, 30, 3];
    let words = vec!["appleeeeee", "banana", "pearrrrrrrrr", "grape"];
    
    print_filtered(items, |item|{item>&10});
    print_filtered(words, |word|{word.len()>10});
}
