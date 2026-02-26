pub fn ati<I, T>(iter: I) -> Vec<i64>
where
    I: IntoIterator<Item = T>,
    T: Into<i64>,
{
    iter.into_iter().map(|v| v.into()).collect()
}

pub trait ListLike {}

impl<T> ListLike for Vec<T> {}
impl<T> ListLike for [T] {}
impl<T, const N: usize> ListLike for [T; N] {}

/// Check whether a value is list-like (Vec/array/slice).
pub fn list_like<T: ?Sized + ListLike>(_data: &T) -> bool {
    true
}

pub trait IntLike {}

impl IntLike for i8 {}
impl IntLike for i16 {}
impl IntLike for i32 {}
impl IntLike for i64 {}
impl IntLike for isize {}
impl IntLike for u8 {}
impl IntLike for u16 {}
impl IntLike for u32 {}
impl IntLike for u64 {}
impl IntLike for usize {}

/// Check whether a value is integer-like.
pub fn int_like<T: IntLike>(_data: &T) -> bool {
    true
}

/// Split text into trimmed lines and drop trailing blanks.
pub fn strtolines(input: &str) -> Vec<String> {
    let mut lines: Vec<String> = input.lines().map(|l| l.trim_end().to_string()).collect();
    while lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        lines.pop();
    }
    lines
}

/// Convert data into a String.
pub fn make_unicode<T: ToString>(data: T) -> String {
    data.to_string()
}

use std::collections::HashMap;
use std::env;

/// Argument specification for unpack_kwargs.
pub enum ArgSpec<T> {
    Required(&'static str),
    Optional(&'static str, T),
}

/// Parse keyword-style arguments from a map.
pub fn unpack_kwargs<T: Clone>(
    funcname: &str,
    kwargs: &HashMap<String, T>,
    arg_pattern: &[ArgSpec<T>],
) -> Result<HashMap<String, T>, String> {
    let mut remaining = kwargs.clone();
    let mut result: HashMap<String, T> = HashMap::new();

    for spec in arg_pattern {
        match spec {
            ArgSpec::Required(key) => {
                if let Some(val) = remaining.remove(*key) {
                    result.insert((*key).to_string(), val);
                } else {
                    return Err(format!(
                        "{funcname}() missing 1 required keyword-only argument: '{key}'"
                    ));
                }
            }
            ArgSpec::Optional(key, default) => {
                let val = remaining.remove(*key).unwrap_or_else(|| default.clone());
                result.insert((*key).to_string(), val);
            }
        }
    }

    if let Some((k, _)) = remaining.iter().next() {
        return Err(format!(
            "{funcname}() got an unexpected keyword argument '{k}'"
        ));
    }

    Ok(result)
}

/// Parse CLI arguments and return optional seed.
pub fn process_args() -> Option<u64> {
    for arg in env::args() {
        if let Some(seed) = arg.strip_prefix("--randseed=") {
            if let Ok(v) = seed.parse::<u64>() {
                return Some(v);
            }
        }
    }
    None
}

/// Escape a path for shell usage.
pub fn escape_path(path: &str) -> String {
    if cfg!(windows) {
        format!("\"{}\"", path.replace('\\', "/"))
    } else {
        format!("'{}'", path.replace('\'', "\\'"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ati() {
        assert_eq!(ati([1i64, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_list_like_int_like() {
        let v = vec![1, 2];
        assert!(list_like(&v));
        let x = 5i32;
        assert!(int_like(&x));
    }

    #[test]
    fn test_strtolines_make_unicode() {
        let lines = strtolines("a  \n\n");
        assert_eq!(lines, vec!["a".to_string()]);
        assert_eq!(make_unicode(123), "123");
    }

    #[test]
    fn test_unpack_kwargs() {
        let mut kwargs: HashMap<String, i32> = HashMap::new();
        kwargs.insert("a".to_string(), 1);
        let specs = [ArgSpec::Required("a"), ArgSpec::Optional("b", 2)];
        let res = unpack_kwargs("f", &kwargs, &specs).unwrap();
        assert_eq!(*res.get("a").unwrap(), 1);
        assert_eq!(*res.get("b").unwrap(), 2);
    }

    #[test]
    fn test_escape_path() {
        let out = escape_path("a b");
        assert!(out.contains('a'));
        if cfg!(windows) {
            assert!(out.starts_with('"'));
        }
    }
}
