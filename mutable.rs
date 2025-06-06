fn main() {
    let immutable_box = Box::new(5u32);
    println!("Immutable box has data {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("Mutable box has data {}", mutable_box);

    *mutable_box = 4;
    println!("Mutable box now has data {}", mutable_box);
}
