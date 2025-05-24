fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main(){
    let arr:[i32;6]=[1,2,3,4,5,6];
    let slice: &[i32] = &arr[0..4];
    let sum = sum(slice);
    println!("{:?}", sum);
}
