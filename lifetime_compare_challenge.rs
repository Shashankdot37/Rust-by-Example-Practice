// 🧪 Challenge: Write a Comparator with Lifetimes
// 🔧 Task:
// Write a function called compare_and_return<'a> that:
//     Takes two references to i32 values: a: &'a i32, b: &'a i32.
//     Returns the reference to the larger of the two.
//     Uses explicit lifetime annotations.
//     Does not copy or move the values — just returns a reference.
// 🧠 Bonus Twist:
// In main():
//     Declare two integers m and n.
//     Call compare_and_return() and store the result.
//     Print the larger number using the returned reference.
// Then:
//     Add a scoped block ({ ... }) that introduces a third variable k.
//     Use compare_and_return() to compare m and k.
//     Try returning and using that result outside the block — observe the compiler error and explain it in comments.

fn compare_and_return<'a>(a: &'a i32, b: &'a i32) -> &'a i32{
    if a>b {a} else {b}
}

fn main(){
    let m = 12;
    let n = 13;
    let result = compare_and_return(&m,&n);
    println!("{:?}", result);
    {
        let k = 34;
        let result2 = compare_and_return(&m, &k);
    }
    // println!("{:?}", result2);
    //Doesn't work because the result2 doesn't exist in this scope.
}
