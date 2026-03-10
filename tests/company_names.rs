//! 会社名マッチングのテスト
//!
//! STT（音声認識）が会社名を誤認識した場合に、正しい会社名にマッチングできるかを検証する。
//! 入力はSTTがカタカナ読みに変換済みの前提。
//! 漢字入力（例：「軽美」）は形態素解析等で事前にカタカナ変換が必要。

use rapid_phonetic_matcher::{PhoneticMatcher, PrecomputedCandidates};

/// 会社名マスタ（カタカナ読み）
const COMPANY_MASTER: &[&str] = &[
    // 食品・飲料
    "カルビー",
    "カルディー",
    "キリン",
    "サントリー",
    "アサヒ",
    // テック
    "ソニー",
    "パナソニック",
    "シャープ",
    "マイクロソフト",
    "グーグル",
    "アマゾン",
    "アップル",
    "メタ",
    // 自動車
    "トヨタ",
    "ホンダ",
    "ニッサン",
    "ダイハツ",
    "ブリヂストン",
    // 小売・外食
    "スターバックス",
    "マクドナルド",
    "ファミリーマート",
    "セブンイレブン",
    "ローソン",
    "ユニクロ",
    "ニトリ",
    // 家電量販・通信
    "ヤマダデンキ",
    "ビックカメラ",
    "ヨドバシカメラ",
    "ソフトバンク",
    "ドコモ",
    "エーユー",
    // メーカー
    "ダイキン",
];

fn matcher() -> PhoneticMatcher {
    PhoneticMatcher::new()
}

/// STT入力に対して、マスタからTop1が期待する会社名であることを確認する。
fn assert_top1_match(input: &str, expected_company: &str) {
    let m = matcher();
    let results = m.find_top_matches(input, COMPANY_MASTER, 1);
    assert!(
        !results.is_empty(),
        "No results for input: {input}"
    );
    assert_eq!(
        results[0].text, expected_company,
        "Input '{input}': expected top1 = '{expected_company}', got '{}' (score: {:.3})",
        results[0].text, results[0].score
    );
}

/// STT入力に対して、期待する会社名がTop N以内に含まれることを確認する。
fn assert_in_top_n(input: &str, expected_company: &str, n: usize) {
    let m = matcher();
    let results = m.find_top_matches(input, COMPANY_MASTER, n);
    let found = results.iter().any(|r| r.text == expected_company);
    assert!(
        found,
        "Input '{input}': expected '{expected_company}' in top {n}, got: {:?}",
        results.iter().map(|r| format!("{}({:.3})", r.text, r.score)).collect::<Vec<_>>()
    );
}

/// STT入力に対して、Top1のスコアが指定範囲内であることを確認する。
fn assert_top1_score_range(input: &str, expected_company: &str, min: f32, max: f32) {
    let m = matcher();
    let results = m.find_top_matches(input, COMPANY_MASTER, 1);
    assert_eq!(results[0].text, expected_company,
        "Input '{input}': expected top1 = '{expected_company}', got '{}'", results[0].text);
    assert!(
        results[0].score >= min && results[0].score <= max,
        "Input '{input}': expected score in [{min}, {max}], got {:.3}",
        results[0].score
    );
}

// ============================================================
// 有声破裂音混同 (b/d/g) によるSTT誤変換
// ============================================================

#[test]
fn company_karumi_to_calbee() {
    // 「軽美」「刈る美」等 → カルミー: m→b の誤変換
    assert_top1_match("カルミー", "カルビー");
}

#[test]
fn company_karubi_vs_kaldi() {
    // カルビー vs カルディー は近い音だが区別できる
    let m = matcher();
    let score_calbee = m.calculate_similarity("カルビー", "カルビー");
    let score_kaldi = m.calculate_similarity("カルビー", "カルディー");
    assert!(score_calbee > score_kaldi, "完全一致の方がスコアが高いこと");
}

// ============================================================
// 清音・濁音の混同 (k/g, t/d, s/z, b/p)
// ============================================================

#[test]
fn company_toyota_voiced() {
    // トヨタ → ドヨダ (t/d混同)
    assert_top1_match("ドヨダ", "トヨタ");
}

#[test]
fn company_honda_devoiced() {
    // ホンダ → ホンタ (d/t混同)
    assert_top1_match("ホンタ", "ホンダ");
}

#[test]
fn company_nissan_voiced() {
    // ニッサン → ニッザン (s/z混同)
    assert_top1_match("ニッザン", "ニッサン");
}

#[test]
fn company_panasonic_voiced() {
    // パナソニック → バナソニック (p/b混同)
    assert_top1_match("バナソニック", "パナソニック");
}

#[test]
fn company_softbank_devoiced() {
    // ソフトバンク → ゾフトバンク (s/z混同)
    assert_top1_match("ゾフトバンク", "ソフトバンク");
}

#[test]
fn company_docomo_devoiced() {
    // ドコモ → トコモ (d/t混同)
    assert_top1_match("トコモ", "ドコモ");
}

#[test]
fn company_mcdonalds_devoiced() {
    // マクドナルド → マクトナルド (d/t混同)
    assert_top1_match("マクトナルド", "マクドナルド");
}

#[test]
fn company_daikin_voiced() {
    // ダイキン → ダイギン (k/g混同)
    assert_top1_match("ダイギン", "ダイキン");
}

#[test]
fn company_uniqlo_voiced() {
    // ユニクロ → ユニグロ (k/g混同)
    assert_top1_match("ユニグロ", "ユニクロ");
}

#[test]
fn company_nitori_voiced() {
    // ニトリ → ニドリ (t/d混同)
    assert_top1_match("ニドリ", "ニトリ");
}

#[test]
fn company_daihatsu_devoiced() {
    // ダイハツ → タイハツ (d/t混同)
    assert_top1_match("タイハツ", "ダイハツ");
}

// ============================================================
// f/h 混同 (外来語)
// ============================================================

#[test]
fn company_familymart_fh() {
    // ファミリーマート → ハミリーマート (f/h混同)
    assert_top1_match("ハミリーマート", "ファミリーマート");
}

// ============================================================
// 拗音・合拗音の混同 (sh/s, ch/t)
// ============================================================

#[test]
fn company_sharp_palatalized() {
    // シャープ → サープ (sh/s混同)
    assert_top1_match("サープ", "シャープ");
}

// ============================================================
// 長音の有無・揺れ
// ============================================================

#[test]
fn company_sony_no_long_vowel() {
    // ソニー → ソニ (長音省略)
    assert_top1_match("ソニ", "ソニー");
}

#[test]
fn company_suntory_no_long_vowel() {
    // サントリー → サントリ (長音省略)
    assert_top1_match("サントリ", "サントリー");
}

#[test]
fn company_kirin_exact() {
    // キリン (完全一致)
    assert_top1_score_range("キリン", "キリン", 1.0, 1.0);
}

// ============================================================
// 促音の有無
// ============================================================

#[test]
fn company_bridgestone_ji_di() {
    // ブリヂストン → ブリジストン (ヂ/ジ表記揺れ — 音素的に同一)
    assert_top1_score_range("ブリジストン", "ブリヂストン", 0.95, 1.0);
}

#[test]
fn company_starbucks_sokuon_long() {
    // スターバックス → スタアバクス (長音展開+促音省略)
    assert_top1_match("スタアバクス", "スターバックス");
}

// ============================================================
// 鼻音混同 (m/n)
// ============================================================

#[test]
fn company_yamada_nasal() {
    // ヤマダデンキ → ヤナダデンキ (m/n混同)
    assert_top1_match("ヤナダデンキ", "ヤマダデンキ");
}

#[test]
fn company_bic_camera_nasal() {
    // ビックカメラ → ビクカネラ (m/n混同+促音省略)
    assert_top1_match("ビクカネラ", "ビックカメラ");
}

// ============================================================
// g/k 混同 (テック企業)
// ============================================================

#[test]
fn company_google_devoiced() {
    // グーグル → クークル (g/k混同)
    assert_top1_match("クークル", "グーグル");
}

// ============================================================
// s/z 混同 (テック企業)
// ============================================================

#[test]
fn company_amazon_devoiced() {
    // アマゾン → アマソン (z/s混同)
    assert_top1_match("アマソン", "アマゾン");
}

#[test]
fn company_microsoft_voiced() {
    // マイクロソフト → マイクロゾフト (s/z混同)
    assert_top1_match("マイクロゾフト", "マイクロソフト");
}

// ============================================================
// 完全一致のケース (全社名)
// ============================================================

#[test]
fn company_exact_match_all() {
    let m = matcher();
    for &company in COMPANY_MASTER {
        let results = m.find_top_matches(company, COMPANY_MASTER, 1);
        assert_eq!(results[0].text, company, "Exact match failed for {company}");
        assert_eq!(results[0].score, 1.0, "Score should be 1.0 for exact match: {company}");
    }
}

// ============================================================
// 関係ない入力は低スコアであること
// ============================================================

#[test]
fn company_unrelated_input_low_score() {
    let m = matcher();
    let unrelated = vec!["ラーメン", "サッカー", "ピアノ", "チョコレート"];
    for input in unrelated {
        let results = m.find_top_matches(input, COMPANY_MASTER, 1);
        assert!(
            results[0].score < 0.6,
            "Unrelated input '{input}' matched '{}' with unexpectedly high score: {:.3}",
            results[0].text, results[0].score
        );
    }
}

// ============================================================
// Top Nの順序が妥当であること
// ============================================================

#[test]
fn company_ranking_calbee_kaldi() {
    // 「カルビー」入力: カルビー(完全一致) > カルディー(b/d違い)
    let m = matcher();
    let results = m.find_top_matches("カルビー", COMPANY_MASTER, 3);
    assert_eq!(results[0].text, "カルビー");
    assert_in_top_n("カルビー", "カルディー", 3);
    assert!(results[0].score > results[1].score);
}

#[test]
fn company_ranking_similar_cameras() {
    // 「カメラ」に似た候補: ビックカメラ vs ヨドバシカメラ の両方がTop圏内
    assert_in_top_n("ビクカメラ", "ビックカメラ", 1);
    assert_in_top_n("ヨドバシカメラ", "ヨドバシカメラ", 1);
}

// ============================================================
// PrecomputedCandidatesでも同じ結果になること
// ============================================================

#[test]
fn company_precomputed_consistency() {
    let m = matcher();
    let precomputed = PrecomputedCandidates::new(COMPANY_MASTER);

    let test_inputs = vec!["カルミー", "ドヨダ", "バナソニック", "クークル", "アマソン"];
    for input in test_inputs {
        let normal = m.find_top_matches(input, COMPANY_MASTER, 3);
        let pre = m.find_top_matches_precomputed(input, &precomputed, 3);

        for (a, b) in normal.iter().zip(pre.iter()) {
            assert_eq!(a.text, b.text,
                "Mismatch for input '{input}': normal='{}' vs precomputed='{}'", a.text, b.text);
            assert!((a.score - b.score).abs() < 0.001);
        }
    }
}

// ============================================================
// 複数の音韻変化が重なるケース
// ============================================================

#[test]
fn company_combined_softbank() {
    // ゾフドバンク (s/z + t/d 二重混同)
    assert_top1_match("ゾフドバンク", "ソフトバンク");
}

#[test]
fn company_combined_panasonic() {
    // バナゾニク (p/b + s/z + 促音省略)
    assert_in_top_n("バナゾニク", "パナソニック", 2);
}

#[test]
fn company_combined_seven_eleven() {
    // ゼブンイレブン (s/z混同)
    assert_top1_match("ゼブンイレブン", "セブンイレブン");
}

#[test]
fn company_combined_lawson() {
    // ロウソン (母音の揺れ ー→ウ)
    assert_top1_match("ロウソン", "ローソン");
}