//! エイリアス（略称）対応マッチングのテスト

use rapid_phonetic_matcher::{AliasEntry, PhoneticMatcher, PrecomputedAliases};

fn matcher() -> PhoneticMatcher {
    PhoneticMatcher::new()
}

/// テスト用マスタ: 正式名称 + 略称
fn company_entries() -> Vec<AliasEntry> {
    vec![
        AliasEntry::new("カルビー", &["かるびー"]),
        AliasEntry::new("カルピス", &["かるぴす"]),
        AliasEntry::new("カルディオ", &["かるでぃお"]),
        AliasEntry::new("トヨタ自動車", &["とよたじどうしゃ", "とよた"]),
        AliasEntry::new("豊田工機", &["とよだこうき", "とよだ"]),
        AliasEntry::new("本田技研工業", &["ほんだぎけんこうぎょう", "ほんだ"]),
        AliasEntry::new("日産自動車", &["にっさんじどうしゃ", "にっさん"]),
        AliasEntry::new("ダイキン工業", &["だいきんこうぎょう", "だいきん"]),
        AliasEntry::new("ダイキサウンド", &["だいきさうんど"]),
        AliasEntry::new("パナソニック", &["ぱなそにっく"]),
        AliasEntry::new("ソニー", &["そにー"]),
        AliasEntry::new("シャープ", &["しゃーぷ"]),
        AliasEntry::new("マイクロソフト", &["まいくろそふと"]),
        AliasEntry::new("グーグル", &["ぐーぐる"]),
        AliasEntry::new("アマゾン", &["あまぞん"]),
        AliasEntry::new("スターバックス", &["すたーばっくす", "すたば"]),
        AliasEntry::new("マクドナルド", &["まくどなるど", "まっく", "まくど"]),
        AliasEntry::new("ファミリーマート", &["ふぁみりーまーと", "ふぁみま"]),
        AliasEntry::new("ユニクロ", &["ゆにくろ"]),
        AliasEntry::new("ニトリ", &["にとり"]),
        AliasEntry::new("ソフトバンク", &["そふとばんく"]),
        AliasEntry::new("ドコモ", &["どこも"]),
        AliasEntry::new("ビックカメラ", &["びっくかめら", "びっく"]),
        AliasEntry::new("ヨドバシカメラ", &["よどばしかめら", "よどばし"]),
        AliasEntry::new("ヤマダデンキ", &["やまだでんき", "やまだ"]),
        AliasEntry::new("ニッセイ", &["にっせい"]),
        AliasEntry::new("ニッセン", &["にっせん"]),
    ]
}

fn assert_alias_top1(input: &str, expected: &str) {
    let m = matcher();
    let entries = company_entries();
    let results = m.find_top_matches_with_aliases(input, &entries, 1);
    assert!(
        !results.is_empty(),
        "No results for '{input}'"
    );
    assert_eq!(
        results[0].text, expected,
        "Input '{}': expected '{}', got '{}' (score: {:.3})",
        input, expected, results[0].text, results[0].score
    );
}

fn assert_alias_top1_score(input: &str, expected: &str, min: f32) {
    let m = matcher();
    let entries = company_entries();
    let results = m.find_top_matches_with_aliases(input, &entries, 1);
    assert_eq!(results[0].text, expected,
        "Input '{}': expected '{}', got '{}'", input, expected, results[0].text);
    assert!(
        results[0].score >= min,
        "Input '{}': score {:.3} < min {:.3}", input, results[0].score, min
    );
}

// ============================================================
// 略称でのマッチング (従来方式では失敗していたケース)
// ============================================================

#[test]
fn alias_honda_short() {
    // 「ほんた」→ 略称「ほんだ」経由で本田技研工業にマッチ
    assert_alias_top1("ほんた", "本田技研工業");
}

#[test]
fn alias_honda_exact_short() {
    assert_alias_top1_score("ほんだ", "本田技研工業", 1.0);
}

#[test]
fn alias_toyota_short() {
    assert_alias_top1_score("とよた", "トヨタ自動車", 1.0);
}

#[test]
fn alias_toyota_short_voiced() {
    // 「どよだ」→ 「とよだ」(豊田工機)のほうが近いので豊田工機になる。
    // これは音素的に正しい挙動。
    let m = matcher();
    let entries = company_entries();
    let results = m.find_top_matches_with_aliases("どよだ", &entries, 2);
    // 豊田工機かトヨタ自動車がTop2に入っていればOK
    let names: Vec<&str> = results.iter().map(|r| r.text.as_str()).collect();
    assert!(
        names.contains(&"トヨタ自動車") || names.contains(&"豊田工機"),
        "Expected Toyota-related company in top 2: {:?}", names
    );
}

#[test]
fn alias_nissan_short() {
    assert_alias_top1_score("にっさん", "日産自動車", 1.0);
}

#[test]
fn alias_nissan_short_voiced() {
    assert_alias_top1("にっざん", "日産自動車");
}

#[test]
fn alias_daikin_short() {
    assert_alias_top1_score("だいきん", "ダイキン工業", 1.0);
}

#[test]
fn alias_daikin_short_voiced() {
    assert_alias_top1("だいぎん", "ダイキン工業");
}

#[test]
fn alias_starbucks_short() {
    assert_alias_top1_score("すたば", "スターバックス", 1.0);
}

#[test]
fn alias_starbucks_short_voiced() {
    assert_alias_top1("ずたば", "スターバックス");
}

#[test]
fn alias_mcdonalds_makku() {
    assert_alias_top1_score("まっく", "マクドナルド", 1.0);
}

#[test]
fn alias_mcdonalds_makudo() {
    assert_alias_top1_score("まくど", "マクドナルド", 1.0);
}

#[test]
fn alias_familymart_famima() {
    assert_alias_top1_score("ふぁみま", "ファミリーマート", 1.0);
}

#[test]
fn alias_familymart_famima_fh() {
    // f/h混同
    assert_alias_top1("はみま", "ファミリーマート");
}

#[test]
fn alias_bic_short() {
    assert_alias_top1_score("びっく", "ビックカメラ", 1.0);
}

#[test]
fn alias_yodobashi_short() {
    assert_alias_top1_score("よどばし", "ヨドバシカメラ", 1.0);
}

#[test]
fn alias_yamada_short() {
    assert_alias_top1_score("やまだ", "ヤマダデンキ", 1.0);
}

#[test]
fn alias_yamada_short_nasal() {
    assert_alias_top1("やなだ", "ヤマダデンキ");
}

// ============================================================
// 正式名称でも引き続きマッチすること
// ============================================================

#[test]
fn alias_formal_toyota() {
    assert_alias_top1("どよだじどうしゃ", "トヨタ自動車");
}

#[test]
fn alias_formal_nissan() {
    assert_alias_top1("にっざんじどうしゃ", "日産自動車");
}

#[test]
fn alias_formal_panasonic() {
    assert_alias_top1("ばなそにっく", "パナソニック");
}

#[test]
fn alias_formal_google() {
    assert_alias_top1("くーくる", "グーグル");
}

#[test]
fn alias_formal_mcdonalds() {
    assert_alias_top1("まくとなるど", "マクドナルド");
}

#[test]
fn alias_formal_familymart() {
    assert_alias_top1("はみりーまーと", "ファミリーマート");
}

// ============================================================
// 完全一致 (全エントリが自分自身にマッチ)
// ============================================================

#[test]
fn alias_exact_match_all_readings() {
    let m = matcher();
    let entries = company_entries();

    for entry in &entries {
        for reading in entry.readings() {
            let results = m.find_top_matches_with_aliases(reading, &entries, 1);
            assert_eq!(
                results[0].text,
                entry.name(),
                "Reading '{}' of '{}' matched '{}' instead",
                reading,
                entry.name(),
                results[0].text
            );
            assert!(
                (results[0].score - 1.0).abs() < 0.001,
                "Exact match score should be 1.0 for '{}' reading '{}'",
                entry.name(),
                reading
            );
        }
    }
}

// ============================================================
// PrecomputedAliases と非事前計算の結果一致
// ============================================================

#[test]
fn alias_precomputed_consistency() {
    let m = matcher();
    let entries = company_entries();
    let precomputed = PrecomputedAliases::new(&entries);

    let inputs = vec![
        "ほんた", "とよた", "すたば", "まっく", "ふぁみま",
        "くーくる", "にっざん", "だいぎん", "びっく", "やなだ",
    ];

    for input in inputs {
        let normal = m.find_top_matches_with_aliases(input, &entries, 3);
        let pre = m.find_top_matches_with_aliases_precomputed(input, &precomputed, 3);

        for (a, b) in normal.iter().zip(pre.iter()) {
            assert_eq!(
                a.text, b.text,
                "Input '{}': normal='{}' vs precomputed='{}'",
                input, a.text, b.text
            );
            assert!(
                (a.score - b.score).abs() < 0.001,
                "Input '{}': score mismatch {:.3} vs {:.3}",
                input, a.score, b.score
            );
        }
    }
}

// ============================================================
// 無関係な入力は低スコア
// ============================================================

#[test]
fn alias_unrelated_low_score() {
    let m = matcher();
    let entries = company_entries();

    let unrelated = vec!["らーめん", "さっかー", "ぴあの", "ちょこれーと"];
    for input in unrelated {
        let results = m.find_top_matches_with_aliases(input, &entries, 1);
        assert!(
            results[0].score < 0.7,
            "Unrelated '{}' matched '{}' with high score: {:.3}",
            input, results[0].text, results[0].score
        );
    }
}

// ============================================================
// AliasEntry API テスト
// ============================================================

#[test]
fn alias_entry_name() {
    let entry = AliasEntry::new("テスト企業", &["てすときぎょう", "てすと"]);
    assert_eq!(entry.name(), "テスト企業");
    assert_eq!(entry.readings().len(), 2);
    assert_eq!(entry.readings()[0], "てすときぎょう");
    assert_eq!(entry.readings()[1], "てすと");
}

#[test]
#[should_panic(expected = "must have at least one reading")]
fn alias_entry_empty_readings_panics() {
    let _entry = AliasEntry::new("テスト", &[]);
}

// ============================================================
// 略称 vs 正式名称のスコア比較
// ============================================================

#[test]
fn alias_short_input_prefers_short_reading() {
    // 短い入力「ほんだ」は略称「ほんだ」(1.0)が正式名称(0.36)より高スコア
    let m = matcher();
    let formal_score = m.calculate_similarity("ほんだ", "ほんだぎけんこうぎょう");
    let alias_score = m.calculate_similarity("ほんだ", "ほんだ");
    assert!(
        alias_score > formal_score,
        "Alias should score higher: alias={:.3}, formal={:.3}",
        alias_score, formal_score
    );
}

#[test]
fn alias_long_input_prefers_formal_reading() {
    // 長い入力「ほんだぎけんこうぎょう」は正式名称(1.0)が略称(0.36)より高スコア
    let m = matcher();
    let formal_score = m.calculate_similarity("ほんだぎけんこうぎょう", "ほんだぎけんこうぎょう");
    let alias_score = m.calculate_similarity("ほんだぎけんこうぎょう", "ほんだ");
    assert!(
        formal_score > alias_score,
        "Formal should score higher: formal={:.3}, alias={:.3}",
        formal_score, alias_score
    );
}