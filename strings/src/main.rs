// Q1. Write a program that will print a String and reverse every word in the string whose length is greater than or equal to 5.
#![allow(unused_variables)]
fn main() {
    let s = "My name is Vishesh".to_string() ;

    let mut res = String::new() ;
    let mut temp = String::new() ;

    for (i, j) in s.char_indices() {
        if j == ' ' {
                if temp.len() < 5 {
                    res.push_str(&temp) ;
                    temp.clear() ;
                } 
                else {
                    let d: String =  temp.chars().rev().collect() ;
                     res.push_str(&d) ;
                    temp.clear() ;
                }
                res.push(' ') ;
        }
        else {
            temp.push(j) ;
        }
    }
    if temp.len() >= 5 {
        let d:String = temp.chars().rev().collect();
        res.push_str(&d) ;
    }
    else if temp.len() < 5 {
        res.push_str(&temp);
    }
    println!("{}", res) ;
}