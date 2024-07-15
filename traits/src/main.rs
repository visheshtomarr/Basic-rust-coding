// // Implementing a custom Debug trait for a struct name Student that contains your name, class and roll number.
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #[derive(Debug)]

// struct Student {
//     name: String,
//     class: i32,
//     roll: i32,
// }

// trait Debug {
//     fn return_struct(&self, name: &String, class: i32, roll: i32) -> Self ;
// }
// impl Debug for Student {
//     fn return_struct(&self, name: &String, class: i32, roll: i32) -> Self{
//         Self { 
//             name: name.to_string(), 
//             class,
//             roll 
//         }
//     }
// }

// fn main() {
//     let st = Student{
//         name: String::from("Vishesh"),
//         class: 12,
//         roll: 54
//     } ;
//     println!("{:?}", st.return_struct(&st.name, st.class, st.roll)) ;
// }

//------------------------------------------------------------------------------------------

// Q2. Implementing custom Drop trait for a struct name Student that contains your name, age and roll number. 
// It should return Roll number <roll number> has name <name> with age <age> and is a <junior/senior>. 
//Being Junior or Senior depends on age (18 or above).

#![allow(dead_code)]
#[derive(Debug)]

struct Student{
    name: String,
    roll: u32,
    age : u32,
}

trait Drop {
    fn is_junior_or_senior(&self, n: u32) -> String ;
}

impl Drop for Student {
    fn is_junior_or_senior(&self, n: u32) -> String {
        if n >= 18{
            String::from("senior")
        }
        else{
            String::from("junior")
        }
    }
}

fn main(){
    let st = Student{
        name: "Vishesh".to_string(),
        roll: 54,
        age: 17
    } ;

    println!("Roll number {} has name {} with age {} is a {}", st.roll, st.name, st.age, st.is_junior_or_senior(st.age)) ;
}