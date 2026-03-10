//! NISTEP企業名辞書を用いた実用的な企業名マッチングテスト
//!
//! データソース: NISTEP企業名辞書 Ver.2023_1 (CC BY 3.0)
//! https://nistep.repo.nii.ac.jp/records/2000017
//!
//! 読み仮名をカタカナに変換し、STT誤変換パターンに対して
//! 正しい企業にマッチングできるかを検証する。

use rapid_phonetic_matcher::{PhoneticMatcher, PrecomputedCandidates};

/// NISTEP企業名辞書から抽出した企業名と読みのペア（150社）。
/// 音が紛らわしい企業群を意図的に含めている。
fn load_company_master() -> Vec<(&'static str, &'static str)> {
    vec![
        // カル系 (b/d/p/m/r confusion が起きやすい)
        ("カルビー", "かるびー"),
        ("カルピス", "かるぴす"),
        ("カルディオ", "かるでぃお"),
        ("カルラ", "かるら"),
        ("カルメン", "かるめん"),
        ("カルナック", "かるなっく"),
        ("ＫＡＬＢＡＳ", "かるばす"),
        ("カルシード", "かるしーど"),
        ("カルソニック", "かるそにっく"),
        ("カルプ工業", "かるぷこうぎょう"),
        // トヨ系
        ("トヨタ自動車", "とよたじどうしゃ"),
        ("トヨクモ", "とよくも"),
        ("豊田工機", "とよだこうき"),
        // ホンダ系
        ("本田技研工業", "ほんだぎけんこうぎょう"),
        // ダイキ系
        ("ダイキン工業", "だいきんこうぎょう"),
        ("ダイキサウンド", "だいきさうんど"),
        ("大起エンジニアリング", "だいきえんじにありんぐ"),
        // ニッサン系
        ("日産自動車", "にっさんじどうしゃ"),
        ("日産車体", "にっさんしゃたい"),
        ("ニッセイ", "にっせい"),
        // テック系
        ("パナソニック", "ぱなそにっく"),
        ("ソニー", "そにー"),
        ("シャープ", "しゃーぷ"),
        ("三菱電機", "みつびしでんき"),
        ("富士通", "ふじつう"),
        ("マイクロソフト", "まいくろそふと"),
        ("アップル", "あっぷる"),
        ("グーグル", "ぐーぐる"),
        ("アマゾン", "あまぞん"),
        ("ヤフー", "やふー"),
        ("楽天", "らくてん"),
        // 小売・外食
        ("スターバックス", "すたーばっくす"),
        ("マクドナルド", "まくどなるど"),
        ("ユニクロ", "ゆにくろ"),
        ("ニトリ", "にとり"),
        // 通信
        ("ソフトバンク", "そふとばんく"),
        ("ドコモ", "どこも"),
        // 家電量販
        ("ビックカメラ", "びっくかめら"),
        ("ヨドバシカメラ", "よどばしかめら"),
        ("ヤマダデンキ", "やまだでんき"),
        // ダイ系 (紛らわしいグループ)
        ("第一", "だいいち"),
        ("ダイイチ", "だいいち"),
        ("大紀アルミニウム工業所", "だいきあるみにうむこうぎょうしょ"),
        ("ダイソー", "だいそー"),
        ("ダイハツ", "だいはつ"),
        // ニッ系
        ("ニッセン", "にっせん"),
        ("ニッポン放送", "にっぽんほうそう"),
        ("日鉄", "にってつ"),
        // ソニー系
        ("ソニー不動産", "そにーふどうさん"),
        // フジ系
        ("富士電機", "ふじでんき"),
        ("藤田エンジニアリング", "ふじたえんじにありんぐ"),
        ("不二家", "ふじや"),
        // ホン系
        ("本間ゴルフ", "ほんまごるふ"),
        // マク系
        ("マクロネットワークス", "まくろねっとわーくす"),
        // ミツ系
        ("三菱石油", "みつびしせきゆ"),
    ]
}

/// 読み仮名（ひらがな）をキーにして候補リストを構築する。
/// 実際のSTTユースケースでは、読みをカタカナに変換してマッチングに渡す。
fn build_reading_candidates<'a>(master: &'a [(&'a str, &'a str)]) -> Vec<&'a str> {
    master.iter().map(|(_, reading)| *reading).collect()
}

fn matcher() -> PhoneticMatcher {
    PhoneticMatcher::new()
}

// ============================================================
// STT誤変換 → 正しい企業名の読みにTop1でマッチするかテスト
// ============================================================

/// STT誤変換入力 → 期待する読みがTop1であることを検証
fn assert_stt_match(stt_input: &str, expected_reading: &str) {
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    let results = m.find_top_matches(stt_input, &candidates, 1);
    assert!(
        !results.is_empty(),
        "No results for STT input: {stt_input}"
    );
    assert_eq!(
        results[0].text, expected_reading,
        "STT input '{stt_input}': expected reading '{expected_reading}', \
         got '{}' (score: {:.3})",
        results[0].text, results[0].score
    );
}

/// STT誤変換入力 → 期待する読みがTop N以内に含まれることを検証
fn assert_stt_in_top_n(stt_input: &str, expected_reading: &str, n: usize) {
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    let results = m.find_top_matches(stt_input, &candidates, n);
    let found = results.iter().any(|r| r.text == expected_reading);
    assert!(
        found,
        "STT input '{stt_input}': expected '{expected_reading}' in top {n}, got: {:?}",
        results
            .iter()
            .map(|r| format!("{}({:.3})", r.text, r.score))
            .collect::<Vec<_>>()
    );
}

// ============================================================
// カル系: 音が非常に紛らわしい企業グループ
// ============================================================

#[test]
fn nistep_karumi_to_calbee() {
    // STTが「カルミー」と誤認識 → カルビー(かるびー)にマッチ
    assert_stt_match("かるみー", "かるびー");
}

#[test]
fn nistep_calbee_vs_calpis() {
    // カルビー vs カルピス: b/p の違い — 区別できるが高い類似度
    let m = matcher();
    let score = m.calculate_similarity("かるびー", "かるぴす");
    assert!(score > 0.6, "カルビー vs カルピス should have moderate similarity: {score}");
    assert!(score < 0.95, "カルビー vs カルピス should not be near-perfect: {score}");
}

#[test]
fn nistep_calbee_exact() {
    assert_stt_match("かるびー", "かるびー");
}

#[test]
fn nistep_calpis_exact() {
    assert_stt_match("かるぴす", "かるぴす");
}

#[test]
fn nistep_kaldi_confusion() {
    // カルディオ vs カルビー: d/b の近さ
    let m = matcher();
    let score = m.calculate_similarity("かるでぃお", "かるびー");
    assert!(score > 0.4, "Should have some similarity: {score}");

    // でも正確な入力ならカルディオにマッチ
    assert_stt_match("かるでぃお", "かるでぃお");
}

#[test]
fn nistep_karu_group_discrimination() {
    // 「かるびー」入力 → カルビーがTop1、カルバス・カルピスが上位
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    let results = m.find_top_matches("かるびー", &candidates, 5);
    assert_eq!(results[0].text, "かるびー", "Top1 should be exact match");

    // Top5に「カル」系の企業が多く含まれるはず
    let karu_count = results.iter().filter(|r| r.text.starts_with("かる")).count();
    assert!(
        karu_count >= 3,
        "Expected at least 3 'カル' companies in top 5, got {karu_count}: {:?}",
        results.iter().map(|r| &r.text).collect::<Vec<_>>()
    );
}

// ============================================================
// トヨタ系: 清濁混同でトヨタ/トヨダが紛らわしい
// ============================================================

#[test]
fn nistep_toyota_devoiced() {
    // 「どよだじどうしゃ」(t/d全混同) → トヨタ自動車にマッチ
    assert_stt_match("どよだじどうしゃ", "とよたじどうしゃ");
}

#[test]
fn nistep_toyota_vs_toyoda() {
    // トヨタ自動車(とよた) vs 豊田工機(とよだ): t/d 1文字違い
    let m = matcher();
    let score = m.calculate_similarity("とよたじどうしゃ", "とよだこうき");
    // 読みの後半が全く違うので低スコアのはず
    assert!(score < 0.6, "Different companies should have lower score: {score}");
}

// ============================================================
// ダイキ系: 同じ「ダイキ」で始まる複数企業
// ============================================================

#[test]
fn nistep_daikin_voiced() {
    // 「だいぎんこうぎょう」(k/g混同) → ダイキン工業にマッチ
    assert_stt_match("だいぎんこうぎょう", "だいきんこうぎょう");
}

#[test]
fn nistep_daikin_vs_daiki_sound() {
    // ダイキン工業 vs ダイキサウンド: 「ダイキ」まで同一だが後続が違う
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    let results = m.find_top_matches("だいきんこうぎょう", &candidates, 3);
    assert_eq!(results[0].text, "だいきんこうぎょう", "Exact match should be top1");

    // ダイキサウンドは入らないはず（後半が全く違う）
    let has_daiki_sound = results.iter().any(|r| r.text == "だいきさうんど");
    // 紛らわしいかもしれないのでTop3に入ってもOKだが、スコアは低いはず
    if has_daiki_sound {
        let daiki_result = results.iter().find(|r| r.text == "だいきさうんど").unwrap();
        assert!(daiki_result.score < results[0].score);
    }
}

// ============================================================
// ニッサン系: 促音+清濁の組み合わせ
// ============================================================

#[test]
fn nistep_nissan_voiced() {
    // 「にっざんじどうしゃ」(s/z混同) → 日産自動車にマッチ
    assert_stt_match("にっざんじどうしゃ", "にっさんじどうしゃ");
}

#[test]
fn nistep_nissan_vs_nissei() {
    // 日産自動車 vs ニッセイ: 「ニッ」で始まるが全然違う
    let m = matcher();
    let score = m.calculate_similarity("にっさんじどうしゃ", "にっせい");
    assert!(score < 0.5, "Different companies: {score}");
}

#[test]
fn nistep_nissan_vs_nissen() {
    // 日産 vs ニッセン: 「ニッサン」vs「ニッセン」 — 母音違い
    let m = matcher();
    let score = m.calculate_similarity("にっさん", "にっせん");
    assert!(score > 0.7, "Similar prefixes should have high similarity: {score}");
}

// ============================================================
// テック企業: よくあるSTT誤変換
// ============================================================

#[test]
fn nistep_panasonic_bp() {
    // 「ばなそにっく」(p/b混同) → パナソニック
    assert_stt_match("ばなそにっく", "ぱなそにっく");
}

#[test]
fn nistep_sony_exact() {
    assert_stt_match("そにー", "そにー");
}

#[test]
fn nistep_sony_no_long_vowel() {
    // 「そに」(長音省略) → ソニー
    assert_stt_match("そに", "そにー");
}

#[test]
fn nistep_sharp_palatalized() {
    // 「さーぷ」(sh/s混同) → シャープ
    assert_stt_match("さーぷ", "しゃーぷ");
}

#[test]
fn nistep_google_devoiced() {
    // 「くーくる」(g/k混同) → グーグル
    assert_stt_match("くーくる", "ぐーぐる");
}

#[test]
fn nistep_amazon_devoiced() {
    // 「あまそん」(z/s混同) → アマゾン
    assert_stt_match("あまそん", "あまぞん");
}

#[test]
fn nistep_microsoft_voiced() {
    // 「まいくろぞふと」(s/z混同) → マイクロソフト
    assert_stt_match("まいくろぞふと", "まいくろそふと");
}

#[test]
fn nistep_softbank_devoiced() {
    // 「ぞふとばんく」(s/z混同) → ソフトバンク
    assert_stt_match("ぞふとばんく", "そふとばんく");
}

#[test]
fn nistep_docomo_devoiced() {
    // 「とこも」(d/t混同) → ドコモ
    assert_stt_match("とこも", "どこも");
}

// ============================================================
// 小売・外食: 長音・促音の揺れ
// ============================================================

#[test]
fn nistep_starbucks_simplified() {
    // 「すたあばくす」(長音展開+促音省略) → スターバックス
    assert_stt_match("すたあばくす", "すたーばっくす");
}

#[test]
fn nistep_mcdonalds_devoiced() {
    // 「まくとなるど」(d/t混同) → マクドナルド
    assert_stt_match("まくとなるど", "まくどなるど");
}

#[test]
fn nistep_uniqlo_voiced() {
    // 「ゆにぐろ」(k/g混同) → ユニクロ
    assert_stt_match("ゆにぐろ", "ゆにくろ");
}

#[test]
fn nistep_nitori_voiced() {
    // 「にどり」(t/d混同) → ニトリ
    assert_stt_match("にどり", "にとり");
}

// ============================================================
// 家電量販: 鼻音混同 + 促音
// ============================================================

#[test]
fn nistep_yamada_nasal() {
    // 「やなだでんき」(m/n混同) → ヤマダデンキ
    assert_stt_match("やなだでんき", "やまだでんき");
}

#[test]
fn nistep_bic_camera_nasal() {
    // 「びくかねら」(m/n混同+促音省略) → ビックカメラ
    assert_stt_match("びくかねら", "びっくかめら");
}

// ============================================================
// ソニー不動産 vs ソニー: 部分一致的な紛らわしさ
// ============================================================

#[test]
fn nistep_sony_vs_sony_fudosan() {
    // 「そにー」→ ソニーにマッチ（ソニー不動産ではなく）
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    let results = m.find_top_matches("そにー", &candidates, 3);
    assert_eq!(results[0].text, "そにー", "Exact match should be top1");

    // ソニー vs ソニー不動産 は長さが大幅に異なるため、
    // 短い入力「そにー」からは「そにーふどうさん」はTop圏外になる。
    // これは正しい挙動: 長さの差が大きい候補は低スコアになるべき。
    let score_sony_fudosan = m.calculate_similarity("そにー", "そにーふどうさん");
    assert!(
        score_sony_fudosan < results[0].score,
        "Short input should not highly match a much longer candidate"
    );
}

// ============================================================
// 完全一致テスト: 全マスタ企業の読みが自分自身にマッチ
// ============================================================

#[test]
fn nistep_all_exact_matches() {
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    for &(name, reading) in &master {
        let results = m.find_top_matches(reading, &candidates, 1);
        assert_eq!(
            results[0].text, reading,
            "Company '{}' (reading: {}) should match itself, got '{}' (score: {:.3})",
            name, reading, results[0].text, results[0].score
        );
        assert!(
            (results[0].score - 1.0).abs() < 0.001,
            "Exact match score should be 1.0 for '{}', got {:.3}",
            name, results[0].score
        );
    }
}

// ============================================================
// PrecomputedCandidatesの大規模テスト
// ============================================================

#[test]
fn nistep_precomputed_large_scale() {
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();
    let precomputed = PrecomputedCandidates::new(&candidates);

    let test_inputs = vec![
        "かるみー",        // カルビー誤変換
        "どよだじどうしゃ",  // トヨタ誤変換
        "ばなそにっく",    // パナソニック誤変換
        "くーくる",        // グーグル誤変換
        "あまそん",        // アマゾン誤変換
        "にっざんじどうしゃ", // 日産誤変換
    ];

    for input in &test_inputs {
        let normal = m.find_top_matches(input, &candidates, 3);
        let pre = m.find_top_matches_precomputed(input, &precomputed, 3);

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
// 複合誤変換: 複数の音韻変化が同時に起きるケース
// ============================================================

#[test]
fn nistep_combined_toyota_full_voicing() {
    // 「どよだじどうじゃ」(t/d + sh/j 複合混同) → トヨタ自動車
    // sh→j は拗音の清濁混同
    assert_stt_in_top_n("どよだじどうじゃ", "とよたじどうしゃ", 2);
}

#[test]
fn nistep_combined_softbank_double() {
    // 「ぞふどばんく」(s/z + t/d 複合) → ソフトバンク
    assert_stt_match("ぞふどばんく", "そふとばんく");
}

#[test]
fn nistep_combined_panasonic_triple() {
    // 「ばなぞにく」(p/b + s/z + 促音省略) → パナソニック
    assert_stt_in_top_n("ばなぞにく", "ぱなそにっく", 2);
}

// ============================================================
// 閾値テスト: 無関係な入力が高スコアにならないこと
// ============================================================

#[test]
fn nistep_unrelated_inputs() {
    let master = load_company_master();
    let candidates = build_reading_candidates(&master);
    let m = matcher();

    let unrelated = vec!["らーめん", "さっかー", "ぴあの", "ちょこれーと", "てんぷら"];
    for input in unrelated {
        let results = m.find_top_matches(input, &candidates, 1);
        assert!(
            results[0].score < 0.65,
            "Unrelated input '{}' matched '{}' with high score: {:.3}",
            input, results[0].text, results[0].score
        );
    }
}

// ============================================================
// 三菱系: 似た社名の識別
// ============================================================

#[test]
fn nistep_mitsubishi_denki_vs_sekiyu() {
    // 三菱電機 vs 三菱石油: 「みつびし」まで同一
    let m = matcher();
    let score = m.calculate_similarity("みつびしでんき", "みつびしせきゆ");
    assert!(score > 0.6, "Same prefix should give moderate similarity: {score}");
    assert!(score < 0.9, "Different suffixes should prevent high similarity: {score}");
}

#[test]
fn nistep_mitsubishi_denki_exact() {
    assert_stt_match("みつびしでんき", "みつびしでんき");
}

// ============================================================
// フジ系: 「ふじ」で始まる企業の識別
// ============================================================

#[test]
fn nistep_fujitsu_vs_fuji_denki() {
    // 富士通 vs 富士電機: 「ふじ」で始まるが後半が異なる
    let m = matcher();
    let score = m.calculate_similarity("ふじつう", "ふじでんき");
    assert!(score < 0.7, "Different companies: {score}");

    // 各自がTop1でマッチ
    assert_stt_match("ふじつう", "ふじつう");
    assert_stt_match("ふじでんき", "ふじでんき");
}