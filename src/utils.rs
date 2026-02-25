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

pub fn int_like<T: IntLike>(_data: &T) -> bool {
    true
}

pub fn strtolines(input: &str) -> Vec<String> {
    let mut lines: Vec<String> = input.lines().map(|l| l.trim_end().to_string()).collect();
    while lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        lines.pop();
    }
    lines
}

pub fn make_unicode<T: ToString>(data: T) -> String {
    data.to_string()
}

use std::collections::HashMap;
use std::env;

pub enum ArgSpec<T> {
    Required(&'static str),
    Optional(&'static str, T),
}

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

pub fn escape_path(path: &str) -> String {
    if cfg!(windows) {
        format!("\"{}\"", path.replace('\\', "/"))
    } else {
        format!("'{}'", path.replace('\'', "\\'"))
    }
}
