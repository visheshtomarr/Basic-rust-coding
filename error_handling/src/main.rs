// // Q1. Write a Rust function that parses a date string in the format "YYYY-MM-DD" and returns Result<(i32, u32, u32), &str>
// // indicating the year, month, and day, or an error message.

// fn parse_date(date_str: &str) -> Result<(i32, u32, u32), &str>{
//     // Splitting date into 3 parts by delimiter "-" 
//     let parts: Vec<&str> = date_str.split('-').collect() ;
       
//     // Checking whether the passed date has day, month and year or not 
//     if parts.len() != 3 {
//         return Err("Date is not entered in the format: YYYY-MM-DD !!") ;
//     }

//     let day: u32 = match parts[2].parse() {
//         Ok(num) => num,
//         Err(_) => return Err("Date should be entered in numeric !!")
//     } ;
//     let month: u32 = match parts[1].parse() {
//         Ok(num) => num,
//         Err(_) => return Err("Month should be entered in numeric !!")
//     } ;
//     let year: i32 = match parts[1].parse() {
//         Ok(num) => num,
//         Err(_) => return Err("Year should be entered in numeric !!")
//     } ;

//     if month < 1 || month > 12 || day < 1 || day > 31 {
//         return Err("Invalid date format !! Month should be between 1 and 12, date should be between 1 and 31.") ;
//     }

//     Ok((year,month,day)) 
// }
// fn main() {
//     let dt1 = "2000-10-25" ;
//     let dt2 = "1975-11-25" ;
//     let dt3 = "2001-00-25" ;
//     let dt4 = "200d-02-2c" ;

//     match parse_date(dt1) {
//         Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
//         Err(err_msg) => println!("Error: {}", err_msg)  
//     }

//     match parse_date(dt2) {
//         Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
//         Err(err_msg) => println!("Error: {}", err_msg)  
//     }

//     match parse_date(dt3) {
//         Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
//         Err(err_msg) => println!("Error: {}", err_msg)  
//     }

//     match parse_date(dt4) {
//         Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
//         Err(err_msg) => println!("Error: {}", err_msg)  
//     }
// }

//-----------------------------------------------------------------------------------------------------------------

// // Q2. Write a Rust function that takes a string and returns Option<usize> representing the string length, returning None for empty strings.

// fn string_len(s: String) -> Option<usize> {
//     // fn "is_empty()" will check whether the string is empty or not
//     if s.is_empty(){
//         None
//     }
//     else {
//         Some(s.len())
//     }
// }

// fn main() {
//     let s1 = "My name is Vishesh".to_string() ;
//     let s2 = "".to_string() ;
    
//     // Checking for a non-empty string
//     match string_len(s1) {
//         Some(len) => println!("Length of string: {}", len) ,
//         None => println!("String is empty!!") 
//     }
//     // Checking for an empty string
//     match string_len(s2) {
//         Some(len) => println!("Length of string: {}", len) ,
//         None => println!("String is empty!!")
//     }
// }

//-----------------------------------------------------------------------------------------------------------------

// // Q3. Write a Rust function that converts a hexadecimal string to an integer and returns Option<u64>, returning None for invalid input.

// fn hex_to_int(s: &str) -> Option<u64> {
//     // Trying to parse a hexadecimal string to a u64 integer  
//     match u64::from_str_radix(s, 16) {
//         // If parsing successful, wrapping the u64 integer value into a "Some()"
//         Ok(parsed_int) => Some(parsed_int),
//         // If parsing failed, returning a "None"
//         Err(_) => None
//     }
// }
// fn main() {
//     // Valid hex-string
//     let s1 = "BF3C" ;
    
//     // invalid hex-string
//     let s2 = "IJKL" ;

//     match hex_to_int(s1) {
//         Some(parsed_int) => println!("Parsed integer: {}", parsed_int),
//         None => println!("Invalid hexadecimal string!!")
//     }

//     match hex_to_int(s2) {
//         Some(parsed_int) => println!("Parsed integer: {}", parsed_int),
//         None => println!("Invalid hexadecimal string!!")
//     }
// }

//-----------------------------------------------------------------------------------------------------------------

// Q4. Write a Rust function that divides two numbers and returns Result<f64, &'static str>, indicating success or division by zero error.

fn divides(a: f64, b: f64) -> Result<f64, &'static str> {
    if b != 0.0 {
        Ok(a/b)
    }
    else {
        Err("Not defined!. Cannot divide a number by 0")
    }
}
fn main() {
    let num1 = 5.0 ;
    let num2 = 2.0 ;
    let num0 = 0.0 ;

    match divides(num1, num2) {
        Ok(res) => println!("Result of division of 1st and 2nd number: {}", res) ,
        Err(err_msg) => println!("{}", err_msg) 
    }

    match divides(num1, num0) {
        Ok(res) => println!("Result of division of 1st and 2nd number: {}", res) ,
        Err(err_msg) => println!("{}", err_msg) 
    }
}