mod helper;

pub fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub struct Key {
    n: u32,
    e: u32,
    d: u32
}

impl Key {
    // https://en.wikipedia.org/wiki/RSA_(cryptosystem)
    pub fn generate_key(p: u32, q: u32) -> Key {
        let n: u32 = p * q;

        // Carmichael totient function
        let lambda: u32 = lcm(p-1, q-1);

        let q: u128 = 4;

        let mut e: u32 = 1;
        for i in 2..lambda {
            if gcd(i, lambda) == 1 {
                e = i;
                break;
            }
        }

        let mut d: u32 = 1;
        for i in 1..n*lambda {
            if (i*e).rem_euclid(lambda) == 1 {
                d = i;
                break;
            }
        }

        println!("Our key from primes {} and {}", p,q);
        println!("With n: {}, l: {}, e:{}, d:{}", n, lambda, e, d);

        println!("HAMMING: {}", helper::hamming_weight(11101000));

        Key{ n, e, d }
    }

    // m is the char to encrypt
    pub fn encrypt(&self, msg: Vec<u32>) -> Vec<u32> {

        let mut encrypted_msg: Vec<u32> = Vec::new();
        for m in msg {
            encrypted_msg.push((u32::pow(m, self.e)).rem_euclid(self.n));
        }

        encrypted_msg
    }

    pub fn decrypt(&self, msg: Vec<u32>) -> Vec<u32> {
        let mut decrypted_msg: Vec<u32> = Vec::new();

        for m in msg {
            decrypted_msg.push((u32::pow(m, self.d)).rem_euclid(self.n));
        }
        decrypted_msg
    }
} 
