use rand::Rng;

pub const ALPHABET_SMALL: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHABET_CAPITAL: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const NUMBERS: &str = "0123456789";
pub const SENTENCE_SEPARATORS: &str = ",,,,,,,;;:";
pub const SENTENCE_TERMINATORS: &str = "....!";

#[derive(Clone, Copy, Debug)]
pub enum LengthRange {
    Exact(usize),
    Range(usize, usize),
}

impl From<usize> for LengthRange {
    fn from(value: usize) -> Self {
        LengthRange::Exact(value)
    }
}

impl From<(usize, usize)> for LengthRange {
    fn from(value: (usize, usize)) -> Self {
        LengthRange::Range(value.0, value.1)
    }
}

fn pick_len(range: LengthRange, rng: &mut impl Rng) -> usize {
    match range {
        LengthRange::Exact(len) => len,
        LengthRange::Range(start, end) => rng.gen_range(start..=end),
    }
}

pub struct StringGen;

impl StringGen {
    pub fn random(length_range: impl Into<LengthRange>, charset: &str) -> String {
        let mut rng = rand::rng();
        let len = pick_len(length_range.into(), &mut rng);
        let chars: Vec<char> = charset.chars().collect();
        if chars.is_empty() {
            return String::new();
        }
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars[idx]
            })
            .collect()
    }
}
