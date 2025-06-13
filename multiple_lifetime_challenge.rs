// Challenge : Multi-Lifetime Reference Resolver
// Create a function choose_earlier<'a, 'b>(a: &'a str, b: &'b str) -> &str that:
//     Takes two string slices (&str) with different lifetimes ('a, 'b)
//     Returns the one that comes first alphabetically (.cmp() or <)
//     The function must return a &str with a lifetime that is compatible with both input lifetimes (hint: -> &'a str or -> &'b str won't always work—consider the shortest one of both)
//     You’ll need to explicitly annotate and justify the lifetime of the returned reference.
//  In main():
//     Define two strings: one declared outside a scope and one inside a scope block.
//     Use choose_earlier() to compare them.
//     Try printing the result outside the block.
//     Observe what the compiler tells you. Comment it with your interpretation.

use std::cmp::Ordering;

fn choose_earlier<'a, 'b>(a: &'a str, b: &'b str) -> &'a str
where
    'b:'a
{
    if a.cmp(&b) == Ordering::Less { a } else { b }
}

fn main() {
    let str1 = "Hello";
    let _str2 = "World";
    {
        let _str3 = "Sansaar";
    }

    // let result = choose_earlier(str1, _str3); //Doesn't work because the _str3 is not found in this scope, rather terminated at the end of the previous scope.
    let result = choose_earlier(str1, _str2); //valid. 
    println!("{:?}", result);
}
