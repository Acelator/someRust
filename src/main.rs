mod rsa;

fn main() {
    rsa::test();
    rsa::generateKey(9,7);
    println!("ENCRYPT: {}", rsa::encrypt(63, 5, 4));
    println!("DECRTPT: {}", rsa::decrypt(63, 5, 16));
    println!("Hello, not the world!");
}
