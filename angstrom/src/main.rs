// created by nalin gabriel for GCI 2019
// this program reads input from the user and checks whether 
//    it is an Angstrom number or not. It also has test cases.
// how to run:
// cargo test 
// contact: gnalin134@gmail.com

//checks whether cubesum is an angstrom number.
fn angstrom_check(input: u64) -> bool
{
    let mut  placeval = 1;
    let mut digits= Vec::new();
   
    loop
    {

       let  inputcopy = input/placeval;
        let digit = inputcopy%10;
      //  println!("{}",digit);
        placeval = placeval * 10;
        digits.push(digit);
        if inputcopy == 0 
        {
            break;
        }
       

    }
    let cubesum = generate_cubesum(&digits);
    let mut is_angstrom  = false;
    if cubesum == input
    {
        println!("It is an Angstrom number");
        is_angstrom = true;
    }
    else
    {
        println!("not an Angstrom number");
    }
    return is_angstrom;
}
//generates cubesum for checking whether its an angstrom number or not later on in the first function.
fn generate_cubesum(digits: &[u64]) -> u64
{
    let mut cubesum: u64 = 0;
    for digit in digits
    {
        let cube = digit*digit*digit;
        
        cubesum = cubesum + cube; 
    }
    return cubesum;
       
}

fn main() {
    use std::io::{stdin,stdout,Write};

    let mut input_text = String::new();
    print!("Please enter some number: ");
    let _=stdout().flush();
    stdin()

        .read_line(&mut input_text)

        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut n = 0;
    match trimmed.parse::<u64>() {
        Ok(i) => {n=i; println!("your integer input: {}", i)},

        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    println!("copied input {} and more {}",n, n);
    angstrom_check(n);
}
//Test cases.
#[cfg(test)]
mod tests{
    use super :: *;
    #[test]
    fn test_angstrom() {
        assert_eq!(angstrom_check(0),true); 
        assert_eq!(angstrom_check(876   ),false); 
        assert_eq!(angstrom_check(153 ),true); 
        assert_eq!(angstrom_check(407  ),true);
        assert_eq!(angstrom_check(370  ),true);  
        assert_eq!(angstrom_check(371   ),true);
        assert_eq!(angstrom_check( 876 ),false);  
    
    }
}

