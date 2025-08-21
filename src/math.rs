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
    let mut i: u64 = 0;
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
}