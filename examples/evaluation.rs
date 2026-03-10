use rapid_phonetic_matcher::PhoneticMatcher;

fn main() {
    let m = PhoneticMatcher::new();

    // === 評価用データセット ===
    // (STT入力, 正解の読み, 誤変換カテゴリ)
    let stt_cases: Vec<(&str, &str, &str)> = vec![
        // 有声破裂音混同 (b/d/g)
        ("かるみー", "かるびー", "有声破裂音 m/b"),
        ("だぐ", "ばぐ", "有声破裂音 b/d"),
        ("だむ", "がむ", "有声破裂音 g/d"),
        ("どぼう", "ごぼう", "有声破裂音 g/d"),
        // 清濁混同 (k/g, t/d, s/z, b/p, h/b)
        ("がき", "かき", "清濁 k/g"),
        ("だい", "たい", "清濁 t/d"),
        ("ざる", "さる", "清濁 s/z"),
        ("ばん", "はん", "清濁 h/b"),
        ("ぱす", "ばす", "清濁 b/p"),
        ("どよだじどうしゃ", "とよたじどうしゃ", "清濁 t/d 複数"),
        ("ほんた", "ほんだ", "清濁 d/t"),
        ("にっざんじどうしゃ", "にっさんじどうしゃ", "清濁 s/z"),
        ("ばなそにっく", "ぱなそにっく", "清濁 p/b"),
        ("ぞふとばんく", "そふとばんく", "清濁 s/z"),
        ("とこも", "どこも", "清濁 d/t"),
        ("まくとなるど", "まくどなるど", "清濁 d/t"),
        ("だいぎんこうぎょう", "だいきんこうぎょう", "清濁 k/g"),
        ("ゆにぐろ", "ゆにくろ", "清濁 k/g"),
        ("にどり", "にとり", "清濁 t/d"),
        ("くーくる", "ぐーぐる", "清濁 g/k 複数"),
        ("あまそん", "あまぞん", "清濁 z/s"),
        ("まいくろぞふと", "まいくろそふと", "清濁 s/z"),
        // 鼻音混同 (m/n)
        ("かに", "かみ", "鼻音 m/n"),
        ("なる", "まる", "鼻音 m/n"),
        ("さんな", "さんま", "鼻音 m/n"),
        ("やなだでんき", "やまだでんき", "鼻音 m/n"),
        // 長音の有無
        ("そに", "そにー", "長音省略"),
        ("さんとり", "さんとりー", "長音省略"),
        ("びる", "びーる", "長音省略"),
        ("める", "めーる", "長音省略"),
        ("かれ", "かれー", "長音省略"),
        // 促音の有無
        ("まち", "まっち", "促音省略"),
        ("ろけと", "ろけっと", "促音省略"),
        ("きぷ", "きっぷ", "促音省略"),
        // 拗音・合拗音混同
        ("さーぷ", "しゃーぷ", "拗音 sh/s"),
        ("たんす", "ちゃんす", "拗音 ch/t"),
        ("ずーす", "じゅーす", "拗音 j/z"),
        // f/h混同
        ("はみりーまーと", "ふぁみりーまーと", "f/h混同"),
        ("へいすぶっく", "ふぇいすぶっく", "f/h混同"),
        ("ひるむ", "ふぃるむ", "f/h混同"),
        // 長音展開+促音省略 (複合)
        ("すたあばくす", "すたーばっくす", "複合:長音+促音"),
        ("びくかねら", "びっくかめら", "複合:鼻音+促音"),
        ("ぞふどばんく", "そふとばんく", "複合:s/z+t/d"),
        // 母音混同
        ("きら", "から", "母音 a/i"),
        ("そし", "すし", "母音 u/o"),
        ("たすと", "てすと", "母音 e/a"),
    ];

    // === 紛らわしい企業マスタ ===
    let confusable_master: Vec<&str> = vec![
        "かるびー", "かるぴす", "かるでぃお", "かるら", "かるめん",
        "かるなっく", "かるばす", "かるしーど", "かるそにっく",
        "とよたじどうしゃ", "とよくも", "とよだこうき",
        "ほんだぎけんこうぎょう",
        "だいきんこうぎょう", "だいきさうんど", "だいきえんじにありんぐ",
        "にっさんじどうしゃ", "にっさんしゃたい", "にっせい",
        "ぱなそにっく", "そにー", "しゃーぷ",
        "みつびしでんき", "ふじつう",
        "まいくろそふと", "あっぷる", "ぐーぐる", "あまぞん",
        "やふー", "らくてん",
        "すたーばっくす", "まくどなるど",
        "ゆにくろ", "にとり",
        "そふとばんく", "どこも",
        "びっくかめら", "よどばしかめら", "やまだでんき",
        "だいいち", "だいそー", "だいはつ",
        "にっせん", "にってつ",
        "そにーふどうさん",
        "ふじでんき", "ふじや",
        "みつびしせきゆ",
    ];

    println!("========================================");
    println!("  音素マッチング精度評価レポート");
    println!("========================================\n");

    // --- 1. 個別スコア分析 ---
    println!("--- 1. STT誤変換に対するスコア分布 ---\n");
    println!("{:<25} {:<25} {:<15} {:>6}", "STT入力", "正解", "カテゴリ", "スコア");
    println!("{}", "-".repeat(75));

    let mut scores_by_category: std::collections::BTreeMap<String, Vec<f32>> = std::collections::BTreeMap::new();
    let mut all_scores = Vec::new();

    for (input, correct, category) in &stt_cases {
        let score = m.calculate_similarity(input, correct);
        println!("{:<25} {:<25} {:<15} {:>6.3}", input, correct, category, score);
        let cat_key = category.split_whitespace().next().unwrap_or(category).to_string();
        scores_by_category.entry(cat_key).or_default().push(score);
        all_scores.push((*category, score));
    }

    // --- 2. カテゴリ別統計 ---
    println!("\n--- 2. カテゴリ別スコア統計 ---\n");
    println!("{:<15} {:>5} {:>8} {:>8} {:>8}", "カテゴリ", "件数", "平均", "最小", "最大");
    println!("{}", "-".repeat(50));

    for (cat, scores) in &scores_by_category {
        let n = scores.len();
        let avg = scores.iter().sum::<f32>() / n as f32;
        let min = scores.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        println!("{:<15} {:>5} {:>8.3} {:>8.3} {:>8.3}", cat, n, avg, min, max);
    }

    // --- 3. 企業マスタに対するTop1正解率 ---
    println!("\n--- 3. 企業マスタに対するTop1正解率 ---\n");

    let company_stt_cases: Vec<(&str, &str, &str)> = vec![
        ("かるみー", "かるびー", "カルビー(m/b)"),
        ("どよだじどうしゃ", "とよたじどうしゃ", "トヨタ(t/d)"),
        ("ほんた", "ほんだぎけんこうぎょう", "ホンダ(d/t) ※読み短縮"),
        ("にっざんじどうしゃ", "にっさんじどうしゃ", "日産(s/z)"),
        ("ばなそにっく", "ぱなそにっく", "パナソニック(p/b)"),
        ("くーくる", "ぐーぐる", "グーグル(g/k)"),
        ("あまそん", "あまぞん", "アマゾン(z/s)"),
        ("まいくろぞふと", "まいくろそふと", "マイクロソフト(s/z)"),
        ("ぞふとばんく", "そふとばんく", "ソフトバンク(s/z)"),
        ("とこも", "どこも", "ドコモ(d/t)"),
        ("まくとなるど", "まくどなるど", "マクドナルド(d/t)"),
        ("はみりーまーと", "ふぁみりーまーと", "ファミリーマート ※マスタ外"),
        ("すたあばくす", "すたーばっくす", "スターバックス(複合)"),
        ("だいぎんこうぎょう", "だいきんこうぎょう", "ダイキン(k/g)"),
        ("ゆにぐろ", "ゆにくろ", "ユニクロ(k/g)"),
        ("にどり", "にとり", "ニトリ(t/d)"),
        ("やなだでんき", "やまだでんき", "ヤマダデンキ(m/n)"),
        ("びくかねら", "びっくかめら", "ビックカメラ(m/n+促音)"),
        ("さーぷ", "しゃーぷ", "シャープ(sh/s)"),
        ("そに", "そにー", "ソニー(長音省略)"),
        ("ぞふどばんく", "そふとばんく", "ソフトバンク(s/z+t/d複合)"),
    ];

    let mut correct_count = 0;
    let mut top3_count = 0;
    let total = company_stt_cases.len();

    println!("{:<25} {:<25} {:<25} {:>6} {}", "STT入力", "期待", "Top1結果", "スコア", "判定");
    println!("{}", "-".repeat(110));

    for (input, expected, desc) in &company_stt_cases {
        let results = m.find_top_matches(input, &confusable_master, 3);
        let top1 = &results[0];
        let is_correct = top1.text == *expected;
        let in_top3 = results.iter().any(|r| r.text == *expected);

        if is_correct { correct_count += 1; }
        if in_top3 { top3_count += 1; }

        let mark = if is_correct { "✓ Top1" } else if in_top3 { "△ Top3" } else { "✗ Miss" };
        println!(
            "{:<25} {:<25} {:<25} {:>6.3} {} [{}]",
            input, expected, top1.text, top1.score, mark, desc
        );
        if !is_correct {
            for (i, r) in results.iter().enumerate().skip(0).take(3) {
                println!("    {}位: {:<20} {:.3}", i + 1, r.text, r.score);
            }
        }
    }

    println!("\n--- 4. 総合評価 ---\n");
    println!("Top1正解率: {}/{} ({:.1}%)", correct_count, total, correct_count as f64 / total as f64 * 100.0);
    println!("Top3正解率: {}/{} ({:.1}%)", top3_count, total, top3_count as f64 / total as f64 * 100.0);

    // --- 5. 弱点分析: 同一入力で誤マッチする競合候補 ---
    println!("\n--- 5. 紛らわしい企業ペア(スコア差が小さいケース) ---\n");
    let ambiguous_inputs = vec![
        ("かるびー", "カルビー入力"),
        ("かるぴす", "カルピス入力"),
        ("だいきんこうぎょう", "ダイキン入力"),
        ("にっさんじどうしゃ", "日産入力"),
    ];
    for (input, desc) in &ambiguous_inputs {
        let results = m.find_top_matches(input, &confusable_master, 5);
        println!("[{}] {}:", desc, input);
        for (i, r) in results.iter().enumerate().take(5) {
            let gap = if i == 0 { "".to_string() } else { format!("(差: {:.3})", results[0].score - r.score) };
            println!("  {}位: {:<25} {:.3} {}", i + 1, r.text, r.score, gap);
        }
        println!();
    }

    // --- 6. False positive分析: 無関係な入力 ---
    println!("--- 6. 無関係な入力に対する最高スコア ---\n");
    let unrelated = vec!["らーめん", "さっかー", "ぴあの", "ちょこれーと", "てんぷら", "おすし", "やきにく"];
    for input in &unrelated {
        let results = m.find_top_matches(input, &confusable_master, 1);
        let status = if results[0].score < 0.5 { "OK(低)" } else if results[0].score < 0.65 { "注意(中)" } else { "問題(高)" };
        println!("  {:<15} → {:<20} {:.3} [{}]", input, results[0].text, results[0].score, status);
    }
}