use std::io ;

fn main() {
    println!("Enter number for fibonacci sequence: ") ;
    let mut n = String::new() ;

    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read input") ;

    let n: u32 = n.trim().parse().expect("Failed in parsing input !!") ;

    let mut i = 0 ;
    let mut a = 0 ;
    let mut b = 1 ;
    let mut sum = 1 ;
    while i != n-1{
        let c: u32 = a + b ;
        print!("{} ", c) ;
        even_odd(c) ;
        if c == 1{
            println!("") ;
            print!("{} Unique", c) ;
        }
        else if is_prime(c){
            print!("Prime") ;
        }
        else{
            print!("Not Prime") ;
        }
        a = b ;
        b = c ;
        i += 1 ;
        sum += c ;
        println!("") ;    
    }
    println!("The sum of the numbers is: {}", sum) ;
}

fn even_odd(x : u32){
    if x == 1 {
        print!("Unique") ;
    }
    else if x % 2 == 0{
        print!("Even ") ;
    }
    else{
        print!("Odd ") ;
    }
}

fn is_prime(x: u32) -> bool {
    if x == 1 {
        return false ; 
    }
    else {
        let mut i = 2 ;
        while i*i <= x {
            if x % i == 0{
                return false;
            }
            i += 1 ;
        }
    }
    true 
}