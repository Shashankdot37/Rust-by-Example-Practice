//generics in function

struct A; //concrete type
struct S1(A); //concrete type as it takes type specified parameter
struct S2<T>(T); //generic type

fn func1(s:S1){} //concrete type function i.e non-generic function

fn func2(s:S2<A>){} //concrete type function as the type "A" has been specified as type parameter

fn func3(s:S2<i32>){} //concrete type function as i32 type is specified

fn func4(s:S2<T>){} //generic function

fn main(){
    func1(S1(A));
    func2(S2(A));
    func3(S2(2));
    func4::<char>(S2('a')); //explicitly specifying the type char
    func4(S2('b')); //implicitly specifying the type char 
}
