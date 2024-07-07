// Q1. Write a Rust function that parses a date string in the format "YYYY-MM-DD" and returns Result<(i32, u32, u32), &str>
// indicating the year, month, and day, or an error message.

fn parse_date(date_str: &str) -> Result<(i32, u32, u32), &str>{
    // Splitting date into 3 parts by delimiter "-" 
    let parts: Vec<&str> = date_str.split('-').collect() ;
       
    // Checking whether the passed date has day, month and year or not 
    if parts.len() != 3 {
        return Err("Date is not entered in the format: YYYY-MM-DD !!") ;
    }

    let day: u32 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Date should be entered in numeric !!")
    } ;
    let month: u32 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => return Err("Month should be entered in numeric !!")
    } ;
    let year: i32 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => return Err("Year should be entered in numeric !!")
    } ;

    if month < 1 || month > 12 || day < 1 || day > 31 {
        return Err("Invalid date format !! Month should be between 1 and 12, date should be between 1 and 31.") ;
    }

    Ok((year,month,day)) 
}
fn main() {
    let dt1 = "2000-10-25" ;
    let dt2 = "1975-11-25" ;
    let dt3 = "2001-00-25" ;
    let dt4 = "200d-02-2c" ;

    match parse_date(dt1) {
        Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
        Err(err_msg) => println!("Error: {}", err_msg)  
    }

    match parse_date(dt2) {
        Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
        Err(err_msg) => println!("Error: {}", err_msg)  
    }

    match parse_date(dt3) {
        Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
        Err(err_msg) => println!("Error: {}", err_msg)  
    }

    match parse_date(dt4) {
        Ok((year, month, day)) => println!("Parsed date: Year = {}, Month = {}, Day = {}", year, month, day) ,
        Err(err_msg) => println!("Error: {}", err_msg)  
    }
}