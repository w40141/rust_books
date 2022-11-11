pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    if n == 0 || m == 0 {
        return 0;
    } else {
        while m != 0 {
            if n > m {
                let t = m;
                m = n;
                n = t;
            }
            m %= n;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(2310, 429), 33);
        assert_eq!(gcd(10, 0), 0);
    }
}
