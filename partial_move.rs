// Challenge: Track and Print Active Fields
// You're building a small system to track a File that has metadata.
// Your Task:
//     Destructure a File instance.
//     Move the name field, but only borrow the data field.
//     Print the name and the size of the file in bytes (using .len() on data).
//     Try using the original File afterward and observe what happens. Comment it out with an explanation.

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

//Cannot implement the Drop cause it requires the whole object and cannot be done when there is partial move.
// impl Drop for File{
//     fn drop(&mut self){
//         println!("Dropping the file: {:?}", self);
//     }
// }

fn main(){
    let file = File{
        name: String::from("Hello"),
        data: vec![1,2,3,3,4,5]
    };
    let File{name, ref data} = file;
    println!("Name of the File: {:?}", name);
    println!("Size of the File: {:?}", data.len());
    println!("Size of the File: {:?} (Shown from the file variable)", file.data.len());
    
    // println!("File info: {:?}", file);
    // error[E0382]: borrow of partially moved value: `file`. The variable field name is moved to the variable "name" so it is partially moved. The data field is however only borrowed so it is still there.
    
}
