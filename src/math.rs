//! This is a module that supports some useful maths functions.

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