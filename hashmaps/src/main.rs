use std::collections::HashMap ;
// // Q1. Creating a hashmap by inserting names and ages of different accounts and performing various hashmap operations on it.

// fn main() {
//     let mut map = HashMap::new() ;

//     map.insert(String::from("Alice"), 23) ;
//     map.insert(String::from("Bob"), 25) ;
//     map.insert(String::from("Charlie"), 28) ;
//     map.insert(String::from("Dave"), 20) ;

//     // Printing map using debug print
//     println!("Map: {:?}", map) ;

//     // Iterating over a map and prinitng using the iterators
//     for (k,v) in map.iter(){
//         println!("Key: {} | Value: {}", k, v) ;
//     }

//     // Checking whether a key is present in Hashmap or not
//     let key_to_check = "Dave".to_string() ;

//     if map.contains_key(&key_to_check){
//         println!("Key: {}, is present in our Hashmap", key_to_check) ;
//     }
//     else{
//         println!("Key: {}, is present in our Hashmap", key_to_check) ;
//     }
    
//     // Retrieving the value associated with a specific key from HashMap and printing it
//     let key_to_retrieve = String::from("Charlie") ;

//     // This ".get()" method will result in an Option<T>
//     let val = map.get(&key_to_retrieve) ;

//     // Using match to check value associated with our given key
//     match val {
//         Some(&val) => println!("Value associated with key: {} is {}", key_to_retrieve, val),
//         None => println!("No value present for key: {}", key_to_retrieve)
//     }

//     // Updating the value associated with a key in HashMap and print the updated HashMap.
//     let key_to_update = String::from("Charlie") ;
//     let new_value = 18 ;

//     // Retrieving a mutable Option value associated with our key using .get_mut() method
//     let updated_value = map.get_mut(&key_to_update) ;

//     match updated_value {
//         // If Option "updated_value" has Some(value), it will be updated with "new_value" 
//         Some(value) => *value = new_value,
//         None => println!("Key {} is not present in Hashmap", key_to_update) 
//     }
//     println!("Updated map: {:?}", map) ;

//     // Using .entry() method to insert into our Map.
//     // .entry() will check whether the passed key is present in our map or not.
//     // If key is present, associated value will not change. 
//     // If key is not present, .or_insert() method will insert value given into it. 
//     map.entry(String::from("Eve")).or_insert(30) ;

//     println!("Updated map after entering key-pair: {:?}", map) ;

//     // Removing a key-value pair from HashMap and printing the HashMap after removal
//     let key_to_remove = String::from("Dave") ;
//     match map.remove(&key_to_remove)  {
//         Some(_) => println!("Updated map after removing key: {} is {:?}", key_to_remove, map) ,
//         None => println!("Key {} is not present in our map", key_to_remove) 
//     } ;

//     let mut map2 = HashMap::<String, i32>::new() ;
//     map2.insert(String::from("Flora"), 23) ;
//     map2.insert(String::from("Gary"), 25) ;
//     map2.insert(String::from("Han"), 29) ;
//     map2.insert(String::from("Irodov"), 24) ;

//     println!("Map2: {:?}", map2) ;

//     // Merging the two maps: map and map2

//     let mut merged_map = HashMap::new() ;
    
//     for (k, v) in map.iter(){
//         merged_map.insert(k, *v) ;
//     }

//     for(k, v) in map2.iter(){
//         merged_map.insert(k, *v) ;
//     }
//     println!("Merged map: {:?}", merged_map) ;
// }

//--------------------------------------------------------------------------------------------------------

// Q2. Program to count the frequency of characters in a string and store the result in a HashMap.
use std::io ;
fn main(){
    println!("Please enter your String: ") ;

    let mut str = String::new() ;

    io::stdin()
    .read_line(&mut str)
    .expect("Failed in reading input") ;

    let str = str.trim() ;

    // Creating an empty Map
    let mut map = HashMap::new();

    for ch in str.chars(){
        // This will check whether the given character is present in our map or not
        // If it is present, increment its frequency count
        // If it is not present, insert it into the HashMap with a frequency count of 0
        let count = map.entry(ch).or_insert(0) ;
        
        // If the character is not present, count will be incremented to 1.
        // If the character is present, count will be updated accordingly while iterating through the characters
        *count += 1 ;
    }

    println!("Frequencies of characters entered by user: ") ;
    for (k, v) in map.iter(){
        println!("Value: {} | Frequency: {}", k, v) ;
    }
}