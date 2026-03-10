//! # rapid_phonetic_matcher
//!
//! Japanese phonetic similarity matching library for STT (Speech-to-Text) error correction.
//!
//! Compares Japanese text using weighted phoneme edit distance, where acoustically
//! similar sounds (e.g., b/d/g, k/g, m/n) have lower substitution costs than
//! unrelated sounds.
//!
//! # Example
//! ```
//! use rapid_phonetic_matcher::PhoneticMatcher;
//!
//! let matcher = PhoneticMatcher::new();
//! let score = matcher.calculate_similarity("カルミ", "カルビ");
//! assert!(score > 0.7); // High similarity due to m/b being somewhat similar
//! ```

mod cost_matrix;
mod distance;
mod normalizer;
mod phoneme;

#[cfg(feature = "kanji")]
mod kanji;

use normalizer::normalize;
use phoneme::{common_prefix_len, to_phonemes, Phoneme};

/// Result of a phonetic match.
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// The matched candidate text.
    pub text: String,
    /// Similarity score from 0.0 (completely different) to 1.0 (identical).
    pub score: f32,
    /// Confidence level of the match.
    pub confidence: Confidence,
}

/// Confidence level of a match result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    /// No meaningful match (score < 0.4).
    NoMatch,
    /// Low confidence (score 0.4 - 0.6).
    Low,
    /// Medium confidence (score 0.6 - 0.8).
    Medium,
    /// High confidence (score >= 0.8).
    High,
    /// Exact or near-exact match (score >= 0.95).
    Exact,
}

impl Confidence {
    /// Determine confidence level from a similarity score.
    pub fn from_score(score: f32) -> Self {
        if score >= 0.95 {
            Confidence::Exact
        } else if score >= 0.8 {
            Confidence::High
        } else if score >= 0.6 {
            Confidence::Medium
        } else if score >= 0.4 {
            Confidence::Low
        } else {
            Confidence::NoMatch
        }
    }
}

/// Phonetic matcher for Japanese text.
///
/// Compares strings by converting them to phoneme sequences and computing
/// a weighted edit distance that accounts for acoustic similarity between sounds.
pub struct PhoneticMatcher {
    _private: (),
}

impl PhoneticMatcher {
    /// Create a new `PhoneticMatcher` with default configuration.
    pub fn new() -> Self {
        PhoneticMatcher { _private: () }
    }

    /// Calculate the phonetic similarity between two strings.
    ///
    /// Returns a value from 0.0 (completely different) to 1.0 (identical).
    /// Input can be in hiragana, katakana, or a mix.
    pub fn calculate_similarity(&self, input: &str, candidate: &str) -> f32 {
        let pa = self.phonemize(input);
        let pb = self.phonemize(candidate);
        Self::similarity_from_phonemes(&pa, &pb)
    }

    /// Find the top `limit` most similar candidates to `input`, sorted by score descending.
    pub fn find_top_matches(&self, input: &str, candidates: &[&str], limit: usize) -> Vec<MatchResult> {
        let input_phonemes = self.phonemize(input);

        let mut results: Vec<MatchResult> = candidates
            .iter()
            .map(|&c| {
                let cand_phonemes = self.phonemize(c);
                let score = Self::similarity_from_phonemes(&input_phonemes, &cand_phonemes);
                MatchResult {
                    text: c.to_string(),
                    score,
                    confidence: Confidence::from_score(score),
                }
            })
            .collect();

        sort_and_truncate(&mut results, limit);
        results
    }

    /// Find top matches, excluding results below `min_score`.
    pub fn find_matches_filtered(
        &self,
        input: &str,
        candidates: &[&str],
        limit: usize,
        min_score: f32,
    ) -> Vec<MatchResult> {
        let input_phonemes = self.phonemize(input);

        let mut results: Vec<MatchResult> = candidates
            .iter()
            .filter_map(|&c| {
                let cand_phonemes = self.phonemize(c);
                let score = Self::similarity_from_phonemes(&input_phonemes, &cand_phonemes);
                if score >= min_score {
                    Some(MatchResult {
                        text: c.to_string(),
                        score,
                        confidence: Confidence::from_score(score),
                    })
                } else {
                    None
                }
            })
            .collect();

        sort_and_truncate(&mut results, limit);
        results
    }

    /// Normalize and phonemize a string.
    fn phonemize(&self, input: &str) -> Vec<Phoneme> {
        let text = self.preprocess(input);
        let normalized = normalize(&text);
        to_phonemes(&normalized)
    }

    /// Pre-process input text. With the "kanji" feature, converts kanji to katakana reading.
    fn preprocess(&self, input: &str) -> String {
        #[cfg(feature = "kanji")]
        {
            kanji::to_katakana_reading(input)
        }
        #[cfg(not(feature = "kanji"))]
        {
            input.to_string()
        }
    }

    /// Compute similarity from pre-computed phoneme sequences.
    /// Includes prefix bonus for partial matches.
    fn similarity_from_phonemes(a: &[Phoneme], b: &[Phoneme]) -> f32 {
        let max_dist = distance::max_distance(a, b);
        if max_dist == 0.0 {
            return 1.0;
        }
        let dist = distance::weighted_edit_distance(a, b);
        let base_score = (1.0 - dist / max_dist).max(0.0);

        // Prefix bonus: reward when the beginning of both sequences match.
        // This helps when a short input is a prefix of a longer candidate.
        let prefix_len = common_prefix_len(a, b);
        let shorter_len = a.len().min(b.len());
        if shorter_len > 0 && prefix_len > 0 {
            let prefix_ratio = prefix_len as f32 / shorter_len as f32;
            // Only apply bonus when significant prefix overlap exists
            if prefix_ratio >= 0.5 {
                let bonus = (1.0 - base_score) * prefix_ratio * 0.15;
                return (base_score + bonus).min(1.0);
            }
        }

        base_score
    }
}

impl Default for PhoneticMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-computed phoneme data for a set of candidates.
/// Use this when matching the same candidate list against multiple inputs
/// to avoid redundant normalization and phonemization.
pub struct PrecomputedCandidates {
    entries: Vec<(String, Vec<Phoneme>)>,
}

impl PrecomputedCandidates {
    /// Build pre-computed phoneme data from a list of candidate string slices.
    pub fn new(candidates: &[&str]) -> Self {
        let entries = candidates
            .iter()
            .map(|&c| {
                let normalized = normalize(c);
                let phonemes = to_phonemes(&normalized);
                (c.to_string(), phonemes)
            })
            .collect();
        PrecomputedCandidates { entries }
    }

    /// Build pre-computed phoneme data from owned `String` values.
    ///
    /// Useful when candidates come from a database or CSV file.
    pub fn from_strings(candidates: &[String]) -> Self {
        let entries = candidates
            .iter()
            .map(|c| {
                let normalized = normalize(c);
                let phonemes = to_phonemes(&normalized);
                (c.clone(), phonemes)
            })
            .collect();
        PrecomputedCandidates { entries }
    }
}

impl PhoneticMatcher {
    /// Find top matches using pre-computed candidate data.
    pub fn find_top_matches_precomputed(
        &self,
        input: &str,
        candidates: &PrecomputedCandidates,
        limit: usize,
    ) -> Vec<MatchResult> {
        let input_phonemes = self.phonemize(input);

        let mut results: Vec<MatchResult> = candidates
            .entries
            .iter()
            .map(|(text, cand_phonemes)| {
                let score = Self::similarity_from_phonemes(&input_phonemes, cand_phonemes);
                MatchResult {
                    text: text.clone(),
                    score,
                    confidence: Confidence::from_score(score),
                }
            })
            .collect();

        sort_and_truncate(&mut results, limit);
        results
    }
}

// ============================================================
// Alias-aware matching API
// ============================================================

/// A master entry that has a canonical name and one or more phonetic readings.
///
/// Use this when a company/entity has both a formal name and abbreviations.
///
/// # Example
/// ```
/// use rapid_phonetic_matcher::AliasEntry;
///
/// let entry = AliasEntry::new("トヨタ自動車", &["とよたじどうしゃ", "とよた"]);
/// assert_eq!(entry.name(), "トヨタ自動車");
/// assert_eq!(entry.readings().len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct AliasEntry {
    name: String,
    readings: Vec<String>,
}

impl AliasEntry {
    /// Create a new alias entry from string slices.
    ///
    /// * `name` - The canonical display name (e.g., company name).
    /// * `readings` - One or more phonetic readings (formal name, abbreviations, etc.).
    ///
    /// # Panics
    /// Panics if `readings` is empty.
    pub fn new(name: &str, readings: &[&str]) -> Self {
        assert!(!readings.is_empty(), "AliasEntry must have at least one reading");
        AliasEntry {
            name: name.to_string(),
            readings: readings.iter().map(|&r| r.to_string()).collect(),
        }
    }

    /// Create a new alias entry from owned `String` values.
    ///
    /// Useful when readings come from a database or CSV file.
    ///
    /// # Panics
    /// Panics if `readings` is empty.
    pub fn from_strings(name: String, readings: Vec<String>) -> Self {
        assert!(!readings.is_empty(), "AliasEntry must have at least one reading");
        AliasEntry { name, readings }
    }

    /// The canonical display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The phonetic readings.
    pub fn readings(&self) -> &[String] {
        &self.readings
    }
}

/// Pre-computed phoneme data for alias entries.
pub struct PrecomputedAliases {
    entries: Vec<AliasPhonemeEntry>,
}

struct AliasPhonemeEntry {
    name: String,
    readings_phonemes: Vec<Vec<Phoneme>>,
}

impl PrecomputedAliases {
    /// Build pre-computed phoneme data from a list of alias entries.
    pub fn new(entries: &[AliasEntry]) -> Self {
        let matcher = PhoneticMatcher::new();
        let entries = entries
            .iter()
            .map(|e| {
                let readings_phonemes = e
                    .readings
                    .iter()
                    .map(|r| matcher.phonemize(r))
                    .collect();
                AliasPhonemeEntry {
                    name: e.name.clone(),
                    readings_phonemes,
                }
            })
            .collect();
        PrecomputedAliases { entries }
    }
}

impl PhoneticMatcher {
    /// Find top matches from alias entries, taking the best score across all readings.
    pub fn find_top_matches_with_aliases(
        &self,
        input: &str,
        entries: &[AliasEntry],
        limit: usize,
    ) -> Vec<MatchResult> {
        let input_phonemes = self.phonemize(input);

        let mut results: Vec<MatchResult> = entries
            .iter()
            .map(|entry| {
                let best_score = entry
                    .readings
                    .iter()
                    .map(|r| {
                        let cand_phonemes = self.phonemize(r);
                        Self::similarity_from_phonemes(&input_phonemes, &cand_phonemes)
                    })
                    .fold(0.0_f32, f32::max);

                MatchResult {
                    text: entry.name.clone(),
                    score: best_score,
                    confidence: Confidence::from_score(best_score),
                }
            })
            .collect();

        sort_and_truncate(&mut results, limit);
        results
    }

    /// Find top matches using pre-computed alias data.
    pub fn find_top_matches_with_aliases_precomputed(
        &self,
        input: &str,
        candidates: &PrecomputedAliases,
        limit: usize,
    ) -> Vec<MatchResult> {
        let input_phonemes = self.phonemize(input);

        let mut results: Vec<MatchResult> = candidates
            .entries
            .iter()
            .map(|entry| {
                let best_score = entry
                    .readings_phonemes
                    .iter()
                    .map(|ph| Self::similarity_from_phonemes(&input_phonemes, ph))
                    .fold(0.0_f32, f32::max);

                MatchResult {
                    text: entry.name.clone(),
                    score: best_score,
                    confidence: Confidence::from_score(best_score),
                }
            })
            .collect();

        sort_and_truncate(&mut results, limit);
        results
    }

    /// Find alias matches with minimum score filtering.
    pub fn find_matches_with_aliases_filtered(
        &self,
        input: &str,
        entries: &[AliasEntry],
        limit: usize,
        min_score: f32,
    ) -> Vec<MatchResult> {
        let input_phonemes = self.phonemize(input);

        let mut results: Vec<MatchResult> = entries
            .iter()
            .filter_map(|entry| {
                let best_score = entry
                    .readings
                    .iter()
                    .map(|r| {
                        let cand_phonemes = self.phonemize(r);
                        Self::similarity_from_phonemes(&input_phonemes, &cand_phonemes)
                    })
                    .fold(0.0_f32, f32::max);

                if best_score >= min_score {
                    Some(MatchResult {
                        text: entry.name.clone(),
                        score: best_score,
                        confidence: Confidence::from_score(best_score),
                    })
                } else {
                    None
                }
            })
            .collect();

        sort_and_truncate(&mut results, limit);
        results
    }
}

fn sort_and_truncate(results: &mut Vec<MatchResult>, limit: usize) {
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(limit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_strings() {
        let m = PhoneticMatcher::new();
        assert_eq!(m.calculate_similarity("カルビ", "カルビ"), 1.0);
    }

    #[test]
    fn test_empty_strings() {
        let m = PhoneticMatcher::new();
        assert_eq!(m.calculate_similarity("", ""), 1.0);
    }

    #[test]
    fn test_find_top_matches() {
        let m = PhoneticMatcher::new();
        let candidates = vec!["カルビ", "カルディ", "サラダ", "カレー"];
        let results = m.find_top_matches("カルミ", &candidates, 2);
        assert_eq!(results.len(), 2);
        assert!(results[0].score > results[1].score || results[0].score == results[1].score);
    }

    #[test]
    fn test_confidence_levels() {
        assert_eq!(Confidence::from_score(1.0), Confidence::Exact);
        assert_eq!(Confidence::from_score(0.96), Confidence::Exact);
        assert_eq!(Confidence::from_score(0.85), Confidence::High);
        assert_eq!(Confidence::from_score(0.7), Confidence::Medium);
        assert_eq!(Confidence::from_score(0.5), Confidence::Low);
        assert_eq!(Confidence::from_score(0.2), Confidence::NoMatch);
    }

    #[test]
    fn test_filtered_matches() {
        let m = PhoneticMatcher::new();
        let candidates = vec!["カルビ", "カルディ", "サラダ", "カレー"];
        let results = m.find_matches_filtered("カルミ", &candidates, 10, 0.8);
        // Only high-similarity matches should remain
        for r in &results {
            assert!(r.score >= 0.8, "Score {} should be >= 0.8", r.score);
        }
    }

    #[test]
    fn test_long_vowel_improved_score() {
        let m = PhoneticMatcher::new();
        // With long vowel cost reduction, ソニー vs ソニ should score much higher
        let score = m.calculate_similarity("ソニー", "ソニ");
        assert!(score > 0.8, "Long vowel tolerance should give high score, got {score}");
    }

    #[test]
    fn test_prefix_bonus() {
        let m = PhoneticMatcher::new();
        // とよた vs とよたじどうしゃ: prefix match should boost score
        let score = m.calculate_similarity("とよた", "とよたじどうしゃ");
        assert!(score > 0.45, "Prefix bonus should improve score, got {score}");
    }
}