pub fn test()  {
  println!("HELLOOO!");
}

pub fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

// https://en.wikipedia.org/wiki/RSA_(cryptosystem)
pub fn generateKey(p: u128, q: u128) {
    let n: u128 = p * q;

    // Carmichael totient function
    let lambda: u128 = lcm(p-1, q-1);


    let mut e: u128 = 1;
    for i in 2..lambda {
        if(gcd(i, lambda) == 1) {
            e = i;
            break;
        }
    }

    let mut d: u128 = 1;
    for i in 1..n*lambda {
        if ((i*e).rem_euclid(lambda) == 1) {
            println!("meffffffffffffff");
            d = i;
            break;
        }
    }

    println!("Our key from primes {} and {}", p,q);
    println!("With n: {}, l: {}, e:{}, d:{}", n, lambda, e, d);
}


// m is the char to encrypt
pub fn encrypt(n: u128, e: u32, m: u128) -> u128 {
    let p: u128 = u128::pow(m,e);
    (p).rem_euclid(n)
}

pub fn decrypt(n:u128, d: u32, c:u128) -> u128 {
    (u128::pow(c, d)).rem_euclid(n)
}
