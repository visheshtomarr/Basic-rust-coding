// // Q1. Creating structure for a student that has a name, roll number and marks in cgpa.
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #[derive(Debug)]

// struct Student{
//     name: String,
//     rollno: u32,
//     marks: f32
// }
// fn main() {
    
//     let mut st1 = Student{
//         name: String::from("ABCD"),
//         rollno: 10,
//         marks: 7.6 
//     } ;
//     println!("st1 is {:?}", st1) ;

//     show_name(&st1) ;                       // Passing reference of our st1 Student struct.
//     change_rollno(&mut st1) ;               // Passing a mutable reference of our st1 Student struct.
//     st1.marks = 8.2 ;
//     println!("st1 is {:?}", st1) ;

//     let st2 = return_student(String::from("PQRS"), 15, 8.9) ;

//     println!("st2 is {:?}", st2) ;

//     //If we need to create a student with same name and marks as that of st2 but has a different rollno ->
//     let st3 = Student{
//         rollno: 20,
//         ..st2           // This annotation of "..st2" means that all the data except mentioned otherwise above this line 
//     } ;                 // will be copied from the student st2.  

//     println!("st3 is {:?}", st3) ;

// }
// // To keep the ownership with the variable "st1" itself, we pass a reference of our struct.
// fn show_name(s: &Student){                  
//     println!("Name is {}", s.name) ; 
// }

// // To change the value of a field inside a struct, we will pass a mutable reference of our struct.
// fn change_rollno(s: &mut Student){          
//     s.rollno += 1 ; 
// }

// // This function will take in the values of the fields present in our struct and it will return a Student struct.
// fn return_student(name: String, rollno: u32, marks: f32) -> Student{        
//         Student{
//             name,       // When the fields on the right and that in our struct are same, we can directly provide the field names.
//             rollno,
//             marks
//         }
// }

//-------------------------------------------------------------------------------------------------------

// // Q2. Write a program to compare two dates entered by user. Make a structure named Date to store the elements day, 
// // month and year to store the dates. If the dates are equal, display "Dates are equal" otherwise display "Dates are not equal".
// #![allow(unused_variables)]
// #![allow(dead_code)]
// #[derive(Debug)]
// #[derive(PartialEq)]
// struct Date{
//     day: u32,
//     month: u32,
//     year: u32
// }

// use std::io ;
// fn main(){
//         println!("Enter first Date in DDMMYYYY format: ") ;
//         let mut date1 = String::new() ;

//         io::stdin()
//         .read_line(&mut date1)
//         .expect("Failed to read input") ;

//         let date1: String = date1.trim().parse().expect("Failed in parsing") ;

//         println!("Enter second Date in DDMMYYYY format: ") ;
//         let mut date2 = String::new() ;

//         io::stdin()
//         .read_line(&mut date2)
//         .expect("Failed to read input") ;

//         let date2: String = date2.trim().parse().expect("Failed in parsing") ;

//         let day1: u32 = date1[0..2].parse().unwrap() ; 
//         let month1: u32 = date1[2..4].parse().unwrap() ;
//         let year1: u32 = date1[4..8].parse().unwrap() ;

//         let day2: u32 = date2[0..2].parse().unwrap() ; 
//         let month2: u32 = date2[2..4].parse().unwrap() ;
//         let year2: u32 = date2[4..8].parse().unwrap() ;

//         let dt1 = Date{
//             day: day1,
//             month: month1,
//             year: year1
//         } ;

//         let dt2 = Date{
//             day: day2,
//             month: month2,
//             year: year2
//         } ;

//         println!("{:?}", dt1) ;
//         println!("{:?}", dt2) ;

//         if dt1 == dt2 {
//             println!("Dates are equal") ;
//         }
//         else{
//             println!("Dates are not equal") ;
//         }
//     }

//---------------------------------------------------------------------------------------------

// // Q3. Create a structure named Date having day, month and year as its elements. 
// // Store the current date in the structure. Now add 45 days to the current date and display the final date.

// #![allow(unused_variables)]
// #![allow(dead_code)]
// #[derive(Debug)]

// struct Date{
//     day: u32,
//     month: u32,
//     year: u32
// }

// fn main(){
//     let mut dt = Date{
//         day: 12,
//         month: 7,
//         year: 2024
//     } ;

//     println!("Current date : {:?}", dt) ;

//     update_date(&mut dt) ;

//     println!("Date after adding 45 days in the current date: {:?}", dt) ;

// }

// fn update_date(date: &mut Date){
//         date.day = 26 ;
//         date.month = 8 ;
// }

//---------------------------------------------------------------------------------------------

// // Q4. Creating associated functions for a Student struct which will Lowercase the Name of the student
// // and the other function will tell if the student passed or not according to his/her marks.

// #![allow(dead_code)]
// #[derive(Debug)]

// struct Student{
//     name: String,
//     rollno: u32,
//     marks: f32
// }

// impl Student{
//     fn is_passed(&self) -> bool {
//         self.marks > 6.0
//     }
    
//     fn to_lowercase(&self) -> Self { 
//         Self {
//             name: self.name.to_lowercase(),
//             ..*self
//         }
//     }
// }

// fn main() {
//     let st1 = Student{
//         name: String::from("ABCD"),
//         rollno: 44,
//         marks: 8.6
//     } ;

//     println!("st1 is {:?}", st1) ;
//     println!("st1 after function call is {:?}", st1.to_lowercase()) ;

//     println!("st1 is passed: {}", st1.is_passed()) ;
// }

//---------------------------------------------------------------------------------------------

// // Q5. Creating a trait to find whether 2 different flights can travel upto the given distance or not.
// #![allow(dead_code)]
// #[derive(Debug)]
// struct Vistara {
//     model: String,
//     range: u32
// }

// #[derive(Debug)]
// struct AirIndia {
//     model: String,
//     range: u32
// }

// // This is how we define a trait
// trait Flight{
//     fn is_valid(&self, distance: u32) -> bool ;
// }

// // Implementing trait Flight for struct Vistara
// impl Flight for Vistara {
//     fn is_valid(&self, distance: u32) -> bool {
//         self.range + 200 > distance
//     }
// }

// // Implementing trait Flight for struct AirIndia
// impl Flight for AirIndia {
//     fn is_valid(&self, distance: u32) -> bool {
//         self.range + 300 > distance
//     }
// }

// fn main(){
//     let vistara = Vistara {
//         model: String::from("A437") ,
//         range: 1500
//     } ;
    
//     let airindia = AirIndia{
//         model: String::from("T289") ,
//         range: 2000
//     } ;

//     println!("Flight vistara is valid for 1000km: {}", vistara.is_valid(1000)) ;
//     println!("Flight airindia is valid for 2500km: {}", airindia.is_valid(2500)) ;

// }

//---------------------------------------------------------------------------------------------

// Q6. Use pattern matching to find that whether a point lies on X-Axis, Y-Axis or on which quadrant.

#![allow(dead_code)]
#![allow(unused_variables)]
#[derive(Debug)]

struct Point {
    x: i32, 
    y: i32
} 

impl Point {
    fn position(&self, x: i32, y: i32) {
        let p = Point{x, y} ;
        match p {
            Point{x: 0, y: 0} => println!("Postion of point is on origin"),
            Point{x: 0, y} => println!("Position of point is on Y-axis"),
            Point{x, y: 0} => println!("Position of point is on X-axis"),
            Point{x, y} => if x > 0 && y > 0 {
                println!("Position of point is in 1st Quadrant")
            }
            else if x < 0 && y > 0 {
                println!("Position of point is in 2nd Quadrant")
            }
            else if x < 0 && y < 0 {
                println!("Position of point is in 3rd Quadrant")
            }
            else if x > 0 && y < 0{
                println!("Position of point is in 4th Quadrant")
            }
        }
    }
}

fn main() {
    let p1 = Point{x: 0, y: 0} ;
    let p2 = Point{x: 5, y: 0} ;
    let p3 = Point{x: 0, y: 3} ;
    let p4 = Point{x: 1, y: 1} ;
    let p5 = Point{x: -3, y: 2} ;
    let p6 = Point{x: -4, y: -6} ;
    let p7 = Point{x: 2, y: -5} ;

    println!("p1: {:?}", p1) ;
    println!("{:?}", p1.position(p1.x, p1.y));
    
    println!("p2: {:?}", p2) ;
    println!("{:?}", p2.position(p2.x, p2.y));

    println!("p3: {:?}", p3) ;
    println!("{:?}", p3.position(p3.x, p3.y));

    println!("p4: {:?}", p4) ;
    println!("{:?}", p4.position(p4.x, p4.y));

    println!("p5: {:?}", p5) ;
    println!("{:?}", p5.position(p5.x, p5.y));

    println!("p6: {:?}", p6) ;
    println!("{:?}", p6.position(p6.x, p6.y));

    println!("p7: {:?}", p7) ;
    println!("{:?}", p7.position(p7.x, p7.y));
    
}