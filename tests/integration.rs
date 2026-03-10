use rapid_phonetic_matcher::PhoneticMatcher;

fn matcher() -> PhoneticMatcher {
    PhoneticMatcher::new()
}

/// Helper: assert similarity is within an expected range.
fn assert_similarity_range(input: &str, candidate: &str, min: f32, max: f32) {
    let m = matcher();
    let score = m.calculate_similarity(input, candidate);
    assert!(
        score >= min && score <= max,
        "{input} vs {candidate}: expected score in [{min}, {max}], got {score}"
    );
}

// ============================================================
// Section 1: 仕様書記載のテストケース (Spec test cases)
// ============================================================

/// Spec case 1: 有声破裂音の混同 — カルミ vs カルビ
#[test]
fn spec_voiced_stop_confusion() {
    assert_similarity_range("カルミ", "カルビ", 0.7, 1.0);
}

/// Spec case 2: 類似固有名詞 — カルミー vs カルディー
#[test]
fn spec_similar_proper_nouns() {
    assert_similarity_range("カルミー", "カルディー", 0.7, 0.99);
}

/// Spec case 3: 清濁の揺らぎ — バレット vs パレット
#[test]
fn spec_voiceless_voiced_variation() {
    assert_similarity_range("バレット", "パレット", 0.7, 1.0);
}

/// Spec case 4a: 長音の無視 — センター vs センタ
#[test]
fn spec_long_vowel_tolerance() {
    assert_similarity_range("センター", "センタ", 0.7, 1.0);
}

/// Spec case 4b: 促音の無視 — トラック vs トラク
#[test]
fn spec_sokuon_tolerance() {
    assert_similarity_range("トラック", "トラク", 0.7, 1.0);
}

// ============================================================
// Section 2: 有声破裂音グループ (b, d, g) — cost 0.3
// ============================================================

#[test]
fn voiced_stops_ba_da() {
    assert_similarity_range("バグ", "ダグ", 0.85, 1.0);
}

#[test]
fn voiced_stops_ga_da() {
    assert_similarity_range("ガム", "ダム", 0.85, 1.0);
}

#[test]
fn voiced_stops_go_do() {
    assert_similarity_range("ゴボウ", "ドボウ", 0.9, 1.0);
}

#[test]
fn voiced_stops_bagu_dagu() {
    // バグ vs ガグ (b/g swap)
    assert_similarity_range("バグ", "ガグ", 0.85, 1.0);
}

// ============================================================
// Section 3: 清濁ペア (k/g, t/d, s/z, h/b/p) — cost 0.5
// ============================================================

#[test]
fn voicing_k_g() {
    assert_similarity_range("カキ", "ガキ", 0.8, 1.0);
}

#[test]
fn voicing_t_d() {
    assert_similarity_range("タイ", "ダイ", 0.8, 1.0);
}

#[test]
fn voicing_s_z() {
    assert_similarity_range("サル", "ザル", 0.8, 1.0);
}

#[test]
fn voicing_h_b() {
    assert_similarity_range("ハン", "バン", 0.8, 1.0);
}

#[test]
fn voicing_b_p() {
    assert_similarity_range("バス", "パス", 0.8, 1.0);
}

#[test]
fn voicing_b_p_longer_word() {
    assert_similarity_range("バレット", "パレット", 0.85, 1.0);
}

#[test]
fn voicing_k_g_longer_word() {
    assert_similarity_range("カガミ", "カカミ", 0.8, 1.0);
}

#[test]
fn voicing_s_z_in_context() {
    assert_similarity_range("カザリモノ", "カサリモノ", 0.9, 1.0);
}

#[test]
fn voicing_t_d_multiple() {
    // タカダ vs ダカタ (two t/d swaps)
    assert_similarity_range("タカダ", "ダカタ", 0.7, 1.0);
}

// ============================================================
// Section 4: 鼻音ペア (m, n) — cost 0.4
// ============================================================

#[test]
fn nasal_m_n_short() {
    assert_similarity_range("カミ", "カニ", 0.8, 1.0);
}

#[test]
fn nasal_m_n_initial() {
    assert_similarity_range("マル", "ナル", 0.8, 1.0);
}

#[test]
fn nasal_m_n_medial() {
    assert_similarity_range("サンマ", "サンナ", 0.9, 1.0);
}

#[test]
fn nasal_m_n_long_word() {
    // アニメ vs アミメ
    assert_similarity_range("アニメ", "アミメ", 0.85, 1.0);
}

// ============================================================
// Section 5: 母音混同 — cost 0.8
// ============================================================

#[test]
fn vowel_a_i() {
    assert_similarity_range("カラ", "キラ", 0.75, 0.95);
}

#[test]
fn vowel_u_o() {
    assert_similarity_range("スシ", "ソシ", 0.75, 0.95);
}

#[test]
fn vowel_e_a_in_context() {
    assert_similarity_range("テスト", "タスト", 0.8, 0.95);
}

#[test]
fn vowel_i_e() {
    // ミセ(m,i s,e) vs ミシ(m,i sh,i): 子音s/shも母音e/iも異なる
    assert_similarity_range("ミセ", "ミシ", 0.6, 0.95);
}

// ============================================================
// Section 6: 長音記号の有無
// ============================================================

#[test]
fn long_vowel_biiru_biru() {
    assert_similarity_range("ビール", "ビル", 0.6, 1.0);
}

#[test]
fn long_vowel_meeru_meru() {
    assert_similarity_range("メール", "メル", 0.6, 1.0);
}

#[test]
fn long_vowel_karee_kare() {
    assert_similarity_range("カレー", "カレ", 0.6, 1.0);
}

#[test]
fn long_vowel_sonii_soni() {
    assert_similarity_range("ソニー", "ソニ", 0.6, 1.0);
}

#[test]
fn long_vowel_both_present() {
    // 同じ長音付き同士は完全一致
    assert_similarity_range("カレー", "カレー", 1.0, 1.0);
}

// ============================================================
// Section 7: 促音の有無
// ============================================================

#[test]
fn sokuon_machi_macchi() {
    assert_similarity_range("マッチ", "マチ", 0.95, 1.0);
}

#[test]
fn sokuon_roketto_roketo() {
    assert_similarity_range("ロケット", "ロケト", 0.95, 1.0);
}

#[test]
fn sokuon_sakkaa_saka() {
    // 促音 + 長音の両方が省略されるケース
    assert_similarity_range("サッカー", "サカ", 0.6, 1.0);
}

#[test]
fn sokuon_kippu_kipu() {
    assert_similarity_range("キップ", "キプ", 0.95, 1.0);
}

// ============================================================
// Section 8: 拗音・合拗音を含むペア (sh/s, ch/t, j/z, etc.)
// ============================================================

#[test]
fn palatalized_sh_s() {
    // シャチ vs サチ (sh/s confusion)
    assert_similarity_range("シャチ", "サチ", 0.8, 1.0);
}

#[test]
fn palatalized_ch_t() {
    // チャンス vs タンス (ch/t confusion)
    assert_similarity_range("チャンス", "タンス", 0.85, 1.0);
}

#[test]
fn palatalized_j_z() {
    // ジュース vs ズース (j/z confusion)
    assert_similarity_range("ジュース", "ズース", 0.85, 1.0);
}

#[test]
fn palatalized_sh_ch() {
    // シャツ vs チャツ (sh/ch confusion)
    assert_similarity_range("シャツ", "チャツ", 0.8, 1.0);
}

#[test]
fn foreign_di_de() {
    // ディスプレイ vs デスプレイ (di/de — same consonant, vowel diff)
    assert_similarity_range("ディスプレイ", "デスプレイ", 0.9, 1.0);
}

// ============================================================
// Section 9: f/h 混同 (外来語でよくある)
// ============================================================

#[test]
fn fh_confusion_feisu_heisu() {
    assert_similarity_range("フェイスマスク", "ヘイスマスク", 0.9, 1.0);
}

#[test]
fn fh_confusion_faibu_haibu() {
    assert_similarity_range("ファイブ", "ハイブ", 0.8, 1.0);
}

#[test]
fn fh_confusion_fi_hi() {
    assert_similarity_range("フィルム", "ヒルム", 0.8, 1.0);
}

// ============================================================
// Section 10: STT頻出誤変換パターン (実用的なケース)
// ============================================================

#[test]
fn stt_voicing_z_s() {
    // ゾウキン vs ソウキン (z/s混同)
    assert_similarity_range("ゾウキン", "ソウキン", 0.9, 1.0);
}

#[test]
fn stt_voicing_k_g_long() {
    // カガミ vs カカミ (k/g混同)
    assert_similarity_range("カガミ", "カカミ", 0.8, 1.0);
}

#[test]
fn stt_voicing_s_z_long() {
    // カザグルマ vs カサクルマ (s/z + k/g混同)
    assert_similarity_range("カザグルマ", "カサクルマ", 0.8, 1.0);
}

#[test]
fn stt_long_vowel_sokuon() {
    // スーパーマーケット vs スパマケト (長音+促音省略)
    assert_similarity_range("スーパーマーケット", "スパマケト", 0.5, 0.9);
}

#[test]
fn stt_voicing_t_d_multi() {
    // タカダ vs ダカタ (t/d 2箇所)
    assert_similarity_range("タカダ", "ダカタ", 0.7, 1.0);
}

#[test]
fn stt_voicing_d_t() {
    // ドングリ vs トンクリ (d/t + g/k混同)
    assert_similarity_range("ドングリ", "トンクリ", 0.8, 1.0);
}

#[test]
fn stt_voicing_b_p_long() {
    // バラエティー vs パラエティ (b/p + 長音省略)
    assert_similarity_range("バラエティー", "パラエティ", 0.8, 1.0);
}

#[test]
fn stt_denki_tenki() {
    // 電気 vs 天気 (d/t swap — よくある誤認識)
    assert_similarity_range("デンキ", "テンキ", 0.8, 1.0);
}

#[test]
fn stt_kaigi_gaigi() {
    // 会議 vs (g/k swap)
    assert_similarity_range("カイギ", "ガイギ", 0.8, 1.0);
}

// ============================================================
// Section 11: 完全に異なる単語 — 低スコアであること
// ============================================================

#[test]
fn different_ringo_banana() {
    assert_similarity_range("リンゴ", "バナナ", 0.0, 0.4);
}

#[test]
fn different_sakura_himawari() {
    assert_similarity_range("サクラ", "ヒマワリ", 0.0, 0.4);
}

#[test]
fn different_terebi_rajio() {
    assert_similarity_range("テレビ", "ラジオ", 0.0, 0.3);
}

#[test]
fn different_neko_inu() {
    assert_similarity_range("ネコ", "イヌ", 0.0, 0.3);
}

#[test]
fn different_yama_umi() {
    assert_similarity_range("ヤマ", "ウミ", 0.0, 0.4);
}

#[test]
fn different_long_vs_short() {
    // 長さが大きく異なる場合も低スコア
    assert_similarity_range("ア", "コンピューター", 0.0, 0.2);
}

// ============================================================
// Section 12: 同一語・完全一致
// ============================================================

#[test]
fn identical_computer() {
    assert_similarity_range("コンピューター", "コンピューター", 1.0, 1.0);
}

#[test]
fn identical_internet() {
    assert_similarity_range("インターネット", "インターネット", 1.0, 1.0);
}

#[test]
fn identical_short() {
    assert_similarity_range("ア", "ア", 1.0, 1.0);
}

#[test]
fn identical_empty() {
    assert_similarity_range("", "", 1.0, 1.0);
}

// ============================================================
// Section 13: ひらがな入力の正規化
// ============================================================

#[test]
fn normalize_hiragana_to_katakana() {
    let m = matcher();
    let score_kata = m.calculate_similarity("カルビ", "カルビ");
    let score_hira = m.calculate_similarity("かるび", "カルビ");
    assert_eq!(score_kata, score_hira);
}

#[test]
fn normalize_hiragana_both_sides() {
    let m = matcher();
    let score = m.calculate_similarity("かるび", "かるび");
    assert_eq!(score, 1.0);
}

#[test]
fn normalize_mixed_hira_kata() {
    let m = matcher();
    let score = m.calculate_similarity("かルビ", "カルビ");
    assert_eq!(score, 1.0);
}

// ============================================================
// Section 14: find_top_matches の順序テスト
// ============================================================

#[test]
fn top_matches_ordering() {
    let m = matcher();
    let candidates = vec!["カルビ", "カルディ", "サラダ", "カレー", "カルタ"];
    let results = m.find_top_matches("カルミ", &candidates, 5);

    // カルビ should be the top match (only m/b difference)
    assert_eq!(results[0].text, "カルビ");
    // カルディ should be second (m→d is voiced stop pair too, though m is nasal)
    assert_eq!(results[1].text, "カルディ");
    // サラダ should be last or near last
    assert!(results.last().unwrap().score < results[0].score);
}

#[test]
fn top_matches_limit() {
    let m = matcher();
    let candidates = vec!["カルビ", "カルディ", "サラダ", "カレー", "カルタ"];
    let results = m.find_top_matches("カルミ", &candidates, 2);
    assert_eq!(results.len(), 2);
}

#[test]
fn top_matches_empty_candidates() {
    let m = matcher();
    let results = m.find_top_matches("カルミ", &[], 5);
    assert!(results.is_empty());
}

// ============================================================
// Section 15: PrecomputedCandidates の一致テスト
// ============================================================

#[test]
fn precomputed_matches_normal() {
    use rapid_phonetic_matcher::PrecomputedCandidates;

    let m = matcher();
    let candidates = vec!["カルビ", "カルディ", "サラダ", "テレビ", "ラジオ"];

    let results_normal = m.find_top_matches("カルミ", &candidates, 5);
    let precomputed = PrecomputedCandidates::new(&candidates);
    let results_pre = m.find_top_matches_precomputed("カルミ", &precomputed, 5);

    for (a, b) in results_normal.iter().zip(results_pre.iter()) {
        assert_eq!(a.text, b.text);
        assert!((a.score - b.score).abs() < 0.001);
    }
}

// ============================================================
// Section 16: 境界・エッジケース
// ============================================================

#[test]
fn edge_single_char_same() {
    assert_similarity_range("カ", "カ", 1.0, 1.0);
}

#[test]
fn edge_single_char_voicing() {
    assert_similarity_range("カ", "ガ", 0.6, 1.0);
}

#[test]
fn edge_empty_vs_nonempty() {
    assert_similarity_range("", "カルビ", 0.0, 0.01);
}

#[test]
fn edge_only_sokuon() {
    // ッ alone normalizes to empty string
    assert_similarity_range("ッ", "ッ", 1.0, 1.0);
}

#[test]
fn edge_long_vowel_chain() {
    // コーヒー vs コーヒー (同一)
    assert_similarity_range("コーヒー", "コーヒー", 1.0, 1.0);
}

#[test]
fn edge_all_vowels() {
    assert_similarity_range("アイウエオ", "アイウエオ", 1.0, 1.0);
}

#[test]
fn edge_n_moraic_nasal() {
    // ン vs ン
    assert_similarity_range("ン", "ン", 1.0, 1.0);
}

// ============================================================
// Section 17: 複合的な音韻変化 (複数の要因が重なるケース)
// ============================================================

#[test]
fn combined_voicing_and_vowel() {
    // バケツ vs パケヅ (b/p + ts/z)
    assert_similarity_range("バケツ", "パケヅ", 0.7, 1.0);
}

#[test]
fn combined_sokuon_and_voicing() {
    // サッカー vs ザカ (s/z + sokuon + long vowel)
    assert_similarity_range("サッカー", "ザカ", 0.5, 0.9);
}

#[test]
fn combined_long_vowel_and_nasal() {
    // ラーメン vs ラメン (long vowel only)
    assert_similarity_range("ラーメン", "ラメン", 0.7, 1.0);
}

#[test]
fn combined_multiple_changes() {
    // テーブル vs デブル (t/d + long vowel)
    assert_similarity_range("テーブル", "デブル", 0.6, 1.0);
}