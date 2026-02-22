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
