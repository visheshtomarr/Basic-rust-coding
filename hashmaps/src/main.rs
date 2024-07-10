use std::collections::HashMap ;
// Q1. Creating a hashmap by inserting names and ages of different accounts and performing various hashmap operations on it.

fn main() {
    let mut map = HashMap::new() ;

    map.insert(String::from("Alice"), 23) ;
    map.insert(String::from("Bob"), 25) ;
    map.insert(String::from("Charlie"), 28) ;
    map.insert(String::from("Dave"), 20) ;

    // Printing map using debug print
    println!("Map: {:?}", map) ;

    // Iterating over a map and prinitng using the iterators
    for (k,v) in map.iter(){
        println!("Key: {} | Value: {}", k, v) ;
    }

    // Checking whether a key is present in Hashmap or not
    let key_to_check = "Dave".to_string() ;

    if map.contains_key(&key_to_check){
        println!("Key: {}, is present in our Hashmap", key_to_check) ;
    }
    else{
        println!("Key: {}, is present in our Hashmap", key_to_check) ;
    }
    
    // Retrieving the value associated with a specific key from HashMap and printing it
    let key_to_retrieve = String::from("Charlie") ;

    // This ".get()" method will result in an Option<T>
    let val = map.get(&key_to_retrieve) ;

    // Using match to check value associated with our given key
    match val {
        Some(&val) => println!("Value associated with key: {} is {}", key_to_retrieve, val),
        None => println!("No value present for key: {}", key_to_retrieve)
    }

    // Updating the value associated with a key in HashMap and print the updated HashMap.
    let key_to_update = String::from("Charlie") ;
    let new_value = 18 ;

    // Retrieving a mutable Option value associated with our key using .get_mut() method
    let updated_value = map.get_mut(&key_to_update) ;

    match updated_value {
        // If Option "updated_value" has Some(value), it will be updated with "new_value" 
        Some(value) => *value = new_value,
        None => println!("Key {} is not present in Hashmap", key_to_update) 
    }
    println!("Updated map: {:?}", map) ;

    // Using .entry() method to insert into our Map.
    // .entry() will check whether the passed key is present in our map or not.
    // If key is present, associated value will not change. 
    // If key is not present, .or_insert() method will insert value given into it. 
    map.entry(String::from("Eve")).or_insert(30) ;

    println!("Updated map after entering key-pair: {:?}", map) ;

    // Removing a key-value pair from HashMap and printing the HashMap after removal
    let key_to_remove = String::from("Dave") ;
    match map.remove(&key_to_remove)  {
        Some(_) => println!("Updated map after removing key: {} is {:?}", key_to_remove, map) ,
        None => println!("Key {} is not present in our map", key_to_remove) 
    } ;
}