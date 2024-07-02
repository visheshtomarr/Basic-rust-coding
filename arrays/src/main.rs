// Write a Rust program to create an array of integers with size 7 and initialize it with random values.
// Calculate and print the sum of all the elements in the array.

use rand::Rng ;
use std::io ;
fn main() {
    let mut arr : [i32; 7] =  [0; 7] ;
    let mut rng = rand::thread_rng() ;

    for i in 0..7 {
        arr[i] = rng.gen_range(0..20) ;
    }

    println!("First array: {:?}", arr) ;
    let mut sum = 0 ;
    for i in arr.iter(){
        sum += i ;
    }
    println!("Sum of elements: {}", sum) ;

    // Taking input from user and finding it in the generated range.

    let mut inp = String::new();
    io::stdin()
    .read_line(&mut inp)
    .expect("Failed to read input") ;

    let inp: i32 = inp.trim().parse().expect("Failed in parsing the input") ;
    
    let mut is_present = false ;
    for i in arr.iter(){
        if inp == *i {
            is_present = true ;
        }
    }

    if is_present {
        println!("Number entered by user is present") ;
    }
    else {
        println!("Number entered by user is not present") ;
    }

    // Creating a second array and from that creating a sub-array consisting of only even numbers
    let mut arr2: [i32; 9] = [0; 9] ;
    let mut rng2 = rand::thread_rng() ;

    let mut vec = Vec::new() ;
    for i in 0..9 {
        arr2[i] = rng2.gen_range(20..40) ;
    }

    println!("Second array: {:?}", arr2) ;

    for i in arr2.iter_mut(){
        if *i % 2 == 0{
            vec.push(*i) ;
        }
    }
    println!("Updated array: {:?}", vec) ;
}