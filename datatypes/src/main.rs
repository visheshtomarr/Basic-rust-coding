fn main() {

    let a: i32 = -20 ;
    let b: u32 = 2 ;
    let boolean: bool = true ;
    let c: f32 = 4.56 ;
    let x  = 1_25_000 ; 

    let tup : (i32, f32, bool) = (3, 3.6, false) ;
    println!("a: {}, b: {}, boolean : {}, c: {}, x: {}", a,b,boolean,c,x) ;
    println!("tup: {:?}", tup);
}
