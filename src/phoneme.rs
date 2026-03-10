/// Phoneme representation for Japanese kana.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phoneme {
    /// Consonant + Vowel (e.g., "k" + "a" for カ)
    CV {
        consonant: &'static str,
        vowel: &'static str,
    },
    /// Pure vowel (e.g., "a" for ア)
    Vowel(&'static str),
    /// Long vowel — a vowel derived from ー resolution.
    /// Insertion/deletion cost is lower than regular phonemes.
    LongVowel(&'static str),
    /// Moraic nasal ン
    Nasal,
}

impl Phoneme {
    /// Returns true if this phoneme is a long-vowel-derived vowel.
    pub fn is_long_vowel(&self) -> bool {
        matches!(self, Phoneme::LongVowel(_))
    }

    /// Returns the vowel value regardless of variant.
    pub fn vowel_value(&self) -> Option<&'static str> {
        match self {
            Phoneme::CV { vowel, .. } => Some(vowel),
            Phoneme::Vowel(v) | Phoneme::LongVowel(v) => Some(v),
            Phoneme::Nasal => None,
        }
    }
}

/// Katakana-to-phoneme mapping entry.
struct KanaEntry {
    kana: &'static str,
    phonemes: &'static [Phoneme],
}

/// Static phoneme constants for reuse.
const fn cv(consonant: &'static str, vowel: &'static str) -> Phoneme {
    Phoneme::CV { consonant, vowel }
}

// Multi-character kana entries (must be checked before single-char).
// Order: longest match first.
static MULTI_CHAR_ENTRIES: &[KanaEntry] = &[
    // キャ行
    KanaEntry { kana: "キャ", phonemes: &[cv("ky", "a")] },
    KanaEntry { kana: "キュ", phonemes: &[cv("ky", "u")] },
    KanaEntry { kana: "キョ", phonemes: &[cv("ky", "o")] },
    // シャ行
    KanaEntry { kana: "シャ", phonemes: &[cv("sh", "a")] },
    KanaEntry { kana: "シュ", phonemes: &[cv("sh", "u")] },
    KanaEntry { kana: "ショ", phonemes: &[cv("sh", "o")] },
    // チャ行
    KanaEntry { kana: "チャ", phonemes: &[cv("ch", "a")] },
    KanaEntry { kana: "チュ", phonemes: &[cv("ch", "u")] },
    KanaEntry { kana: "チョ", phonemes: &[cv("ch", "o")] },
    // ニャ行
    KanaEntry { kana: "ニャ", phonemes: &[cv("ny", "a")] },
    KanaEntry { kana: "ニュ", phonemes: &[cv("ny", "u")] },
    KanaEntry { kana: "ニョ", phonemes: &[cv("ny", "o")] },
    // ヒャ行
    KanaEntry { kana: "ヒャ", phonemes: &[cv("hy", "a")] },
    KanaEntry { kana: "ヒュ", phonemes: &[cv("hy", "u")] },
    KanaEntry { kana: "ヒョ", phonemes: &[cv("hy", "o")] },
    // ミャ行
    KanaEntry { kana: "ミャ", phonemes: &[cv("my", "a")] },
    KanaEntry { kana: "ミュ", phonemes: &[cv("my", "u")] },
    KanaEntry { kana: "ミョ", phonemes: &[cv("my", "o")] },
    // リャ行
    KanaEntry { kana: "リャ", phonemes: &[cv("ry", "a")] },
    KanaEntry { kana: "リュ", phonemes: &[cv("ry", "u")] },
    KanaEntry { kana: "リョ", phonemes: &[cv("ry", "o")] },
    // ギャ行
    KanaEntry { kana: "ギャ", phonemes: &[cv("gy", "a")] },
    KanaEntry { kana: "ギュ", phonemes: &[cv("gy", "u")] },
    KanaEntry { kana: "ギョ", phonemes: &[cv("gy", "o")] },
    // ジャ行
    KanaEntry { kana: "ジャ", phonemes: &[cv("j", "a")] },
    KanaEntry { kana: "ジュ", phonemes: &[cv("j", "u")] },
    KanaEntry { kana: "ジョ", phonemes: &[cv("j", "o")] },
    // ビャ行
    KanaEntry { kana: "ビャ", phonemes: &[cv("by", "a")] },
    KanaEntry { kana: "ビュ", phonemes: &[cv("by", "u")] },
    KanaEntry { kana: "ビョ", phonemes: &[cv("by", "o")] },
    // ピャ行
    KanaEntry { kana: "ピャ", phonemes: &[cv("py", "a")] },
    KanaEntry { kana: "ピュ", phonemes: &[cv("py", "u")] },
    KanaEntry { kana: "ピョ", phonemes: &[cv("py", "o")] },
    // 外来語系 (ティ, ディ, ファ, etc.)
    KanaEntry { kana: "ティ", phonemes: &[cv("t", "i")] },
    KanaEntry { kana: "ディ", phonemes: &[cv("d", "i")] },
    KanaEntry { kana: "トゥ", phonemes: &[cv("t", "u")] },
    KanaEntry { kana: "ドゥ", phonemes: &[cv("d", "u")] },
    KanaEntry { kana: "ファ", phonemes: &[cv("f", "a")] },
    KanaEntry { kana: "フィ", phonemes: &[cv("f", "i")] },
    KanaEntry { kana: "フェ", phonemes: &[cv("f", "e")] },
    KanaEntry { kana: "フォ", phonemes: &[cv("f", "o")] },
    KanaEntry { kana: "ヴァ", phonemes: &[cv("v", "a")] },
    KanaEntry { kana: "ヴィ", phonemes: &[cv("v", "i")] },
    KanaEntry { kana: "ヴェ", phonemes: &[cv("v", "e")] },
    KanaEntry { kana: "ヴォ", phonemes: &[cv("v", "o")] },
    KanaEntry { kana: "ウィ", phonemes: &[cv("w", "i")] },
    KanaEntry { kana: "ウェ", phonemes: &[cv("w", "e")] },
    KanaEntry { kana: "ウォ", phonemes: &[cv("w", "o")] },
    KanaEntry { kana: "ツァ", phonemes: &[cv("ts", "a")] },
    KanaEntry { kana: "ツィ", phonemes: &[cv("ts", "i")] },
    KanaEntry { kana: "ツェ", phonemes: &[cv("ts", "e")] },
    KanaEntry { kana: "ツォ", phonemes: &[cv("ts", "o")] },
    KanaEntry { kana: "デュ", phonemes: &[cv("dy", "u")] },
    KanaEntry { kana: "テュ", phonemes: &[cv("ty", "u")] },
];

// Single-character kana entries.
static SINGLE_CHAR_ENTRIES: &[KanaEntry] = &[
    // ア行 (vowels)
    KanaEntry { kana: "ア", phonemes: &[Phoneme::Vowel("a")] },
    KanaEntry { kana: "イ", phonemes: &[Phoneme::Vowel("i")] },
    KanaEntry { kana: "ウ", phonemes: &[Phoneme::Vowel("u")] },
    KanaEntry { kana: "エ", phonemes: &[Phoneme::Vowel("e")] },
    KanaEntry { kana: "オ", phonemes: &[Phoneme::Vowel("o")] },
    // カ行
    KanaEntry { kana: "カ", phonemes: &[cv("k", "a")] },
    KanaEntry { kana: "キ", phonemes: &[cv("k", "i")] },
    KanaEntry { kana: "ク", phonemes: &[cv("k", "u")] },
    KanaEntry { kana: "ケ", phonemes: &[cv("k", "e")] },
    KanaEntry { kana: "コ", phonemes: &[cv("k", "o")] },
    // サ行
    KanaEntry { kana: "サ", phonemes: &[cv("s", "a")] },
    KanaEntry { kana: "シ", phonemes: &[cv("sh", "i")] },
    KanaEntry { kana: "ス", phonemes: &[cv("s", "u")] },
    KanaEntry { kana: "セ", phonemes: &[cv("s", "e")] },
    KanaEntry { kana: "ソ", phonemes: &[cv("s", "o")] },
    // タ行
    KanaEntry { kana: "タ", phonemes: &[cv("t", "a")] },
    KanaEntry { kana: "チ", phonemes: &[cv("ch", "i")] },
    KanaEntry { kana: "ツ", phonemes: &[cv("ts", "u")] },
    KanaEntry { kana: "テ", phonemes: &[cv("t", "e")] },
    KanaEntry { kana: "ト", phonemes: &[cv("t", "o")] },
    // ナ行
    KanaEntry { kana: "ナ", phonemes: &[cv("n", "a")] },
    KanaEntry { kana: "ニ", phonemes: &[cv("n", "i")] },
    KanaEntry { kana: "ヌ", phonemes: &[cv("n", "u")] },
    KanaEntry { kana: "ネ", phonemes: &[cv("n", "e")] },
    KanaEntry { kana: "ノ", phonemes: &[cv("n", "o")] },
    // ハ行
    KanaEntry { kana: "ハ", phonemes: &[cv("h", "a")] },
    KanaEntry { kana: "ヒ", phonemes: &[cv("h", "i")] },
    KanaEntry { kana: "フ", phonemes: &[cv("f", "u")] },
    KanaEntry { kana: "ヘ", phonemes: &[cv("h", "e")] },
    KanaEntry { kana: "ホ", phonemes: &[cv("h", "o")] },
    // マ行
    KanaEntry { kana: "マ", phonemes: &[cv("m", "a")] },
    KanaEntry { kana: "ミ", phonemes: &[cv("m", "i")] },
    KanaEntry { kana: "ム", phonemes: &[cv("m", "u")] },
    KanaEntry { kana: "メ", phonemes: &[cv("m", "e")] },
    KanaEntry { kana: "モ", phonemes: &[cv("m", "o")] },
    // ヤ行
    KanaEntry { kana: "ヤ", phonemes: &[cv("y", "a")] },
    KanaEntry { kana: "ユ", phonemes: &[cv("y", "u")] },
    KanaEntry { kana: "ヨ", phonemes: &[cv("y", "o")] },
    // ラ行
    KanaEntry { kana: "ラ", phonemes: &[cv("r", "a")] },
    KanaEntry { kana: "リ", phonemes: &[cv("r", "i")] },
    KanaEntry { kana: "ル", phonemes: &[cv("r", "u")] },
    KanaEntry { kana: "レ", phonemes: &[cv("r", "e")] },
    KanaEntry { kana: "ロ", phonemes: &[cv("r", "o")] },
    // ワ行
    KanaEntry { kana: "ワ", phonemes: &[cv("w", "a")] },
    KanaEntry { kana: "ヲ", phonemes: &[Phoneme::Vowel("o")] },
    // ン
    KanaEntry { kana: "ン", phonemes: &[Phoneme::Nasal] },
    // ガ行
    KanaEntry { kana: "ガ", phonemes: &[cv("g", "a")] },
    KanaEntry { kana: "ギ", phonemes: &[cv("g", "i")] },
    KanaEntry { kana: "グ", phonemes: &[cv("g", "u")] },
    KanaEntry { kana: "ゲ", phonemes: &[cv("g", "e")] },
    KanaEntry { kana: "ゴ", phonemes: &[cv("g", "o")] },
    // ザ行
    KanaEntry { kana: "ザ", phonemes: &[cv("z", "a")] },
    KanaEntry { kana: "ジ", phonemes: &[cv("j", "i")] },
    KanaEntry { kana: "ズ", phonemes: &[cv("z", "u")] },
    KanaEntry { kana: "ゼ", phonemes: &[cv("z", "e")] },
    KanaEntry { kana: "ゾ", phonemes: &[cv("z", "o")] },
    // ダ行
    KanaEntry { kana: "ダ", phonemes: &[cv("d", "a")] },
    KanaEntry { kana: "ヂ", phonemes: &[cv("j", "i")] },
    KanaEntry { kana: "ヅ", phonemes: &[cv("z", "u")] },
    KanaEntry { kana: "デ", phonemes: &[cv("d", "e")] },
    KanaEntry { kana: "ド", phonemes: &[cv("d", "o")] },
    // バ行
    KanaEntry { kana: "バ", phonemes: &[cv("b", "a")] },
    KanaEntry { kana: "ビ", phonemes: &[cv("b", "i")] },
    KanaEntry { kana: "ブ", phonemes: &[cv("b", "u")] },
    KanaEntry { kana: "ベ", phonemes: &[cv("b", "e")] },
    KanaEntry { kana: "ボ", phonemes: &[cv("b", "o")] },
    // パ行
    KanaEntry { kana: "パ", phonemes: &[cv("p", "a")] },
    KanaEntry { kana: "ピ", phonemes: &[cv("p", "i")] },
    KanaEntry { kana: "プ", phonemes: &[cv("p", "u")] },
    KanaEntry { kana: "ペ", phonemes: &[cv("p", "e")] },
    KanaEntry { kana: "ポ", phonemes: &[cv("p", "o")] },
    // ヴ (standalone)
    KanaEntry { kana: "ヴ", phonemes: &[cv("v", "u")] },
];

/// Convert a normalized katakana string into a sequence of phonemes.
/// Characters marked with LONG_VOWEL_MARKER produce LongVowel phonemes.
pub fn to_phonemes(katakana: &str) -> Vec<Phoneme> {
    let mut result = Vec::new();
    let chars: Vec<char> = katakana.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for long vowel marker
        if chars[i] == LONG_VOWEL_MARKER {
            // Next char is the vowel kana — convert to LongVowel
            if i + 1 < chars.len() {
                let vowel_kana = chars[i + 1];
                if let Some(v) = vowel_to_str(vowel_kana) {
                    result.push(Phoneme::LongVowel(v));
                    i += 2;
                    continue;
                }
            }
            i += 1;
            continue;
        }

        // Try multi-char match (2 chars)
        if i + 1 < chars.len() {
            let two: String = chars[i..=i + 1].iter().collect();
            if let Some(entry) = MULTI_CHAR_ENTRIES.iter().find(|e| e.kana == two) {
                result.extend_from_slice(entry.phonemes);
                i += 2;
                continue;
            }
        }

        // Try single-char match
        let one: String = chars[i..=i].iter().collect();
        if let Some(entry) = SINGLE_CHAR_ENTRIES.iter().find(|e| e.kana == one) {
            result.extend_from_slice(entry.phonemes);
            i += 1;
            continue;
        }

        // Unknown character: skip
        i += 1;
    }

    result
}

/// Marker character inserted before long-vowel-derived vowels by the normalizer.
/// Uses a private-use Unicode character that won't appear in normal text.
pub const LONG_VOWEL_MARKER: char = '\u{F0000}';

/// Convert a katakana vowel character to a static str.
fn vowel_to_str(c: char) -> Option<&'static str> {
    match c {
        'ア' => Some("a"),
        'イ' => Some("i"),
        'ウ' => Some("u"),
        'エ' => Some("e"),
        'オ' => Some("o"),
        _ => None,
    }
}

/// Count the length of the common prefix between two phoneme sequences.
pub fn common_prefix_len(a: &[Phoneme], b: &[Phoneme]) -> usize {
    a.iter().zip(b.iter()).take_while(|(x, y)| phoneme_eq_relaxed(x, y)).count()
}

/// Relaxed equality: LongVowel("a") == Vowel("a") for prefix matching.
fn phoneme_eq_relaxed(a: &Phoneme, b: &Phoneme) -> bool {
    if a == b {
        return true;
    }
    match (a, b) {
        (Phoneme::Vowel(v1), Phoneme::LongVowel(v2))
        | (Phoneme::LongVowel(v1), Phoneme::Vowel(v2))
        | (Phoneme::LongVowel(v1), Phoneme::LongVowel(v2)) => v1 == v2,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_vowels() {
        let phonemes = to_phonemes("アイウエオ");
        assert_eq!(phonemes, vec![
            Phoneme::Vowel("a"),
            Phoneme::Vowel("i"),
            Phoneme::Vowel("u"),
            Phoneme::Vowel("e"),
            Phoneme::Vowel("o"),
        ]);
    }

    #[test]
    fn test_karubi() {
        let phonemes = to_phonemes("カルビ");
        assert_eq!(phonemes, vec![
            cv("k", "a"),
            cv("r", "u"),
            cv("b", "i"),
        ]);
    }

    #[test]
    fn test_multi_char_di() {
        let phonemes = to_phonemes("ディ");
        assert_eq!(phonemes, vec![cv("d", "i")]);
    }

    #[test]
    fn test_multi_char_shu() {
        let phonemes = to_phonemes("シュ");
        assert_eq!(phonemes, vec![cv("sh", "u")]);
    }

    #[test]
    fn test_nasal() {
        let phonemes = to_phonemes("ン");
        assert_eq!(phonemes, vec![Phoneme::Nasal]);
    }

    #[test]
    fn test_long_vowel_marker() {
        // Simulate what normalizer produces: カルビ + MARKER + イ
        let input = format!("カルビ{}イ", LONG_VOWEL_MARKER);
        let phonemes = to_phonemes(&input);
        assert_eq!(phonemes, vec![
            cv("k", "a"),
            cv("r", "u"),
            cv("b", "i"),
            Phoneme::LongVowel("i"),
        ]);
    }

    #[test]
    fn test_common_prefix_len() {
        let a = vec![cv("k", "a"), cv("r", "u"), cv("b", "i")];
        let b = vec![cv("k", "a"), cv("r", "u"), cv("d", "i")];
        assert_eq!(common_prefix_len(&a, &b), 2);
    }
}
