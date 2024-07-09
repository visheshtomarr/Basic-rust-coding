// Write a program to print a block F using hash (#), where the F has a height of seven characters and width of six and five characters.
// ######
// #
// #
// #####
// #
// #
// #
#[allow(unused_variables)]
fn main() {

        for i in 1..=7{
            print!("#") ;

            for j in 1..=5{
                if i == 1{
                    print!("#") ;
                }
                else if i == 4{
                    for k in 1..=4{
                        print!("#") ;
                    }
                    break ;
                }
                else {
                    break ;
                }
            }
            println!("") ;
        }    
}