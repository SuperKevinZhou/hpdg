//! Random string and text generators.
//!
//! The API in this module focuses on practical testcase generation instead of full natural
//! language modeling. It supports fixed or ranged lengths, simplified regex-like patterns,
//! dictionary sampling, and quick sentence/paragraph construction.
//!
//! # Example
//!
//! ```rust
//! use hpdg::string::{SentenceConfig, StringGen};
//!
//! let mut cfg = SentenceConfig::default();
//! cfg.sentence_terminators = ".".to_string();
//! let sentence = StringGen::random_sentence(3, Some(&cfg));
//! assert!(sentence.ends_with('.'));
//! ```

use rand::Rng;

/// Lowercase Latin alphabet.
pub const ALPHABET_SMALL: &str = "abcdefghijklmnopqrstuvwxyz";
/// Uppercase Latin alphabet.
pub const ALPHABET_CAPITAL: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// Combined lowercase and uppercase Latin alphabet.
pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// Decimal digits.
pub const NUMBERS: &str = "0123456789";
/// Default separators used when generating paragraphs.
pub const SENTENCE_SEPARATORS: &str = ",,,,,,,;;:";
/// Default sentence terminators.
pub const SENTENCE_TERMINATORS: &str = "....!";

/// Length specification accepted by many string-generation helpers.
#[derive(Clone, Copy, Debug)]
pub enum LengthRange {
    Exact(usize),
    Range(usize, usize),
}

/// High-level character mode used by [`StringGen::random_with_mode`].
#[derive(Clone, Copy, Debug)]
pub enum CharsetMode {
    Ascii,
    Unicode,
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
        LengthRange::Range(start, end) => rng.random_range(start..=end),
    }
}

fn random_char(chars: &str, rng: &mut impl Rng) -> Option<char> {
    let pool: Vec<char> = chars.chars().collect();
    if pool.is_empty() {
        return None;
    }
    Some(pool[rng.random_range(0..pool.len())])
}

fn capitalize_first(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn random_unicode_string(length_range: LengthRange, rng: &mut impl Rng) -> String {
    let len = pick_len(length_range, rng);
    let mut out = String::new();
    for _ in 0..len {
        loop {
            let code = rng.random_range(0x4E00u32..=0x9FFFu32);
            if let Some(ch) = char::from_u32(code) {
                out.push(ch);
                break;
            }
        }
    }
    out
}

/// Random string helpers.
pub struct StringGen;

/// Configuration for sentence generation.
#[derive(Clone, Debug)]
pub struct SentenceConfig {
    /// Length of each generated word.
    pub word_length_range: LengthRange,
    /// Whether the first generated word should be capitalized.
    pub first_letter_uppercase: bool,
    /// Character set used to build words.
    pub charset: String,
    /// Candidate separators inserted between words.
    pub word_separators: String,
    /// Candidate sentence terminators appended at the end.
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

/// Configuration for paragraph generation.
#[derive(Clone, Debug)]
pub struct ParagraphConfig {
    /// Number of words per generated sentence.
    pub word_count_range: LengthRange,
    /// Length of each generated word.
    pub word_length_range: LengthRange,
    /// Whether the first word of a sentence should be capitalized.
    pub first_letter_uppercase: bool,
    /// Character set used to build words.
    pub charset: String,
    /// Candidate separators inserted between words.
    pub word_separators: String,
    /// Candidate separators inserted between non-terminal sentences.
    pub sentence_separators: String,
    /// Candidate sentence terminators.
    pub sentence_terminators: String,
    /// Characters inserted between generated sentence fragments.
    pub sentence_joiners: String,
    /// Probability that a sentence ends instead of continuing with a separator.
    pub termination_percentage: f64,
}

impl Default for ParagraphConfig {
    fn default() -> Self {
        Self {
            word_count_range: LengthRange::Range(6, 10),
            word_length_range: LengthRange::Range(3, 8),
            first_letter_uppercase: true,
            charset: ALPHABET_SMALL.to_string(),
            word_separators: " ".to_string(),
            sentence_separators: SENTENCE_SEPARATORS.to_string(),
            sentence_terminators: SENTENCE_TERMINATORS.to_string(),
            sentence_joiners: " ".to_string(),
            termination_percentage: 0.3,
        }
    }
}

impl StringGen {
    /// Generate a random string from a charset.
    ///
    /// The charset is treated as a pool of Unicode scalar values; each generated position
    /// independently samples one character from that pool.
    pub fn random(length_range: impl Into<LengthRange>, charset: &str) -> String {
        let mut rng = rand::rng();
        let len = pick_len(length_range.into(), &mut rng);
        let chars: Vec<char> = charset.chars().collect();
        if chars.is_empty() {
            return String::new();
        }
        (0..len)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
                chars[idx]
            })
            .collect()
    }

    /// Generate a random word with an optional charset.
    ///
    /// When `charset` is `None`, [`ALPHABET_SMALL`] is used.
    pub fn random_word(length_range: impl Into<LengthRange>, charset: Option<&str>) -> String {
        let charset = charset.unwrap_or(ALPHABET_SMALL);
        Self::random(length_range, charset)
    }

    /// Generate a random sentence.
    ///
    /// Words are sampled independently according to [`SentenceConfig`], then joined using
    /// random separators and finished with a random terminator when available.
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

    /// Generate a random paragraph.
    ///
    /// Internally this repeatedly builds sentence fragments and decides whether each fragment
    /// should terminate or continue based on `termination_percentage`.
    pub fn random_paragraph(
        sentence_count_range: impl Into<LengthRange>,
        config: Option<&ParagraphConfig>,
    ) -> String {
        let cfg = config.cloned().unwrap_or_default();
        assert!(
            (0.0..=1.0).contains(&cfg.termination_percentage),
            "Invalid termination_percentage"
        );

        let mut rng = rand::rng();
        let sentence_count = pick_len(sentence_count_range.into(), &mut rng);
        let sentence_cfg = SentenceConfig {
            word_length_range: cfg.word_length_range,
            first_letter_uppercase: false,
            charset: cfg.charset.clone(),
            word_separators: cfg.word_separators.clone(),
            sentence_terminators: String::new(),
        };

        let mut sentences: Vec<String> = Vec::with_capacity(sentence_count);
        let mut capitalize_next = true;
        for i in 0..sentence_count {
            let mut sentence = Self::random_sentence(cfg.word_count_range, Some(&sentence_cfg));
            if capitalize_next && cfg.first_letter_uppercase {
                sentence = capitalize_first(&sentence);
            }

            let sep_or_term: f64 = rng.random();
            if sep_or_term < cfg.termination_percentage || i + 1 == sentence_count {
                if let Some(term) = random_char(&cfg.sentence_terminators, &mut rng) {
                    sentence.push(term);
                }
                capitalize_next = true;
            } else {
                if let Some(sep) = random_char(&cfg.sentence_separators, &mut rng) {
                    sentence.push(sep);
                }
                capitalize_next = false;
            }
            sentences.push(sentence);
        }

        let mut paragraph = String::new();
        for (idx, sentence) in sentences.iter().enumerate() {
            if idx > 0 {
                if let Some(joiner) = random_char(&cfg.sentence_joiners, &mut rng) {
                    paragraph.push(joiner);
                }
            }
            paragraph.push_str(sentence);
        }
        paragraph
    }

    /// Generate a string that matches a simplified regex-like pattern.
    ///
    /// The supported syntax is intentionally lightweight and aimed at testcase generation,
    /// not full regular-expression compatibility.
    pub fn random_regex(pattern: &str, limit: usize) -> String {
        let mut rng = rand::rng();
        let lim = if limit <= 1 { 10 } else { limit };
        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0usize;
        let mut out = String::new();
        let any_charset: Vec<char> = (ALPHABET.to_string() + NUMBERS + "_").chars().collect();

        while i < chars.len() {
            let c = chars[i];
            if c == '^' || c == '$' {
                i += 1;
                continue;
            }
            let mut charset: Vec<char> = Vec::new();
            if c == '\\' {
                i += 1;
                if i >= chars.len() {
                    break;
                }
                let esc = chars[i];
                match esc {
                    'd' => charset.extend(NUMBERS.chars()),
                    'w' => charset.extend((ALPHABET.to_string() + NUMBERS + "_").chars()),
                    _ => charset.push(esc),
                }
                i += 1;
            } else if c == '[' {
                i += 1;
                while i < chars.len() && chars[i] != ']' {
                    let ch = chars[i];
                    if ch == '\\' {
                        i += 1;
                        if i >= chars.len() {
                            break;
                        }
                        let esc = chars[i];
                        match esc {
                            'd' => charset.extend(NUMBERS.chars()),
                            'w' => charset.extend((ALPHABET.to_string() + NUMBERS + "_").chars()),
                            _ => charset.push(esc),
                        }
                        i += 1;
                        continue;
                    }
                    if i + 2 < chars.len() && chars[i + 1] == '-' && chars[i + 2] != ']' {
                        let start = ch as u8;
                        let end = chars[i + 2] as u8;
                        if start <= end {
                            for code in start..=end {
                                charset.push(code as char);
                            }
                        } else {
                            for code in end..=start {
                                charset.push(code as char);
                            }
                        }
                        i += 3;
                        continue;
                    }
                    charset.push(ch);
                    i += 1;
                }
                if i < chars.len() && chars[i] == ']' {
                    i += 1;
                }
            } else if c == '.' {
                charset = any_charset.clone();
                i += 1;
            } else {
                charset.push(c);
                i += 1;
            }

            if charset.is_empty() {
                continue;
            }

            let mut min = 1usize;
            let mut max = 1usize;
            if i < chars.len() {
                match chars[i] {
                    '*' => {
                        min = 0;
                        max = lim;
                        i += 1;
                    }
                    '+' => {
                        min = 1;
                        max = lim;
                        i += 1;
                    }
                    '?' => {
                        min = 0;
                        max = 1;
                        i += 1;
                    }
                    '{' => {
                        let mut j = i + 1;
                        let mut num1 = String::new();
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            num1.push(chars[j]);
                            j += 1;
                        }
                        if !num1.is_empty() {
                            let parsed_min: usize = num1.parse().unwrap_or(0);
                            let mut parsed_max = parsed_min;
                            if j < chars.len() && chars[j] == ',' {
                                j += 1;
                                let mut num2 = String::new();
                                while j < chars.len() && chars[j].is_ascii_digit() {
                                    num2.push(chars[j]);
                                    j += 1;
                                }
                                if num2.is_empty() {
                                    parsed_max = lim;
                                } else {
                                    parsed_max = num2.parse().unwrap_or(parsed_min);
                                }
                            }
                            if j < chars.len() && chars[j] == '}' {
                                min = parsed_min;
                                max = parsed_max;
                                i = j + 1;
                            }
                        }
                    }
                    _ => {}
                }
            }

            let count = if min == max {
                min
            } else {
                rng.random_range(min..=max)
            };
            for _ in 0..count {
                let idx = rng.random_range(0..charset.len());
                out.push(charset[idx]);
            }
        }
        out
    }

    /// Choose a random entry from a dictionary.
    ///
    /// Returns an empty string when the dictionary is empty.
    pub fn random_from_dict<T: AsRef<str>>(dict: &[T]) -> String {
        if dict.is_empty() {
            return String::new();
        }
        let mut rng = rand::rng();
        let idx = rng.random_range(0..dict.len());
        dict[idx].as_ref().to_string()
    }

    /// Build a sentence from dictionary words.
    ///
    /// Words are sampled with replacement from `dict`.
    pub fn random_sentence_from_dict<T: AsRef<str>>(
        word_count_range: impl Into<LengthRange>,
        dict: &[T],
        first_letter_uppercase: bool,
        separator: &str,
    ) -> String {
        if dict.is_empty() {
            return String::new();
        }
        let mut rng = rand::rng();
        let count = pick_len(word_count_range.into(), &mut rng);
        let mut words: Vec<String> = Vec::with_capacity(count);
        for _ in 0..count {
            let idx = rng.random_range(0..dict.len());
            words.push(dict[idx].as_ref().to_string());
        }
        if first_letter_uppercase && !words.is_empty() {
            words[0] = capitalize_first(&words[0]);
        }
        words.join(separator)
    }

    /// Generate random strings with ASCII or Unicode modes.
    ///
    /// [`CharsetMode::Ascii`] uses a built-in alphanumeric alphabet; [`CharsetMode::Unicode`]
    /// samples CJK Unified Ideographs for visually obvious non-ASCII output.
    pub fn random_with_mode(length_range: impl Into<LengthRange>, mode: CharsetMode) -> String {
        let mut rng = rand::rng();
        let len_range = length_range.into();
        match mode {
            CharsetMode::Ascii => {
                let charset = format!("{}{}", ALPHABET, NUMBERS);
                Self::random(len_range, &charset)
            }
            CharsetMode::Unicode => random_unicode_string(len_range, &mut rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_basic_charset() {
        let s = StringGen::random(10, "abc");
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| "abc".contains(c)));
    }

    #[test]
    fn test_random_sentence_basic() {
        let mut cfg = SentenceConfig::default();
        cfg.word_length_range = LengthRange::Exact(3);
        cfg.word_separators = " ".to_string();
        cfg.sentence_terminators = ".".to_string();
        let sentence = StringGen::random_sentence(3, Some(&cfg));
        assert!(sentence.ends_with('.'));
        assert_eq!(sentence.matches(' ').count(), 2);
        assert!(sentence.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_random_paragraph_smoke() {
        let para = StringGen::random_paragraph(3, None);
        assert!(!para.is_empty());
    }

    #[test]
    fn test_random_regex_simple() {
        let pattern = "[0-9]+\\w_.{0,9}";
        let s = StringGen::random_regex(pattern, 5);
        let chars = s.chars().collect::<Vec<_>>();
        assert!(!chars.is_empty());
        let matches_pattern = (1..chars.len()).any(|digit_end| {
            chars[..digit_end].iter().all(|c| c.is_ascii_digit())
                && digit_end + 1 < chars.len()
                && (chars[digit_end].is_ascii_alphanumeric() || chars[digit_end] == '_')
                && chars[digit_end + 1] == '_'
                && chars.len().saturating_sub(digit_end + 2) <= 9
        });
        assert!(
            matches_pattern,
            "generated string did not match simplified regex: {s}"
        );
    }

    #[test]
    fn test_random_dict_helpers() {
        let dict = ["lorem", "ipsum", "dolor"];
        let word = StringGen::random_from_dict(&dict);
        assert!(dict.contains(&word.as_str()));
        let sentence = StringGen::random_sentence_from_dict(2, &dict, true, " ");
        assert_eq!(sentence.split(' ').count(), 2);
    }

    #[test]
    fn test_random_with_mode() {
        let ascii = StringGen::random_with_mode(5, CharsetMode::Ascii);
        assert!(ascii.is_ascii());
        let unicode = StringGen::random_with_mode(5, CharsetMode::Unicode);
        assert!(!unicode.is_ascii());
    }
}
