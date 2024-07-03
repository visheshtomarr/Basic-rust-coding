// Q1. Write a Rust function that takes a closure as an argument and applies it to a given number.

// fn takes_closure <F : FnOnce(i32) -> i32> (closure: F , n: i32) -> i32{
//     closure(n)
// }
// fn main() {
    
//     let closure = |x| x * x ;
    
//     let result = takes_closure(closure, 6) ;

//     println!("result: {}", result) ;
// }

//-------------------------------------------------------------------------------------------------------------------

//Q2. Write a Rust function that takes a closure and two numbers, and returns the result of applying the closure to the two numbers.
use std::io ;

fn apply_closure_to_numbers<F: FnOnce(i32,i32) -> i32> (closure: F, a: i32, b: i32) -> i32{
    closure(a, b)
}

fn main(){
    
    println!("Enter first number: ") ;
    let mut a = String::new() ;

    io::stdin()
    .read_line(&mut a)
    .expect("Failed in reading input") ;

    let a: i32 = a.trim().parse().expect("Failed in parsing") ;
    println!("Enter second number: ") ;

    let mut b = String::new() ;

    io::stdin()
    .read_line(&mut b)
    .expect("Failed in reading input") ;

    let b: i32 = b.trim().parse().expect("Failed in parsing") ;

    // Defining a closure that will add two numbers
    let add_closure = |a: i32, b: i32| a + b ;

    let res = apply_closure_to_numbers(add_closure, a, b) ;
    println!("Result of passing our closure in function: {}", res) ;

}
