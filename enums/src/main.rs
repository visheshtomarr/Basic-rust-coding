#![allow(dead_code)]
#[derive(Debug)]

enum Payment {
    Cash,
    Cheque,
    Card(i32)       // This Card field will have an integer associated with it.
}

fn main() {
    let number = 24 ;
    
    // Match number with various ranges and single integer values
    match number {
        1..=10 => println!("Number found between 1 and 10"),
        11..=20 => println!("Number found between 11 and 20") ,  // Always need to give inclusive range values while using in match arm
        _ => println!("Number is greater than 20") 
    }

    // Defining a variable through the result getting from "MATCH" expression
    let is_even = match number % 2 {
        0 => true,
        1 => false,
        _ => false
    } ;

    if is_even {
        println!("Number {} is even", number) ;
    }
    else {
        println!("Number {} is odd", number) ;
    }

    // Initializing variables of enum type, "Payment".
    let p1 = Payment::Cash ;
    // Passing the associated integer value with Card field here. 
    let p2 = Payment::Card(5000) ;      

    match p1 {
        Payment::Cash => println!("Payment by Cash"),
        Payment::Cheque => println!("Payment by Cheque"),
        Payment::Card(amt) => println!("Amount {} is paid by Card", amt)
    }
    match p2 {
        Payment::Cash => println!("Payment by Cash"),
        Payment::Cheque => println!("Payment by Cheque"),
        Payment::Card(amt) => println!("Amount {} is paid by Card", amt)
    }
}
