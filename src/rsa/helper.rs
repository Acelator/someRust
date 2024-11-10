pub fn hamming_weight(mut n: u128) -> u128 {
    let mut count: u128 = 0;
    while n > 9 {
        let r = n % 10;
        if r != 0 {
            count += 1;
        };
        n = n / 10;
    }
    if n != 0 {
        count += 1;
    }
    count
}
