// Implementing a custom Debug trait for a struct name Student that contains your name, class and roll number.
#![allow(dead_code)]
#![allow(unused_variables)]
#[derive(Debug)]

struct Student {
    name: String,
    class: i32,
    roll: i32,
}

trait Debug {
    fn return_struct(&self, name: &String, class: i32, roll: i32) -> Self ;
}
impl Debug for Student {
    fn return_struct(&self, name: &String, class: i32, roll: i32) -> Self{
        Self { 
            name: name.to_string(), 
            class,
            roll 
        }
    }
}

fn main() {
    let st = Student{
        name: String::from("Vishesh"),
        class: 12,
        roll: 54
    } ;
    println!("{:?}", st.return_struct(&st.name, st.class, st.roll)) ;
}
