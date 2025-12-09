//! This is a module that supports some useful maths functions.

const DIGIT_FACT: [u64; 10] = [
    1,          // 0!
    1,          // 1!
    2,          // 2!
    6,          // 3!
    24,         // 4!
    120,        // 5!
    720,        // 6!
    5040,       // 7!
    40320,      // 8!
    362880,     // 9!
];

/// Check if two numbers are permutations of each other.
pub fn is_perm(a: u64, b: u64) -> bool {
    if a == 0 && b == 0 {
        return true;
    } else if a == 0 || b == 0 {
        return false;
    }
    
    if (a as f64).log10().floor() != (b as f64).log10().floor() {
        return false;
    }
    
    let mut count = [0i32; 10];
    
    let mut x = a;
    while x > 0 {
        count[(x % 10) as usize] += 1;
        x /= 10;
    }
    
    let mut y = b;
    while y > 0 {
        let digit = (y % 10) as usize;
        count[digit] -= 1;
        if count[digit] < 0 {
            return false;
        }
        y /= 10;
    }
    
    true
}

pub fn is_pal_string(s: String) -> bool {
    if s.is_ascii() {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len().saturating_sub(1);
        while left < right {
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    } else {
        let mut forward = s.char_indices();
        let mut backward = s.char_indices().rev();
        
        while let (Some((i, f_char)), Some((j, b_char))) = (forward.next(), backward.next()) {
            if i >= j {
                break;
            }
            if f_char != b_char {
                return false;
            }
        }
        true
    }
}

pub fn is_pal_u64(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    if n % 10 == 0 {
        return false;
    }

    let mut x = n;
    let mut reversed = 0;

    while x > reversed {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }

    x == reversed || x == reversed / 10
}

pub fn divisor_sum(n: u64) -> u64 {
    if n == 0 { return 0; }

    let mut i: u64 = 1;
    let mut sum: u64 = 0;

    while i * i <= n {
        if n % i == 0 { sum += i + n / i; }
        if i * i == n { sum -= i; }
        i += 1;
    }

    sum
}

pub fn d(n: u64) -> u64 {
    divisor_sum(n)
}

pub fn pal_list(k: usize) -> Vec<u64> {
    if k == 0 {
        return Vec::new();
    }
    if k == 1 {
        return (1..=9).collect();
    }

    let half_len = (k + 1) / 2;
    let start = 10u64.pow((half_len - 1) as u32);
    let end = 10u64.pow(half_len as u32);
    let mut res = Vec::new();

    for half in start..end {
        let mut x = half;
        let mut pal = half;
        if k % 2 == 1 {
            x /= 10;
        }
        while x > 0 {
            pal = pal * 10 + x % 10;
            x /= 10;
        }
        res.push(pal);
    }

    res
}

pub fn sof_digits(n: u64) -> u64 {
    if n == 0 {
        return DIGIT_FACT[0];
    }
    let mut x = n;
    let mut sum = 0u64;
    while x > 0 {
        let digit = (x % 10) as usize;
        sum += DIGIT_FACT[digit];
        x /= 10;
    }
    sum
}

pub fn sos_digits(n: u64) -> u64 {
    let mut x = n;
    let mut sum = 0u64;
    while x > 0 {
        let digit = x % 10;
        sum += digit * digit;
        x /= 10;
    }
    sum
}

pub fn pow_digits(n: u64, e: u32) -> u64 {
    let mut x = n;
    let mut sum = 0u64;
    while x > 0 {
        let digit = x % 10;
        sum += digit.pow(e);
        x /= 10;
    }
    sum
}

fn fib_pair(n: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let (a, b) = fib_pair(n / 2);
    let c = (a as u128 * ((2 * b - a) as u128)) as u64;
    let d = ((a as u128 * a as u128) + (b as u128 * b as u128)) as u64;
    if n % 2 == 0 {
        (c, d)
    } else {
        (d, c + d)
    }
}

pub fn fibonacci(n: u64) -> u64 {
    fib_pair(n).0
}

pub fn fibonacci_list(n: usize) -> Vec<u64> {
    let mut res = Vec::with_capacity(n);
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 0..n {
        res.push(a);
        let next = a + b;
        a = b;
        b = next;
    }
    res
}

pub fn fibonacci_range(start: u64, end: u64) -> Vec<u64> {
    if start > end {
        return Vec::new();
    }
    let mut res = Vec::new();
    let mut a = 0u64;
    let mut b = 1u64;
    for i in 0..=end {
        if i >= start {
            res.push(a);
        }
        let next = a + b;
        a = b;
        b = next;
    }
    res
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3u64;
    while i <= n / i {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn mod_pow(mut a: u64, mut d: u64, m: u64) -> u64 {
    let mut res = 1u64;
    while d > 0 {
        if d & 1 == 1 {
            res = mod_mul(res, a, m);
        }
        a = mod_mul(a, a, m);
        d >>= 1;
    }
    res
}

fn miller_rabin_pass(a: u64, s: u64, d: u64, n: u64) -> bool {
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..s {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

pub fn miller_rabin(n: u64, repeat_time: u32) -> bool {
    if n < 4 {
        return n == 2 || n == 3;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0u64;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
    let rounds = if repeat_time == 0 {
        bases.len() as u32
    } else {
        repeat_time.min(bases.len() as u32)
    };

    for &a in bases.iter().take(rounds as usize) {
        let a = a % n;
        if a == 0 {
            continue;
        }
        if !miller_rabin_pass(a, s, d, n) {
            return false;
        }
    }
    true
}

pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut res = Vec::new();
    if n < 2 {
        return res;
    }
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0u32;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            res.push((p, e));
        }
        if p == 2 {
            p = 3;
        } else {
            p += 2;
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

pub fn perm(mut n: u64, s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let mut fact = vec![1u64; len + 1];
    for i in 1..=len {
        fact[i] = fact[i - 1].saturating_mul(i as u64);
    }
    if fact[len] > 0 {
        n %= fact[len];
    }

    let mut res = String::with_capacity(len);
    for i in (1..=len).rev() {
        let f = fact[i - 1];
        let idx = (n / f) as usize;
        n %= f;
        let ch = chars.remove(idx);
        res.push(ch);
    }
    res
}

pub fn binomial(n: u64, k: u64) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut res: u128 = 1;
    for i in 0..k {
        res = res * (n - i) as u128 / (i + 1) as u128;
    }
    res
}

pub fn catalan_number(n: u64) -> u128 {
    let mut num: u128 = 1;
    let mut den: u128 = 1;
    for k in 2..=n {
        num *= (n + k) as u128;
        den *= k as u128;
    }
    num / den
}

pub fn prime_sieve(n: u64) -> Vec<u64> {
    if n <= 2 {
        return Vec::new();
    }

    let size = (n / 2) as usize;
    let mut sieve = vec![true; size];
    let mut i = 3u64;
    while i * i < n {
        if sieve[(i / 2) as usize] {
            let mut j = i * i;
            let step = i * 2;
            while j < n {
                sieve[(j / 2) as usize] = false;
                j += step;
            }
        }
        i += 2;
    }

    let mut res = Vec::new();
    res.push(2);
    let mut i = 3u64;
    while i < n {
        if sieve[(i / 2) as usize] {
            res.push(i);
        }
        i += 2;
    }
    res
}

pub fn exgcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut u, mut v, mut s, mut t) = (1i64, 0i64, 0i64, 1i64);
    while b != 0 {
        let q = a / b;
        let r = a % b;
        a = b;
        b = r;
        let next_u = u - q * s;
        let next_v = v - q * t;
        u = s;
        v = t;
        s = next_u;
        t = next_v;
    }
    (u, v, a)
}

pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    if m == 0 {
        return None;
    }
    let (x, _, g) = exgcd(a, m);
    if g != 1 && g != -1 {
        return None;
    }
    let mut res = x % m;
    if res < 0 {
        res += m.abs();
    }
    Some(res)
}

pub fn is_pandigital(n: &str, s: usize) -> bool {
    if s == 0 {
        return n.is_empty();
    }
    
    if n.len() != s {
        return false;
    }
    
    if s > 10 {
        return false;
    }
    
    let bytes = n.as_bytes();
    let mut seen: u16 = 0;
    
    for &byte in bytes {
        if !byte.is_ascii_digit() {
            return false;
        }
        
        let digit = (byte - b'0') as u16;
        let bit = 1 << digit;
        
        if seen & bit != 0 {
            return false;
        }
        
        seen |= bit;
    }
    
    if s == 10 {
        seen == 0x3FF
    } else {
        seen == ((1 << s) - 1) << 1
    }
}

pub fn is_pandigital_num<T: std::fmt::Display>(n: T, s: usize) -> bool {
    is_pandigital(&n.to_string(), s)
}

#[inline]
pub fn is_pandigital_u64(n: u64, s: usize) -> bool {
    if s == 0 || s > 10 {
        return false;
    }
    
    const POW10: [u64; 11] = [
        1,          // 10^0
        10,         // 10^1
        100,        // 10^2
        1000,       // 10^3
        10000,      // 10^4
        100000,     // 10^5
        1000000,    // 10^6
        10000000,   // 10^7
        100000000,  // 10^8
        1000000000, // 10^9
        10000000000 // 10^10
    ];
    
    if n < POW10[s - 1] || n >= POW10[s] {
        return false;
    }
    
    let mut num = n;
    let mut seen = 0u16;
    
    for _ in 0..s {
        let digit = (num % 10) as u8;
        num /= 10;
        
        if digit > 9 {
            return false;
        }

        if s < 10 && digit == 0 {
            return false;
        }
        
        let bit = 1 << digit;
        if (seen & bit) != 0 {
            return false;
        }
        seen |= bit;
    }
    
    if s == 10 {
        seen == 0x3FF
    } else {
        seen == (1 << (s + 1)) - 2
    }
}

#[inline]
pub fn is_pandigital_u64_default(n: u64) -> bool {
    if n < 123456789 || n > 987654321 {
        return false;
    }
    
    let mut num = n;
    let mut seen = 0u16;
    
    for _ in 0..9 {
        let digit = (num % 10) as u8;
        num /= 10;
        
        if digit == 0 {
            return false;
        }
        
        let bit = 1 << digit;
        if (seen & bit) != 0 {
            return false;
        }
        seen |= bit;
    }
    
    seen == 0b1111111110
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perm() {
        assert_eq!(is_perm(1, 2), false);
        assert_eq!(is_perm(23, 32), true);
        assert_eq!(is_perm(12345, 54123), true);
        assert_eq!(is_perm(1222, 2221), true);
        assert_eq!(is_perm(222, 222), true);
        assert_eq!(is_perm(222, 2222), false);
    }

    #[test]
    fn test_is_pal_u64() {
        assert_eq!(is_pal_u64(0), true);
        assert_eq!(is_pal_u64(121), true);
        assert_eq!(is_pal_u64(1231), false);
        assert_eq!(is_pal_u64(777), true);
        assert_eq!(is_pal_u64(1), true);
        assert_eq!(is_pal_u64(100001), true);
        assert_eq!(is_pal_u64(1221), true);
        assert_eq!(is_pal_u64(134), false);
    }

    #[test]
    fn test_is_pal_string_empty() {
        assert!(is_pal_string("".to_string()));
    }

    #[test]
    fn test_is_pal_string_ascii() {
        assert!(is_pal_string("a".to_string()));
        assert!(is_pal_string("aa".to_string()));
        assert!(is_pal_string("aba".to_string()));
        assert!(is_pal_string("abba".to_string()));
        assert!(!is_pal_string("abc".to_string()));
    }

    #[test]
    fn test_is_pal_string_unicode() {
        assert!(is_pal_string("中".to_string()));
        assert!(is_pal_string("中中".to_string()));
        assert!(is_pal_string("a中a".to_string()));
        assert!(is_pal_string("ab中ba".to_string()));
        assert!(!is_pal_string("a中b".to_string()));
    }

    #[test]
    fn test_s9_valid() {
        assert!(is_pandigital("123456789", 9));
    }

    #[test]
    fn test_s9_invalid() {
        assert!(!is_pandigital("123456788", 9));
        assert!(!is_pandigital("12345678", 9));
        assert!(!is_pandigital("012345678", 9));
    }

    #[test]
    fn test_s10_valid() {
        assert!(is_pandigital("0123456789", 10));
        assert!(is_pandigital("1023456789", 10));
    }

    #[test]
    fn test_s10_invalid() {
        assert!(!is_pandigital("1123456789", 10));
        assert!(!is_pandigital("123456789", 10));
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_pandigital("", 0));
        assert!(!is_pandigital("1", 0));
        assert!(!is_pandigital("123", 11));
    }

    #[test]
    fn test_u64_s9_valid() {
        assert!(is_pandigital_u64(123456789, 9));
        assert!(is_pandigital_u64(918273645, 9));
        assert!(is_pandigital_u64_default(123456789));
    }

    #[test]
    fn test_u64_s9_invalid() {
        assert!(!is_pandigital_u64(123456788, 9));
        assert!(!is_pandigital_u64(12345678, 9));
        assert!(!is_pandigital_u64(102345678, 9));
        assert!(!is_pandigital_u64_default(0));
    }

    #[test]
    fn test_u64_s10_valid() {
        assert!(is_pandigital_u64(1023456789, 10));
        assert!(is_pandigital_u64(9081726354, 10));
    }

    #[test]
    fn test_u64_s10_invalid() {
        assert!(!is_pandigital_u64(1123456789, 10));
        assert!(!is_pandigital_u64(123456789, 10));
        assert!(!is_pandigital_u64(12345678900, 10));
    }

    #[test]
    fn test_u64_edge_cases() {
        assert!(!is_pandigital_u64(0, 0));
        assert!(!is_pandigital_u64(0, 1));
        assert!(!is_pandigital_u64(1, 11));
        assert!(is_pandigital_u64(1, 1));
        assert!(!is_pandigital_u64(10, 2));
        assert!(!is_pandigital_u64(10, 2));
    }

    #[test]
    fn test_divisor_sum_zero() {
        assert_eq!(divisor_sum(0), 0);
    }

    #[test]
    fn test_divisor_sum_one() {
        assert_eq!(divisor_sum(1), 1);
    }

    #[test]
    fn test_divisor_sum_prime() {
        assert_eq!(divisor_sum(5), 6);
    }

    #[test]
    fn test_divisor_sum_perfect_square() {
        assert_eq!(divisor_sum(16), 31);
    }

    #[test]
    fn test_divisor_sum_non_square() {
        assert_eq!(divisor_sum(12), 28);
    }

    #[test]
    fn test_divisor_sum_large_number() {
        assert_eq!(divisor_sum(100), 217);
    }
}

pub fn is_palindromic<T: ToString>(v: T) -> bool {
    let s = v.to_string();
    if s.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(n) = s.parse::<u64>() {
            return is_pal_u64(n);
        }
    }
    is_pal_string(s)
}
