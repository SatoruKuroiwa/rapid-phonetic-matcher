use crate::phoneme::Phoneme;

/// Calculate the substitution cost between two phonemes.
/// Returns a value in [0.0, 1.0] where 0.0 = identical, 1.0 = completely different.
///
/// Consonant and vowel costs are combined with weights:
///   total = 0.6 * consonant_cost + 0.4 * vowel_cost
/// This keeps substitution costs on the same scale as insertion/deletion (1.0).
pub fn substitution_cost(a: &Phoneme, b: &Phoneme) -> f32 {
    if a == b {
        return 0.0;
    }

    // LongVowel vs Vowel with same value: free substitution
    match (a, b) {
        (Phoneme::Vowel(v1), Phoneme::LongVowel(v2))
        | (Phoneme::LongVowel(v1), Phoneme::Vowel(v2)) if v1 == v2 => return 0.0,
        (Phoneme::LongVowel(v1), Phoneme::LongVowel(v2)) if v1 == v2 => return 0.0,
        _ => {}
    }

    match (a, b) {
        // Both are CV: compare consonant and vowel components
        (Phoneme::CV { consonant: c1, vowel: v1 }, Phoneme::CV { consonant: c2, vowel: v2 }) => {
            0.6 * consonant_cost(c1, c2) + 0.4 * vowel_cost(v1, v2)
        }
        // Vowel vs Vowel (including LongVowel)
        (a, b) if is_vowel_like(a) && is_vowel_like(b) => {
            let v1 = a.vowel_value().unwrap();
            let v2 = b.vowel_value().unwrap();
            vowel_cost(v1, v2)
        }
        // CV vs Vowel-like: consonant is "deleted", vowels compared
        (Phoneme::CV { vowel: v1, .. }, other) | (other, Phoneme::CV { vowel: v1, .. })
            if is_vowel_like(other) =>
        {
            let v2 = other.vowel_value().unwrap();
            0.6 * 1.0 + 0.4 * vowel_cost(v1, v2)
        }
        // Nasal vs anything else
        (Phoneme::Nasal, other) | (other, Phoneme::Nasal) => {
            match other {
                Phoneme::CV { consonant: "n", .. } | Phoneme::CV { consonant: "m", .. } => 0.4,
                _ => 1.0,
            }
        }
        _ => 1.0,
    }
}

fn is_vowel_like(p: &Phoneme) -> bool {
    matches!(p, Phoneme::Vowel(_) | Phoneme::LongVowel(_))
}

/// Substitution cost between two consonants.
fn consonant_cost(c1: &str, c2: &str) -> f32 {
    if c1 == c2 {
        return 0.0;
    }

    // Normalize to a sorted pair for symmetric matching
    let pair = normalize_pair(c1, c2);

    match pair {
        // Voiced stops (b, d, g): 0.3
        ("b", "d") | ("b", "g") | ("d", "g") => 0.3,

        // Voiceless/voiced pairs: 0.5
        ("b", "h") | ("h", "p") | ("b", "p") => 0.5,
        ("f", "h") | ("b", "f") | ("f", "p") => 0.5,
        ("s", "z") => 0.5,
        ("g", "k") => 0.5,
        ("d", "t") => 0.5,

        // Palatalized voiceless/voiced pairs: 0.5
        ("gy", "ky") => 0.5,
        ("by", "hy") | ("hy", "py") | ("by", "py") => 0.5,
        ("dy", "ty") => 0.5,
        ("my", "ny") => 0.4,
        ("by", "gy") | ("by", "dy") | ("dy", "gy") => 0.3,

        // Sibilant/affricate pairs
        ("j", "sh") | ("j", "ch") | ("ch", "sh") => 0.5,
        ("j", "z") => 0.4,
        ("s", "sh") | ("s", "ch") | ("ch", "t") | ("s", "ts") | ("t", "ts") => 0.5,

        // Nasals (m, n): 0.4
        ("m", "n") => 0.4,

        // Liquid/flap (r) vs nasals and stops
        ("n", "r") | ("m", "r") => 0.6,
        ("d", "r") => 0.5,

        // Foreign sound pairs
        ("b", "v") | ("d", "v") => 0.3,  // ヴァ/バ — nearly identical in Japanese
        ("f", "v") => 0.4,

        // Other consonant pairs: full cost
        _ => 1.0,
    }
}

/// Substitution cost between two vowels.
/// All different vowel pairs cost 0.8 (per spec).
fn vowel_cost(v1: &str, v2: &str) -> f32 {
    if v1 == v2 {
        0.0
    } else {
        0.8
    }
}

/// Normalize a pair of strings into alphabetical order for symmetric matching.
fn normalize_pair<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Cost for inserting or deleting a regular phoneme.
pub const INDEL_COST: f32 = 1.0;

/// Cost for inserting or deleting a long-vowel-derived phoneme.
/// Much lower than regular indel because long vowel presence/absence
/// is a minor variation (e.g., ソニー vs ソニ).
pub const LONG_VOWEL_INDEL_COST: f32 = 0.3;

/// Return the insertion/deletion cost for a specific phoneme.
pub fn indel_cost(p: &Phoneme) -> f32 {
    if p.is_long_vowel() {
        LONG_VOWEL_INDEL_COST
    } else {
        INDEL_COST
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_phonemes() {
        let p = Phoneme::CV { consonant: "k", vowel: "a" };
        assert_eq!(substitution_cost(&p, &p), 0.0);
    }

    #[test]
    fn test_voiced_stops() {
        let b = Phoneme::CV { consonant: "b", vowel: "a" };
        let d = Phoneme::CV { consonant: "d", vowel: "a" };
        let cost = substitution_cost(&b, &d);
        assert!((cost - 0.18).abs() < 0.001);
    }

    #[test]
    fn test_voiceless_voiced_pair() {
        let k = Phoneme::CV { consonant: "k", vowel: "a" };
        let g = Phoneme::CV { consonant: "g", vowel: "a" };
        let cost = substitution_cost(&k, &g);
        assert!((cost - 0.30).abs() < 0.001);
    }

    #[test]
    fn test_nasals() {
        let m = Phoneme::CV { consonant: "m", vowel: "a" };
        let n = Phoneme::CV { consonant: "n", vowel: "a" };
        let cost = substitution_cost(&m, &n);
        assert!((cost - 0.24).abs() < 0.001);
    }

    #[test]
    fn test_vowel_difference() {
        let a = Phoneme::Vowel("a");
        let i = Phoneme::Vowel("i");
        assert!((substitution_cost(&a, &i) - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_long_vowel_vs_vowel_same() {
        let v = Phoneme::Vowel("i");
        let lv = Phoneme::LongVowel("i");
        assert_eq!(substitution_cost(&v, &lv), 0.0);
    }

    #[test]
    fn test_long_vowel_indel_cost() {
        let lv = Phoneme::LongVowel("a");
        assert_eq!(indel_cost(&lv), LONG_VOWEL_INDEL_COST);
        let v = Phoneme::Vowel("a");
        assert_eq!(indel_cost(&v), INDEL_COST);
    }

    #[test]
    fn test_palatalized_voicing() {
        let ky = Phoneme::CV { consonant: "ky", vowel: "a" };
        let gy = Phoneme::CV { consonant: "gy", vowel: "a" };
        let cost = substitution_cost(&ky, &gy);
        assert!((cost - 0.30).abs() < 0.001, "ky/gy should cost 0.30, got {cost}");
    }

    #[test]
    fn test_v_b_similarity() {
        let va = Phoneme::CV { consonant: "v", vowel: "a" };
        let ba = Phoneme::CV { consonant: "b", vowel: "a" };
        let cost = substitution_cost(&va, &ba);
        assert!((cost - 0.18).abs() < 0.001, "v/b should cost 0.18, got {cost}");
    }
}