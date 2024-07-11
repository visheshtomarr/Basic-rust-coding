// // Q1. Write a program that will print a String and reverse every word in the string whose length is greater than or equal to 5.
// #![allow(unused_variables)]
// fn main() {
//     let s = "My name is Vishesh".to_string() ;

//     let mut res = String::new() ;
//     let mut temp = String::new() ;

//     for (i, j) in s.char_indices() {
//         if j == ' ' {
//                 if temp.len() < 5 {
//                     res.push_str(&temp) ;
//                     temp.clear() ;
//                 } 
//                 else {
//                     let d: String =  temp.chars().rev().collect() ;
//                      res.push_str(&d) ;
//                     temp.clear() ;
//                 }
//                 res.push(' ') ;
//         }
//         else {
//             temp.push(j) ;
//         }
//     }
//     if temp.len() >= 5 {
//         let d:String = temp.chars().rev().collect();
//         res.push_str(&d) ;
//     }
//     else if temp.len() < 5 {
//         res.push_str(&temp);
//     }
//     println!("{}", res) ;
// }

//---------------------------------------------------------------------------------------

// Q2. Write a program that will split the array based on a given size and print resulting arrays of that size from the original array.
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::vec;
use std::io ;

fn main() {
    println!("Enter a split value greater than 0:") ;

    let mut s = String::new() ;
    io::stdin()
    .read_line(&mut s)
    .expect("Failed in reading input") ;

    let s: usize = s.trim().parse().expect("Failed in parsing input") ;
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10] ;

    let mut res = Vec::new() ;
    for i in 0..vec.len() {
        if i % s == 0 {
            if i + s < vec.len() {
               res.push(&vec[i..i+s]) ;       
            }
            else {
                res.push(&vec[i..vec.len()]) ;
            }
        }
    }
    println!("{:?}", res) ;
}