use std::borrow::Cow;

use lindera::dictionary::{DictionaryConfig, DictionaryKind};
use lindera::mode::Mode;
use lindera::segmenter::{Segmenter, SegmenterConfig};
use lindera::tokenizer::Tokenizer;

/// Convert input text containing kanji to katakana reading.
/// Uses lindera morphological analysis (IPADIC) to extract readings.
/// Non-kanji text passes through unchanged.
pub fn to_katakana_reading(input: &str) -> String {
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };

    let config = SegmenterConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };

    let segmenter = Segmenter::from_config(config).expect("Failed to create lindera segmenter");
    let tokenizer = Tokenizer::new(segmenter);
    let mut tokens = tokenizer
        .tokenize(input)
        .expect("Failed to tokenize input");

    let mut result = String::new();
    for token in &mut tokens {
        // IPADIC detail format:
        // [品詞, 品詞細分類1, 品詞細分類2, 品詞細分類3, 活用型, 活用形, 原形, 読み, 発音]
        // Index 7 is the katakana reading
        let detail = token.details();
        if detail.len() > 7 && detail[7] != "*" {
            result.push_str(detail[7]);
        } else {
            // No reading available — use surface form as-is
            result.push_str(&token.text);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kanji_to_katakana() {
        let result = to_katakana_reading("東京");
        assert_eq!(result, "トウキョウ");
    }

    #[test]
    fn test_already_katakana() {
        let result = to_katakana_reading("カルビ");
        assert_eq!(result, "カルビ");
    }

    #[test]
    fn test_mixed_kanji_kana() {
        let result = to_katakana_reading("トヨタ自動車");
        assert!(result.contains("トヨタ"));
        assert!(result.contains("ジドウシャ"));
    }
}