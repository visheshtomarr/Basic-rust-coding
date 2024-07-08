// Creating a program that prints the array excluding its middle value.

fn main() {
    
    let arr1 = [1, 2, 3, 4, 5, 6] ;
    let arr2 = [1, 2, 3, 4, 5] ; 

    let n1 = arr1.len() ;
    let n2 = arr2.len() ;

    println!("Length of first array: {}", n1) ;
    println!("Length of second array: {}", n2) ;

    remove_middle_value(n1, &arr1) ;
    remove_middle_value(n2, &arr2) ;
}

fn remove_middle_value(n: usize, arr: &[u32]) {
    if n % 2 != 0 {
        let as1 = &arr[0 .. n/2] ;
        let as2 = &arr[(n/2 + 1) .. n] ;

        let res = [as1,as2].concat() ;
        println!("{:?}{:?}", as1,as2) ;
        println!("{:?}", res) ;
    }
    else {
        let as1 = &arr[0 .. (n/2 - 1)] ;
        let as2 = &arr[(n/2 + 1) .. n] ;

        let res = [as1,as2].concat() ;
        println!("{:?}{:?}", as1,as2) ;
        println!("{:?}", res) ;
    }
}
