// #![allow(dead_code)]
// #[derive(Debug)]

// enum Payment {
//     Cash,
//     Cheque,
//     Card(i32)       // This Card field will have an integer associated with it.
// }

// fn main() {
//     let number = 24 ;
    
//     // Match number with various ranges and single integer values
//     match number {
//         1..=10 => println!("Number found between 1 and 10"),
//         11..=20 => println!("Number found between 11 and 20") ,  // Always need to give inclusive range values while using in match arm
//         _ => println!("Number is greater than 20") 
//     }

//     // Defining a variable through the result getting from "MATCH" expression
//     let is_even = match number % 2 {
//         0 => true,
//         1 => false,
//         _ => false
//     } ;

//     if is_even {
//         println!("Number {} is even", number) ;
//     }
//     else {
//         println!("Number {} is odd", number) ;
//     }

//     // Initializing variables of enum type, "Payment".
//     let p1 = Payment::Cash ;
//     // Passing the associated integer value with Card field here. 
//     let p2 = Payment::Card(5000) ;      

//     match p1 {
//         Payment::Cash => println!("Payment by Cash"),
//         Payment::Cheque => println!("Payment by Cheque"),
//         Payment::Card(amt) => println!("Amount {} is paid by Card", amt)
//     }
//     match p2 {
//         Payment::Cash => println!("Payment by Cash"),
//         Payment::Cheque => println!("Payment by Cheque"),
//         Payment::Card(amt) => println!("Amount {} is paid by Card", amt)
//     }
// }

// Task: Write a function that takes an enum variant and performs different operations based on the variant.

#![allow(dead_code)]
#[derive(Debug)]

enum Operation {
    Add(i32, i32),
    Sub(i32, i32), 
    Multi(i32, i32)
}

fn math_oper(some_oper: Operation) -> i32 {
    match some_oper {
        Operation::Add(a, b) => a + b,
        Operation::Sub(a, b) => a - b,
        Operation::Multi(a, b) => a * b 
    }
}

fn main() {
    let add = Operation::Add(10, 15) ;
    let sub = Operation::Sub(30, 15) ;
    let product = Operation::Multi(5, 10) ;

    println!("Addition = {}, subtraction = {} and multiplication = {}", math_oper(add), math_oper(sub), math_oper(product)) ;
}