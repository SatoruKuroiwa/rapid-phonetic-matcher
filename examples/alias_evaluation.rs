use rapid_phonetic_matcher::PhoneticMatcher;

/// 企業マスタ: 企業ID → (表示名, 読みのリスト[正式名称, 略称, ...])
fn build_master() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        // (企業表示名, [正式読み, 略称読み, ...])
        ("カルビー", vec!["かるびー"]),
        ("カルピス", vec!["かるぴす"]),
        ("カルディオ", vec!["かるでぃお"]),
        ("カルラ", vec!["かるら"]),
        ("カルメン", vec!["かるめん"]),
        ("カルナック", vec!["かるなっく"]),
        ("ＫＡＬＢＡＳ", vec!["かるばす"]),
        ("カルシード", vec!["かるしーど"]),
        ("カルソニック", vec!["かるそにっく"]),
        // 正式名称 + 略称
        ("トヨタ自動車", vec!["とよたじどうしゃ", "とよた"]),
        ("トヨクモ", vec!["とよくも"]),
        ("豊田工機", vec!["とよだこうき", "とよだ"]),
        ("本田技研工業", vec!["ほんだぎけんこうぎょう", "ほんだ"]),
        ("ダイキン工業", vec!["だいきんこうぎょう", "だいきん"]),
        ("ダイキサウンド", vec!["だいきさうんど"]),
        ("大起エンジニアリング", vec!["だいきえんじにありんぐ"]),
        ("日産自動車", vec!["にっさんじどうしゃ", "にっさん"]),
        ("日産車体", vec!["にっさんしゃたい"]),
        ("ニッセイ", vec!["にっせい"]),
        ("パナソニック", vec!["ぱなそにっく"]),
        ("ソニー", vec!["そにー"]),
        ("シャープ", vec!["しゃーぷ"]),
        ("三菱電機", vec!["みつびしでんき"]),
        ("富士通", vec!["ふじつう"]),
        ("マイクロソフト", vec!["まいくろそふと"]),
        ("アップル", vec!["あっぷる"]),
        ("グーグル", vec!["ぐーぐる"]),
        ("アマゾン", vec!["あまぞん"]),
        ("ヤフー", vec!["やふー"]),
        ("楽天", vec!["らくてん"]),
        ("スターバックス", vec!["すたーばっくす", "すたば"]),
        ("マクドナルド", vec!["まくどなるど", "まっく", "まくど"]),
        ("ファミリーマート", vec!["ふぁみりーまーと", "ふぁみま"]),
        ("ユニクロ", vec!["ゆにくろ"]),
        ("ニトリ", vec!["にとり"]),
        ("ソフトバンク", vec!["そふとばんく"]),
        ("ドコモ", vec!["どこも"]),
        ("ビックカメラ", vec!["びっくかめら", "びっく"]),
        ("ヨドバシカメラ", vec!["よどばしかめら", "よどばし"]),
        ("ヤマダデンキ", vec!["やまだでんき", "やまだ"]),
        ("ダイソー", vec!["だいそー"]),
        ("ダイハツ", vec!["だいはつ"]),
        ("ニッセン", vec!["にっせん"]),
        ("日鉄", vec!["にってつ"]),
        ("ソニー不動産", vec!["そにーふどうさん"]),
        ("富士電機", vec!["ふじでんき"]),
        ("不二家", vec!["ふじや"]),
        ("三菱石油", vec!["みつびしせきゆ"]),
    ]
}

/// 略称なし (従来方式): 正式名称の読みだけでマッチング
fn match_without_aliases(m: &PhoneticMatcher, input: &str, master: &[(&str, Vec<&str>)]) -> (String, f32) {
    let mut best_name = String::new();
    let mut best_score = -1.0_f32;

    for (name, readings) in master {
        // 正式名称(最初のエントリ)のみ使用
        let score = m.calculate_similarity(input, readings[0]);
        if score > best_score {
            best_score = score;
            best_name = name.to_string();
        }
    }
    (best_name, best_score)
}

/// 略称あり (新方式): 全ての読み(正式名称+略称)の中で最高スコアを採用
fn match_with_aliases(m: &PhoneticMatcher, input: &str, master: &[(&str, Vec<&str>)]) -> (String, f32) {
    let mut best_name = String::new();
    let mut best_score = -1.0_f32;

    for (name, readings) in master {
        for &reading in readings {
            let score = m.calculate_similarity(input, reading);
            if score > best_score {
                best_score = score;
                best_name = name.to_string();
            }
        }
    }
    (best_name, best_score)
}

fn main() {
    let m = PhoneticMatcher::new();
    let master = build_master();

    // テストケース: (STT入力, 期待する企業名, 説明)
    let test_cases: Vec<(&str, &str, &str)> = vec![
        // === 前回失敗したケース ===
        ("ほんた", "本田技研工業", "短縮入力 d/t混同"),
        ("ほんだ", "本田技研工業", "略称そのまま"),

        // === 略称が効くケース ===
        ("とよた", "トヨタ自動車", "略称そのまま"),
        ("どよだ", "トヨタ自動車", "略称+t/d混同"),
        ("にっさん", "日産自動車", "略称そのまま"),
        ("にっざん", "日産自動車", "略称+s/z混同"),
        ("だいきん", "ダイキン工業", "略称そのまま"),
        ("だいぎん", "ダイキン工業", "略称+k/g混同"),
        ("すたば", "スターバックス", "略称そのまま"),
        ("ずたば", "スターバックス", "略称+s/z混同"),
        ("まっく", "マクドナルド", "略称(マック)"),
        ("まくど", "マクドナルド", "略称(マクド)"),
        ("ふぁみま", "ファミリーマート", "略称(ファミマ)"),
        ("はみま", "ファミリーマート", "略称+f/h混同"),
        ("びっく", "ビックカメラ", "略称"),
        ("よどばし", "ヨドバシカメラ", "略称"),
        ("やまだ", "ヤマダデンキ", "略称"),
        ("やなだ", "ヤマダデンキ", "略称+m/n混同"),

        // === 正式名称でもマッチするケース ===
        ("どよだじどうしゃ", "トヨタ自動車", "正式名称 t/d混同"),
        ("にっざんじどうしゃ", "日産自動車", "正式名称 s/z混同"),
        ("ばなそにっく", "パナソニック", "正式名称 p/b混同"),
        ("くーくる", "グーグル", "正式名称 g/k混同"),
        ("かるみー", "カルビー", "正式名称 m/b混同"),
        ("まくとなるど", "マクドナルド", "正式名称 d/t混同"),
        ("はみりーまーと", "ファミリーマート", "正式名称 f/h混同"),

        // === 無関係な入力 ===
        ("らーめん", "?", "無関係"),
        ("やきにく", "?", "無関係"),
    ];

    println!("=============================================");
    println!("  略称マスタによる精度向上の検証");
    println!("=============================================\n");

    println!("{:<20} {:<15} | {:<15} {:>6} | {:<15} {:>6} | {}",
        "STT入力", "期待", "従来Top1", "Score", "略称有Top1", "Score", "改善");
    println!("{}", "=".repeat(120));

    let mut old_correct = 0;
    let mut new_correct = 0;
    let mut total_testable = 0;

    for (input, expected, desc) in &test_cases {
        let (old_name, old_score) = match_without_aliases(&m, input, &master);
        let (new_name, new_score) = match_with_aliases(&m, input, &master);

        let is_unrelated = *expected == "?";
        if !is_unrelated {
            total_testable += 1;
            if old_name == *expected { old_correct += 1; }
            if new_name == *expected { new_correct += 1; }
        }

        let improvement = if is_unrelated {
            "---".to_string()
        } else {
            let old_ok = old_name == *expected;
            let new_ok = new_name == *expected;
            match (old_ok, new_ok) {
                (false, true) => "✓ 改善!".to_string(),
                (true, true) if new_score > old_score + 0.01 => format!("↑ +{:.3}", new_score - old_score),
                (true, true) => "= 同等".to_string(),
                (true, false) => "✗ 劣化".to_string(),
                (false, false) => "✗ 両方失敗".to_string(),
            }
        };

        let old_mark = if !is_unrelated && old_name == *expected { "✓" } else { " " };
        let new_mark = if !is_unrelated && new_name == *expected { "✓" } else { " " };

        println!("{:<20} {:<15} |{} {:<14} {:>6.3} |{} {:<14} {:>6.3} | {} [{}]",
            input, expected,
            old_mark, old_name, old_score,
            new_mark, new_name, new_score,
            improvement, desc);
    }

    println!("\n{}", "=".repeat(120));
    println!("\n--- 正解率比較 ---\n");
    println!("従来方式 (正式名称のみ): {}/{} ({:.1}%)",
        old_correct, total_testable, old_correct as f64 / total_testable as f64 * 100.0);
    println!("略称併用方式:           {}/{} ({:.1}%)",
        new_correct, total_testable, new_correct as f64 / total_testable as f64 * 100.0);
    println!("改善:                  +{} 件", new_correct - old_correct);

    // False positive check
    println!("\n--- 略称によるFalse Positive確認 ---\n");
    let unrelated = vec!["らーめん", "さっかー", "ぴあの", "てんぷら", "やきにく", "おすし", "からおけ"];
    for input in &unrelated {
        let (old_name, old_score) = match_without_aliases(&m, input, &master);
        let (new_name, new_score) = match_with_aliases(&m, input, &master);
        let worse = if new_score > old_score + 0.05 { "⚠ スコア上昇" } else { "OK" };
        println!("  {:<12} 従来: {}({:.3})  略称有: {}({:.3})  [{}]",
            input, old_name, old_score, new_name, new_score, worse);
    }
}