use crate::cost_matrix::{indel_cost, substitution_cost};
use crate::phoneme::Phoneme;

/// Compute the weighted phoneme edit distance between two phoneme sequences.
/// Uses Wagner-Fischer dynamic programming with custom substitution and indel costs.
/// Long-vowel-derived phonemes have reduced insertion/deletion costs.
pub fn weighted_edit_distance(a: &[Phoneme], b: &[Phoneme]) -> f32 {
    let m = a.len();
    let n = b.len();

    // prev[j] = dp[i-1][j], curr[j] = dp[i][j]
    let mut prev: Vec<f32> = Vec::with_capacity(n + 1);
    prev.push(0.0);
    for j in 1..=n {
        prev.push(prev[j - 1] + indel_cost(&b[j - 1]));
    }
    let mut curr = vec![0.0_f32; n + 1];

    for i in 1..=m {
        curr[0] = prev[0] + indel_cost(&a[i - 1]);
        for j in 1..=n {
            let del = prev[j] + indel_cost(&a[i - 1]);
            let ins = curr[j - 1] + indel_cost(&b[j - 1]);
            let sub = prev[j - 1] + substitution_cost(&a[i - 1], &b[j - 1]);
            curr[j] = del.min(ins).min(sub);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

/// Compute the maximum possible edit distance for normalization.
/// Accounts for the fact that long-vowel phonemes have lower indel costs.
pub fn max_distance(a: &[Phoneme], b: &[Phoneme]) -> f32 {
    let cost_a: f32 = a.iter().map(|p| indel_cost(p)).sum();
    let cost_b: f32 = b.iter().map(|p| indel_cost(p)).sum();
    cost_a.max(cost_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phoneme::Phoneme;

    fn cv(c: &'static str, v: &'static str) -> Phoneme {
        Phoneme::CV { consonant: c, vowel: v }
    }

    #[test]
    fn test_identical_sequences() {
        let seq = vec![cv("k", "a"), cv("r", "u"), cv("b", "i")];
        assert_eq!(weighted_edit_distance(&seq, &seq), 0.0);
    }

    #[test]
    fn test_empty_vs_nonempty() {
        let seq = vec![cv("k", "a")];
        assert_eq!(weighted_edit_distance(&[], &seq), 1.0);
        assert_eq!(weighted_edit_distance(&seq, &[]), 1.0);
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(weighted_edit_distance(&[], &[]), 0.0);
    }

    #[test]
    fn test_long_vowel_indel_cheaper() {
        // Sequence with a long vowel at the end vs without
        let with_lv = vec![cv("s", "o"), cv("n", "i"), Phoneme::LongVowel("i")];
        let without = vec![cv("s", "o"), cv("n", "i")];
        let dist = weighted_edit_distance(&with_lv, &without);
        // Deleting LongVowel costs 0.3 instead of 1.0
        assert!((dist - 0.3).abs() < 0.001, "Long vowel indel should cost 0.3, got {dist}");
    }

    #[test]
    fn test_max_distance() {
        let a = vec![cv("k", "a"), Phoneme::LongVowel("a")];
        let b = vec![cv("k", "a")];
        let max = max_distance(&a, &b);
        // a costs: 1.0 + 0.3 = 1.3
        // b costs: 1.0
        assert!((max - 1.3).abs() < 0.001);
    }
}