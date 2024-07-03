// Q1. Write a Rust function that takes a closure as an argument and applies it to a given number.

fn takes_closure <F : FnOnce(i32) -> i32> (closure: F , n: i32) -> i32{
    closure(n)
}
fn main() {
    
    let closure = |x| x * x ;
    
    let result = takes_closure(closure, 6) ;

    println!("result: {}", result) ;
}
