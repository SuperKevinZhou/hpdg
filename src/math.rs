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