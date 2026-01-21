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
}
