// Your Task
//     Define a Book struct with those fields.
//     Implement two functions:
//         fn print_info(book: &Book)
//         📘 Takes an immutable reference, and prints the book's info.
//         fn mark_unavailable(book: &mut Book)
//         🔒 Takes a mutable reference and sets available = false.
//     In main():
//         Create a book.
//         Immutably borrow it and print its info.
//         Then mutably borrow it to mark it as unavailable.
//         Try immutably borrowing it again after it’s been mutably borrowed and modified.
//     Lastly, try mutably borrowing it while it’s already immutably borrowed, and observe the compiler error — comment it out with explanation.

#[derive(Debug)]
struct Book{
    author: &'static str,
    title: &'static str,
    year: u32,
    available: bool
}

fn print_info(book: &Book){
    println!("Book Info \n Author:{:?} \n Title:{:?} \n Published Year:{:?} \n Available: {:?}", book.author, book.title, book.year, book.available);
}

fn mark_unavailable(book : &mut Book){
    book.available = false;
     println!("Modified Book Info \n Author:{:?} \n Title:{:?} \n Published Year:{:?} \n Available: {:?}", book.author, book.title, book.year, book.available);
}


fn main(){
    let mut book1 = Book{
        author: "Paul Kalanithi",
        title: "When Breathe becomes Air",
        year: 2016,
        available: true
    };
    
    print_info(&book1);
    println!("\n");
    println!("\n");
    mark_unavailable(&mut book1);
    println!("\n");
    println!("\n");
    print_info(&book1);
    //The variable field was referenced mutable and was modified so even when now the variable field is immutably borrowed and printed, it shows the modified value.
}
