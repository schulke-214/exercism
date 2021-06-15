pub fn check_prime_from(n: u32, i: u32) -> bool {
    if i * i > n {
        return true;
    }

    if n % i == 0 || n % (i + 2) == 0 {
        return false;
    }

    check_prime_from(n, i + 6)
}

pub fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    check_prime_from(n, 5)
}

pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .take((n + 1) as usize)
        .last()
        .unwrap()
}
