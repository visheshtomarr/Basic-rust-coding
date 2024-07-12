// Q1. Creating structure for a student that has a name, roll number and marks in cgpa.
#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Debug)]

struct Student{
    name: String,
    rollno: u32,
    marks: f32
}
fn main() {
    
    let mut st1 = Student{
        name: String::from("ABCD"),
        rollno: 10,
        marks: 7.6 
    } ;
    println!("st1 is {:?}", st1) ;

    show_name(&st1) ;                       // Passing reference of our st1 Student struct.
    change_rollno(&mut st1) ;               // Passing a mutable reference of our st1 Student struct.
    st1.marks = 8.2 ;
    println!("st1 is {:?}", st1) ;

    let st2 = return_student(String::from("PQRS"), 15, 8.9) ;

    println!("st2 is {:?}", st2) ;

    //If we need to create a student with same name and marks as that of st2 but has a different rollno ->
    let st3 = Student{
        rollno: 20,
        ..st2           // This annotation of "..st2" means that all the data except mentioned otherwise above this line 
    } ;                 // will be copied from the student st2.  

    println!("st3 is {:?}", st3) ;

}
// To keep the ownership with the variable "st1" itself, we pass a reference of our struct.
fn show_name(s: &Student){                  
    println!("Name is {}", s.name) ; 
}

// To change the value of a field inside a struct, we will pass a mutable reference of our struct.
fn change_rollno(s: &mut Student){          
    s.rollno += 1 ; 
}

// This function will take in the values of the fields present in our struct and it will return a Student struct.
fn return_student(name: String, rollno: u32, marks: f32) -> Student{        
        Student{
            name,       // When the fields on the right and that in our struct are same, we can directly provide the field names.
            rollno,
            marks
        }
}