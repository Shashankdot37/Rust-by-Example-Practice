//Higher Order Function -- Function that takes one or more functions and produce more useful functions
//Program to find the sum of all the numbers with odd squares under 1000
fn odd_check(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    let mut acc = 0; //accumulator
    let upper_limit = 1000;

    //Imperative Approach

    for i in 0.. {
        let i_squared = i * i;

        if i_squared >= upper_limit {
            break;
        } else if odd_check(i_squared) {
            acc = acc + i_squared;
        }
    }

    //Functional Approach

    let sum_of_app_odd: u32 = (0..)
        .map(|i| i * i)
        .take_while(|&i_squared| i_squared <= upper_limit)
        .filter(|&i_squared| odd_check(i_squared))
        .sum();

    println!("{:?}", sum_of_app_odd);
}
