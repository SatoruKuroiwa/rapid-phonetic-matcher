use rapid_phonetic_matcher::PhoneticMatcher;

fn main() {
    let m = PhoneticMatcher::new();

    println!("=============================================");
    println!("  日本人苗字に対する音素マッチング精度評価");
    println!("=============================================\n");

    // === よくある苗字マスタ(読み仮名, 上位100程度から抜粋) ===
    let surname_master: Vec<&str> = vec![
        "さとう", "すずき", "たかはし", "たなか", "わたなべ",
        "いとう", "やまもと", "なかむら", "こばやし", "かとう",
        "よしだ", "やまだ", "ささき", "やまぐち", "まつもと",
        "いのうえ", "きむら", "はやし", "さいとう", "しみず",
        "やまざき", "もり", "いけだ", "はしもと", "あべ",
        "いしかわ", "やました", "おがわ", "いしい", "おかだ",
        "はせがわ", "まえだ", "ふじた", "おおつか", "ごとう",
        "おかもと", "こんどう", "ふくだ", "なかがわ", "みうら",
        "ふじい", "むらかみ", "すぎやま", "おおの", "くどう",
        "えんどう", "あおき", "さかもと", "ふじわら", "ほり",
        // 紛らわしいペア用の追加
        "さいたま", "しみずたに", "もりた", "もりかわ",
        "たかだ", "たかの", "たかぎ", "たかせ",
        "なかやま", "なかじま", "なかの", "なかい",
        "おおた", "おおにし", "おおもり", "おおはし",
    ];

    // === 1. STT誤認識パターン (苗字特有) ===
    println!("--- 1. STT誤認識パターンのスコア ---\n");
    let stt_cases: Vec<(&str, &str, &str)> = vec![
        // 清濁混同
        ("さどう", "さとう", "佐藤: t/d混同"),
        ("だなか", "たなか", "田中: t/d混同"),
        ("だかはし", "たかはし", "高橋: t/d混同"),
        ("わだなべ", "わたなべ", "渡辺: t/d混同"),
        ("がとう", "かとう", "加藤: k/g混同"),
        ("ごばやし", "こばやし", "小林: k/g混同"),
        ("ごんどう", "こんどう", "近藤: k/g混同"),
        ("いどう", "いとう", "伊藤: t/d混同"),
        ("ふぐだ", "ふくだ", "福田: k/g混同"),
        ("ざかもと", "さかもと", "坂本: s/z混同"),
        ("ざさき", "ささき", "佐々木: s/z混同"),
        ("ぐどう", "くどう", "工藤: k/g混同"),

        // 鼻音混同
        ("なかぬら", "なかむら", "中村: m/n混同"),
        ("きぬら", "きむら", "木村: m/n混同"),
        ("やなもと", "やまもと", "山本: m/n混同"),
        ("やなだ", "やまだ", "山田: m/n混同"),
        ("やなぐち", "やまぐち", "山口: m/n混同"),
        ("やなした", "やました", "山下: m/n混同"),
        ("やなざき", "やまざき", "山崎: m/n混同"),

        // 拗音・合拗音混同
        ("しにず", "しみず", "清水: m/n混同"),
        ("なかじな", "なかじま", "中島: m/n混同"),

        // 長音の有無
        ("さいとー", "さいとう", "斉藤: 長音"),
        ("いのーえ", "いのうえ", "井上: 長音"),
        ("えんどー", "えんどう", "遠藤: 長音"),
        ("ごとー", "ごとう", "後藤: 長音"),
        ("くどー", "くどう", "工藤: 長音"),

        // 複合誤変換
        ("だがはし", "たかはし", "高橋: t/d+k/g混同"),
        ("わだなべ", "わたなべ", "渡辺: t/d混同"),
        ("ばやし", "はやし", "林: h/b混同"),
    ];

    println!("{:<20} {:<15} {:<25} {:>6}", "STT入力", "正解", "説明", "スコア");
    println!("{}", "-".repeat(70));

    let mut category_scores: std::collections::BTreeMap<&str, Vec<f32>> = std::collections::BTreeMap::new();

    for (input, correct, desc) in &stt_cases {
        let score = m.calculate_similarity(input, correct);
        println!("{:<20} {:<15} {:<25} {:>6.3}", input, correct, desc, score);

        let cat = if desc.contains("t/d") || desc.contains("k/g") || desc.contains("s/z") {
            "清濁混同"
        } else if desc.contains("m/n") {
            "鼻音混同"
        } else if desc.contains("長音") {
            "長音"
        } else if desc.contains("h/b") {
            "h/b混同"
        } else {
            "複合"
        };
        category_scores.entry(cat).or_default().push(score);
    }

    println!("\n--- カテゴリ別平均 ---\n");
    println!("{:<15} {:>5} {:>8} {:>8} {:>8}", "カテゴリ", "件数", "平均", "最小", "最大");
    println!("{}", "-".repeat(50));
    for (cat, scores) in &category_scores {
        let n = scores.len();
        let avg = scores.iter().sum::<f32>() / n as f32;
        let min = scores.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        println!("{:<15} {:>5} {:>8.3} {:>8.3} {:>8.3}", cat, n, avg, min, max);
    }

    // === 2. マスタに対するTop1正解率 ===
    println!("\n--- 2. 苗字マスタに対するTop1/Top3正解率 ---\n");

    let matching_cases: Vec<(&str, &str, &str)> = vec![
        ("さどう", "さとう", "佐藤 t/d"),
        ("だなか", "たなか", "田中 t/d"),
        ("だかはし", "たかはし", "高橋 t/d"),
        ("わだなべ", "わたなべ", "渡辺 t/d"),
        ("がとう", "かとう", "加藤 k/g"),
        ("いどう", "いとう", "伊藤 t/d"),
        ("ごばやし", "こばやし", "小林 k/g"),
        ("ざさき", "ささき", "佐々木 s/z"),
        ("なかぬら", "なかむら", "中村 m/n"),
        ("きぬら", "きむら", "木村 m/n"),
        ("やなもと", "やまもと", "山本 m/n"),
        ("やなだ", "やまだ", "山田 m/n"),
        ("やなぐち", "やまぐち", "山口 m/n"),
        ("やなざき", "やまざき", "山崎 m/n"),
        ("さいとー", "さいとう", "斉藤 長音"),
        ("えんどー", "えんどう", "遠藤 長音"),
        ("ごとー", "ごとう", "後藤 長音"),
        ("くどー", "くどう", "工藤 長音"),
        ("だがはし", "たかはし", "高橋 t/d+k/g"),
        ("ばやし", "はやし", "林 h/b"),
        ("ふぐだ", "ふくだ", "福田 k/g"),
        ("ざかもと", "さかもと", "坂本 s/z"),
        ("しにず", "しみず", "清水 m/n"),
        ("ごんどー", "こんどう", "近藤 k/g+長音"),
    ];

    let mut correct_top1 = 0;
    let mut correct_top3 = 0;
    let total = matching_cases.len();

    println!("{:<18} {:<12} {:<12} {:>6} {:<8} {}",
        "STT入力", "期待", "Top1結果", "Score", "判定", "説明");
    println!("{}", "-".repeat(90));

    for (input, expected, desc) in &matching_cases {
        let results = m.find_top_matches(input, &surname_master, 5);
        let top1 = &results[0];
        let is_top1 = top1.text == *expected;
        let is_top3 = results.iter().take(3).any(|r| r.text == *expected);

        if is_top1 { correct_top1 += 1; }
        if is_top3 { correct_top3 += 1; }

        let mark = if is_top1 { "✓ Top1" } else if is_top3 { "△ Top3" } else { "✗ Miss" };
        println!("{:<18} {:<12} {:<12} {:>6.3} {:<8} {}",
            input, expected, top1.text, top1.score, mark, desc);
        if !is_top1 {
            for (i, r) in results.iter().enumerate().take(3) {
                println!("    {}位: {:<12} {:.3}", i + 1, r.text, r.score);
            }
        }
    }

    println!("\n--- 総合正解率 ---\n");
    println!("Top1正解率: {}/{} ({:.1}%)", correct_top1, total, correct_top1 as f64 / total as f64 * 100.0);
    println!("Top3正解率: {}/{} ({:.1}%)", correct_top3, total, correct_top3 as f64 / total as f64 * 100.0);

    // === 3. 苗字特有の課題: 紛らわしいペア ===
    println!("\n--- 3. 紛らわしい苗字ペア ---\n");
    let confusable_pairs: Vec<(&str, &str, &str)> = vec![
        ("さとう", "さいとう", "佐藤 vs 斉藤"),
        ("さとう", "かとう", "佐藤 vs 加藤"),
        ("さとう", "ごとう", "佐藤 vs 後藤"),
        ("かとう", "ごとう", "加藤 vs 後藤"),
        ("かとう", "いとう", "加藤 vs 伊藤"),
        ("いとう", "さいとう", "伊藤 vs 斉藤"),
        ("たかはし", "たかだ", "高橋 vs 高田"),
        ("たかはし", "たかぎ", "高橋 vs 高木"),
        ("たかだ", "たかの", "高田 vs 高野"),
        ("なかむら", "なかやま", "中村 vs 中山"),
        ("なかむら", "なかじま", "中村 vs 中島"),
        ("なかやま", "なかの", "中山 vs 中野"),
        ("なかやま", "なかがわ", "中山 vs 中川"),
        ("やまだ", "やまもと", "山田 vs 山本"),
        ("やまだ", "やまぐち", "山田 vs 山口"),
        ("やまだ", "やまざき", "山田 vs 山崎"),
        ("やました", "やまもと", "山下 vs 山本"),
        ("おおた", "おおにし", "太田 vs 大西"),
        ("おおた", "おおの", "太田 vs 大野"),
        ("もり", "もりた", "森 vs 森田"),
        ("もりた", "もりかわ", "森田 vs 森川"),
        ("こばやし", "はやし", "小林 vs 林"),
    ];

    println!("{:<15} {:<15} {:>6}  {}", "苗字A", "苗字B", "スコア", "ペア");
    println!("{}", "-".repeat(60));
    for (a, b, desc) in &confusable_pairs {
        let score = m.calculate_similarity(a, b);
        let risk = if score > 0.85 { "⚠ 混同高" } else if score > 0.75 { "△ やや高" } else if score > 0.65 { "注意" } else { "OK" };
        println!("{:<15} {:<15} {:>6.3}  {} [{}]", a, b, score, desc, risk);
    }

    // === 4. 苗字特有の問題: 短い苗字 ===
    println!("\n--- 4. 短い苗字(2音素)のマッチング ---\n");
    let short_surnames: Vec<(&str, &str, &str)> = vec![
        ("のり", "もり", "森: m/n, o→o"),
        ("ぼり", "ほり", "堀: h/b"),
        ("あぺ", "あべ", "阿部: b/p"),
        ("もり", "もり", "森: 完全一致"),
        ("ほり", "ほり", "堀: 完全一致"),
    ];
    println!("{:<12} {:<12} {:>6}  {}", "入力", "候補", "スコア", "説明");
    for (input, cand, desc) in &short_surnames {
        let score = m.calculate_similarity(input, cand);
        println!("{:<12} {:<12} {:>6.3}  {}", input, cand, score, desc);
    }

    // === 5. 企業名 vs 苗字 の特性比較 ===
    println!("\n--- 5. 企業名との特性比較 ---\n");
    println!("特性                    企業名                    苗字");
    println!("{}", "-".repeat(70));
    println!("平均文字列長              長い(4-10音素)              短い(2-5音素)");
    println!("略称の存在               あり(トヨタ/とよた)          なし");
    println!("長音ー の頻度             高い(カルビー, ソニー)        低い(さいとう→おう)");
    println!("促音ッ の頻度             中程度(ニック, バックス)       低い");
    println!("拗音の頻度               中程度(シャープ)             低い");
    println!("同音異義・類似語           中程度                     高い(〜とう, 〜やま等)");
    println!("主なSTT誤変換            清濁, 長音省略              清濁, 鼻音m/n");

    println!("\n--- 6. 苗字マッチングの総合評価 ---\n");
    println!("Top1正解率: {:.1}%", correct_top1 as f64 / total as f64 * 100.0);
    println!("Top3正解率: {:.1}%", correct_top3 as f64 / total as f64 * 100.0);
}