// // Q1. Performing basic operations on vector.
// fn main() {

//     // Using Vec method to initialize a vector.
//     let mut v1 = Vec::new() ;

//     // Using "vec!" to initialize a hard-coded vector.
//     let mut v2 = vec![10, 20, 30, 40, 50] ;
    
//     // Inserting elements into Vector using for loop
//     for i in 1..=10{
//         v1.push(i) ;
//     }
//     print!("Vector v1 = ") ;
//     println!("{:?}", v1) ;
//     print!("Vector v2 = ") ;
//     println!("{:?}", v2) ;

//     // A `Vector` can also be iterated over while the iteration
//     // count is enumerated in a separate variable (`i`)
//     for (i, value) in v1.iter().enumerate() {
//         println!("Value at {} is {}", i, value) ;
//     }

//     // Iterating over a vector without .enumerate()
//     print!("Contents of Vector v2: ") ;
//     for i in v2.iter(){
//         print!("{} ", i) ;
//     }
//     println!("") ;

//     // Pushing values into v2 with multiplying values with 10
//     for i in 6..=10{
//         v2.push(i * 10) ;
//     } 
//     println!("Updated v2 = {:?}", v2) ;

//     // Removing values from a vector
//     println!("Pop last element from v1: {:?}", v1.pop()) ;
//     println!("Pop last element from v2: {:?}", v2.pop()) ;

//     println!("Length of v1: {}, Capacity of v1: {}", v1.len(), v1.capacity()) ;
//     println!("Length of v2: {}, Capacity of v2: {}", v2.len(), v2.capacity()) ;

//     println!("Updated v1 after popping values = {:?}", v1) ;
//     println!("Updated v2 after popping values = {:?}", v2) ;

//     // Changing values of v1 using .iter_mut() method
//     for i in v1.iter_mut(){
//         *i *= 10 ;
//     }
//     println!("Updated v1 after using .iter_mut() method: {:?}", v1) ;

//     // We can compare two vector directly
//     if v1 == v2 {
//         println!("Vector v1 is equal to vector v2") ;
//         println!("v1: {:?}", v1) ;
//         println!("v2: {:?}", v2) ;
//     }
//     else {
//         println!("Vector v1 is not equal to vector v2") ;
//         println!("v1: {:?}", v1) ;
//         println!("v2: {:?}", v2) ;
//     }

//     // Using .concat() method to conact two vectors
//     let v3 = [v1,v2].concat() ;
//     println!("v3: {:?}", v3) ;

//     // Using .collect() function to create a vector from range
//     let v4: Vec<i32> = (1..=10).collect() ;

//     // Finding an element "6" in vector v4
//     let does_exist = v4.contains(&6) ;
//     if does_exist {
//         println!("6 is present in vector v4") ;
//     }
//     else {
//         println!("6 is not present in vector v4") ;
//     }

//     // Creating a vector "Slice"
//     let v4_slice = &v4[3..=7] ;
//     println!("Vector slice: {:?}", v4_slice) ;
// }

//------------------------------------------------------------------------------------------

// // Q2. Write a Rust program that creates a vector of floating-point numbers. Map each element of the vector to its square root.
// // Slice the resulting vector to get a sub-vector containing the elements from index 2 to index 6. Print the sub-vector.
    
// fn main () {
//     let v: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0] ;
//     println!("Second vector: {:?}", v) ;
        
//     let vec_with_sqrt: Vec<f32> = v.into_iter().map(|x| x.sqrt()).collect() ;
//     let sqrt_vec = &vec_with_sqrt[2..7] ;
//     println!("Square rooted vector: {:?}", sqrt_vec) ;
// }

//------------------------------------------------------------------------------------------

// Q3. Write a Rust program that creates an array of strings with size 6 and initializes it with days of the week.
// Remove the last 2 elements from the array and slice it to get a sub-array containing the first 3 days.
// Print the resulting sub-array.

fn main() {
    let mut week_days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"] ;
    let n = week_days.len() ;
    println!("Weekdays: {:?}", week_days) ;
    week_days.drain(n-2..) ;
    println!("Removing last 2 days from weekdays: {:?}", week_days) ;

    let first_three_days = &week_days[0..3] ;
    println!("First 3 days of week: {:?}", first_three_days) ;
}