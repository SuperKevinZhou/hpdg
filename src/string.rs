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

pub struct StringGen;

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

#[derive(Clone, Debug)]
pub struct ParagraphConfig {
    pub word_count_range: LengthRange,
    pub word_length_range: LengthRange,
    pub first_letter_uppercase: bool,
    pub charset: String,
    pub word_separators: String,
    pub sentence_separators: String,
    pub sentence_terminators: String,
    pub sentence_joiners: String,
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

            let sep_or_term: f64 = rng.gen();
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
                    '*' => { min = 0; max = lim; i += 1; }
                    '+' => { min = 1; max = lim; i += 1; }
                    '?' => { min = 0; max = 1; i += 1; }
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

            let count = if min == max { min } else { rng.gen_range(min..=max) };
            for _ in 0..count {
                let idx = rng.gen_range(0..charset.len());
                out.push(charset[idx]);
            }
        }
        out
    }
}
