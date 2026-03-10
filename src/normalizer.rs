use unicode_normalization::UnicodeNormalization;

use crate::phoneme::LONG_VOWEL_MARKER;

/// Normalize input text for phonetic comparison.
///
/// Steps:
/// 1. NFKC unicode normalization
/// 2. Convert ASCII alphabets to katakana letter-readings (e.g., B → ビー)
/// 3. Convert digits and symbols to katakana readings (e.g., & → アンド)
/// 4. Hiragana to Katakana conversion
/// 5. Resolve long vowel mark (ー) to the preceding vowel, marked as long-vowel-derived
/// 6. Remove sokuon (ッ) for lenient matching
/// 7. Strip non-katakana characters (preserving long vowel markers)
pub fn normalize(input: &str) -> String {
    let nfkc: String = input.nfkc().collect();
    let alpha_converted = convert_ascii(&nfkc);
    let katakana = hiragana_to_katakana(&alpha_converted);
    let resolved = resolve_long_vowel(&katakana);
    let no_sokuon = remove_sokuon(&resolved);
    filter_katakana(&no_sokuon)
}

/// Convert ASCII letters, digits, and symbols to katakana readings.
///
/// Letters are converted to their Japanese letter-name readings (e.g., A → エー, B → ビー).
/// When STT outputs alphabet text like "IBM", the original speech was letter-by-letter
/// ("アイビーエム"), so this conversion recovers the original phonetic content.
///
/// Digits use English readings when alphabets are present in the input (e.g., 3M → スリーエム),
/// and Japanese readings otherwise (e.g., 3 → サン). This is because brand names mixing
/// letters and numbers almost always use English number readings.
///
/// Common symbols are converted (e.g., & → アンド, + → プラス).
fn convert_ascii(input: &str) -> String {
    let has_alpha = input.chars().any(|c| c.is_ascii_alphabetic());
    let mut result = String::with_capacity(input.len() * 3);
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            result.push_str(letter_to_katakana(c));
        } else if c.is_ascii_digit() {
            if has_alpha {
                result.push_str(digit_to_english(c));
            } else {
                result.push_str(digit_to_japanese(c));
            }
        } else {
            match c {
                '&' => result.push_str("アンド"),
                '+' => result.push_str("プラス"),
                _ if c.is_ascii() && !c.is_ascii_whitespace() => {
                    // Other ASCII punctuation (!, ., -, etc.) — skip
                }
                _ => result.push(c),
            }
        }
    }
    result
}

/// Convert an ASCII letter to its Japanese letter-name reading in katakana.
fn letter_to_katakana(c: char) -> &'static str {
    match c.to_ascii_uppercase() {
        'A' => "エー",
        'B' => "ビー",
        'C' => "シー",
        'D' => "ディー",
        'E' => "イー",
        'F' => "エフ",
        'G' => "ジー",
        'H' => "エイチ",
        'I' => "アイ",
        'J' => "ジェイ",
        'K' => "ケー",
        'L' => "エル",
        'M' => "エム",
        'N' => "エヌ",
        'O' => "オー",
        'P' => "ピー",
        'Q' => "キュー",
        'R' => "アール",
        'S' => "エス",
        'T' => "ティー",
        'U' => "ユー",
        'V' => "ブイ",
        'W' => "ダブリュー",
        'X' => "エックス",
        'Y' => "ワイ",
        'Z' => "ゼット",
        _ => "",
    }
}

/// Convert a digit to English reading in katakana (used when mixed with alphabets).
fn digit_to_english(c: char) -> &'static str {
    match c {
        '0' => "ゼロ",
        '1' => "ワン",
        '2' => "ツー",
        '3' => "スリー",
        '4' => "フォー",
        '5' => "ファイブ",
        '6' => "シックス",
        '7' => "セブン",
        '8' => "エイト",
        '9' => "ナイン",
        _ => "",
    }
}

/// Convert a digit to Japanese reading in katakana (used when no alphabets present).
fn digit_to_japanese(c: char) -> &'static str {
    match c {
        '0' => "ゼロ",
        '1' => "イチ",
        '2' => "ニ",
        '3' => "サン",
        '4' => "ヨン",
        '5' => "ゴ",
        '6' => "ロク",
        '7' => "ナナ",
        '8' => "ハチ",
        '9' => "キュウ",
        _ => "",
    }
}

/// Convert hiragana characters (U+3041..U+3096) to katakana (U+30A1..U+30F6).
fn hiragana_to_katakana(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if ('\u{3041}'..='\u{3096}').contains(&c) {
                char::from_u32(c as u32 + 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

/// Replace long vowel mark (ー) with LONG_VOWEL_MARKER + vowel kana.
/// This allows the phoneme layer to distinguish long-vowel-derived vowels.
fn resolve_long_vowel(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::with_capacity(input.len());

    for (i, &c) in chars.iter().enumerate() {
        if c == 'ー' {
            if let Some(vowel) = find_preceding_vowel(&chars, i) {
                result.push(LONG_VOWEL_MARKER);
                result.push(vowel);
            }
            // If no preceding vowel found, drop the ー
        } else {
            result.push(c);
        }
    }

    result
}

/// Find the vowel sound of the kana preceding position `pos`.
fn find_preceding_vowel(chars: &[char], pos: usize) -> Option<char> {
    if pos == 0 {
        return None;
    }
    // Skip any long vowel markers when looking back
    let mut idx = pos - 1;
    while chars[idx] == LONG_VOWEL_MARKER && idx > 0 {
        idx -= 1;
    }
    let prev = chars[idx];
    // If the previous char is a vowel kana that was itself a long vowel resolution,
    // use it directly.
    vowel_of_katakana(prev)
}

/// Return the vowel katakana character for a given katakana character.
fn vowel_of_katakana(c: char) -> Option<char> {
    match c {
        // ア段
        'ア' | 'カ' | 'サ' | 'タ' | 'ナ' | 'ハ' | 'マ' | 'ヤ' | 'ラ' | 'ワ'
        | 'ガ' | 'ザ' | 'ダ' | 'バ' | 'パ' => Some('ア'),
        // イ段
        'イ' | 'キ' | 'シ' | 'チ' | 'ニ' | 'ヒ' | 'ミ' | 'リ'
        | 'ギ' | 'ジ' | 'ヂ' | 'ビ' | 'ピ' => Some('イ'),
        // ウ段
        'ウ' | 'ク' | 'ス' | 'ツ' | 'ヌ' | 'フ' | 'ム' | 'ユ' | 'ル'
        | 'グ' | 'ズ' | 'ヅ' | 'ブ' | 'プ' | 'ヴ' => Some('ウ'),
        // エ段
        'エ' | 'ケ' | 'セ' | 'テ' | 'ネ' | 'ヘ' | 'メ' | 'レ'
        | 'ゲ' | 'ゼ' | 'デ' | 'ベ' | 'ペ' => Some('エ'),
        // オ段
        'オ' | 'コ' | 'ソ' | 'ト' | 'ノ' | 'ホ' | 'モ' | 'ヨ' | 'ロ' | 'ヲ'
        | 'ゴ' | 'ゾ' | 'ド' | 'ボ' | 'ポ' => Some('オ'),
        // 小書きカナ (拗音の後半)
        'ャ' => Some('ア'),
        'ュ' => Some('ウ'),
        'ョ' => Some('オ'),
        'ァ' => Some('ア'),
        'ィ' => Some('イ'),
        'ゥ' => Some('ウ'),
        'ェ' => Some('エ'),
        'ォ' => Some('オ'),
        // ン
        'ン' => None,
        _ => None,
    }
}

/// Remove sokuon (ッ) from the string.
fn remove_sokuon(input: &str) -> String {
    input.chars().filter(|&c| c != 'ッ').collect()
}

/// Keep only katakana characters and long vowel markers.
fn filter_katakana(input: &str) -> String {
    input
        .chars()
        .filter(|&c| ('\u{30A0}'..='\u{30FF}').contains(&c) || c == LONG_VOWEL_MARKER)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hiragana_to_katakana() {
        assert_eq!(normalize("かるび"), "カルビ");
    }

    #[test]
    fn test_long_vowel_resolution() {
        // センター -> セ ン タ MARKER ア
        let result = normalize("センター");
        assert_eq!(result, format!("センタ{}ア", LONG_VOWEL_MARKER));
    }

    #[test]
    fn test_sokuon_removal() {
        let result = normalize("トラック");
        assert_eq!(result, "トラク");
    }

    #[test]
    fn test_combined_normalization() {
        // カルビー -> カルビ + MARKER + イ
        let result = normalize("カルビー");
        assert_eq!(result, format!("カルビ{}イ", LONG_VOWEL_MARKER));
    }

    #[test]
    fn test_already_katakana() {
        assert_eq!(normalize("カルビ"), "カルビ");
    }

    #[test]
    fn test_strip_non_kana() {
        assert_eq!(normalize("カルビ!"), "カルビ");
    }

    #[test]
    fn test_empty() {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn test_ascii_letter_conversion() {
        // IBM → アイビーエム
        let result = normalize("IBM");
        assert!(result.contains("アイ"));
        assert!(result.contains("エム"));
    }

    #[test]
    fn test_ascii_with_symbol() {
        // P&G → ピーアンドジー
        let result = normalize("P&G");
        assert!(result.contains("ピ"));
        assert!(result.contains("アンド"));
        assert!(result.contains("ジ"));
    }

    #[test]
    fn test_ascii_case_insensitive() {
        assert_eq!(normalize("ibm"), normalize("IBM"));
    }

    #[test]
    fn test_digit_with_alpha_english_reading() {
        // 3M → スリーエム (English reading when mixed with alphabets)
        let result = normalize("3M");
        assert!(result.contains("スリ"));
        assert!(result.contains("エム"));
    }

    #[test]
    fn test_digit_only_japanese_reading() {
        // Pure digits use Japanese reading
        let result = normalize("3");
        assert!(result.contains("サン"));
    }

    #[test]
    fn test_mixed_kana_and_ascii() {
        // カナ部分はそのまま、ASCII部分だけ変換
        let result = normalize("NTTドコモ");
        assert!(result.contains("ドコモ"));
        assert!(result.contains("エヌ"));
    }
}