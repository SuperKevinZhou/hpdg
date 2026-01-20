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

#[derive(Clone, Debug)]
pub struct SentenceConfig {
    pub word_length_range: LengthRange,
    pub first_letter_uppercase: bool,
    pub charset: String,
    pub word_separators: String,
    pub sentence_terminators: String,
}

impl Default for SentenceConfig {
    fn default() -> Self {
        Self {
            word_length_range: LengthRange::Range(3, 8),
            first_letter_uppercase: true,
            charset: ALPHABET_SMALL.to_string(),
            word_separators: " ".to_string(),
            sentence_terminators: SENTENCE_TERMINATORS.to_string(),
        }
    }
}

fn random_char(chars: &str, rng: &mut impl Rng) -> Option<char> {
    let pool: Vec<char> = chars.chars().collect();
    if pool.is_empty() {
        return None;
    }
    Some(pool[rng.gen_range(0..pool.len())])
}

fn capitalize_first(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

impl StringGen {
    pub fn random_word(length_range: impl Into<LengthRange>, charset: Option<&str>) -> String {
        let charset = charset.unwrap_or(ALPHABET_SMALL);
        Self::random(length_range, charset)
    }

    pub fn random_sentence(
        word_count_range: impl Into<LengthRange>,
        config: Option<&SentenceConfig>,
    ) -> String {
        let cfg = config.cloned().unwrap_or_default();
        let mut rng = rand::rng();
        let word_count = pick_len(word_count_range.into(), &mut rng);
        let mut words: Vec<String> = Vec::with_capacity(word_count);
        for _ in 0..word_count {
            words.push(Self::random(cfg.word_length_range, &cfg.charset));
        }
        if cfg.first_letter_uppercase && !words.is_empty() {
            words[0] = capitalize_first(&words[0]);
        }
        let mut sentence = String::new();
        for (idx, word) in words.iter().enumerate() {
            if idx > 0 {
                if let Some(sep) = random_char(&cfg.word_separators, &mut rng) {
                    sentence.push(sep);
                }
            }
            sentence.push_str(word);
        }
        if let Some(term) = random_char(&cfg.sentence_terminators, &mut rng) {
            sentence.push(term);
        }
        sentence
    }
}
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars[idx]
            })
            .collect()
    }
}
