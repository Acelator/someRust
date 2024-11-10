mod rsa;

use std::char;
use std::iter;

fn main() {
    let key = rsa::Key::generate_key(9,7);
    println!("ENCRYPT: {:?}", key.encrypt(vec![4, 32, 45, 50]));
    println!("DECRTPT: {:?}", key.decrypt(vec![16, 2, 54, 29]));



    let s = "HELLO WORLD";
    let s_char: Vec<char> = s.chars().collect();

    // TODO: FIX
    // let s_u32: Vec<u32> = s_char.into_iter().map(|x| char::to_digit(x, 10)).collect();


    // println!("ENCRYPT: {:?}", key.encrypt(s_char));
    println!("DECRTPT: {:?}", key.decrypt(vec![16, 2, 54, 29]));
    
    println!("");
    rsa::Key::generate_key(2, 5);
}
