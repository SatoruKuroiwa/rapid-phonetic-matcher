use rapid_phonetic_matcher::PhoneticMatcher;

fn main() {
    let m = PhoneticMatcher::new();

    println!("=============================================");
    println!("  弱点・改善余地の定量分析");
    println!("=============================================\n");

    // === 1. 長音挿入/削除のコスト問題 ===
    println!("--- 1. 長音の挿入/削除コストが高すぎる ---\n");
    let long_vowel_cases = vec![
        ("そにー", "そに", "ソニー: 3音素 vs 2音素"),
        ("びーる", "びる", "ビール: 3音素 vs 2音素"),
        ("めーる", "める", "メール: 3音素 vs 2音素"),
        ("こーひー", "こひ", "コーヒー: 4音素 vs 2音素"),
        ("さーばー", "さば", "サーバー: 4音素 vs 2音素"),
        ("せんたー", "せんた", "センター: 4音素 vs 3音素"),
        ("すたーばっくす", "すたばくす", "スターバックス(促音+長音除去)"),
    ];
    println!("  {:<20} {:<15} {:>6}  {}", "入力A", "入力B", "スコア", "説明");
    for (a, b, desc) in &long_vowel_cases {
        let score = m.calculate_similarity(a, b);
        println!("  {:<20} {:<15} {:>6.3}  {}", a, b, score, desc);
    }
    println!("\n  問題: 長音展開された母音(アイウエオ)の挿入/削除コストが1.0で、");
    println!("       通常の子音+母音の挿入/削除と同じ重みになっている。");
    println!("       「そにー」→「そにイ」の末尾「イ」は実質的に長音の有無なので、");
    println!("       もっと低いコスト(例: 0.3)にすべき。\n");

    // === 2. 同一接頭辞の企業の弁別力 ===
    println!("--- 2. 接頭辞が同じ企業の弁別 ---\n");
    let prefix_cases = vec![
        ("かるびー", "かるぴす", "カルビー vs カルピス"),
        ("かるびー", "かるでぃお", "カルビー vs カルディオ"),
        ("かるびー", "かるばす", "カルビー vs カルバス"),
        ("かるぴす", "かるばす", "カルピス vs カルバス"),
        ("にっさん", "にっせい", "日産 vs ニッセイ"),
        ("にっさん", "にっせん", "日産 vs ニッセン"),
        ("にっせい", "にっせん", "ニッセイ vs ニッセン"),
        ("だいきん", "だいそー", "ダイキン vs ダイソー"),
        ("だいきん", "だいはつ", "ダイキン vs ダイハツ"),
    ];
    println!("  {:<20} {:<20} {:>6}  {}", "企業A", "企業B", "スコア", "ペア");
    for (a, b, desc) in &prefix_cases {
        let score = m.calculate_similarity(a, b);
        let risk = if score > 0.8 { "⚠ 混同リスク高" } else if score > 0.7 { "△ やや高" } else { "OK" };
        println!("  {:<20} {:<20} {:>6.3}  {} [{}]", a, b, score, desc, risk);
    }
    println!("\n  問題: 「カルピス」vs「カルバス」が0.845。p/bの清濁ペア(0.5)が近すぎる。");
    println!("       接頭辞が一致すると後半の差異が相対的に小さくなる。\n");

    // === 3. 文字列長の非対称性 ===
    println!("--- 3. 長さが異なる文字列の類似度 ---\n");
    let length_cases = vec![
        ("とよた", "とよたじどうしゃ", "3音素 vs 8音素"),
        ("ほんだ", "ほんだぎけんこうぎょう", "3音素 vs 11音素"),
        ("にっさん", "にっさんじどうしゃ", "4音素 vs 9音素"),
        ("そにー", "そにーふどうさん", "3音素 vs 8音素"),
        ("まくど", "まくどなるど", "3音素 vs 6音素"),
    ];
    println!("  {:<20} {:<25} {:>6}  {}", "短い方", "長い方", "スコア", "説明");
    for (short, long, desc) in &length_cases {
        let score = m.calculate_similarity(short, long);
        println!("  {:<20} {:<25} {:>6.3}  {}", short, long, score, desc);
    }
    println!("\n  問題: 短い入力が長い正式名称にマッチしにくい。");
    println!("       → AliasEntryで略称を追加して解決済みだが、");
    println!("       「接頭辞一致ボーナス」のような仕組みがあるとさらに改善。\n");

    // === 4. 偽陽性(False Positive)の分析 ===
    println!("--- 4. 偽陽性パターンの分析 ---\n");
    let master = vec![
        "かるびー", "かるぴす", "かるめん", "かるなっく",
        "にっさん", "にっせい", "にっせん",
        "そにー", "そふとばんく", "どこも",
    ];
    let fp_inputs = vec![
        ("からめる", "カラメル(食品)"),
        ("かるて", "カルテ(医療)"),
        ("にっき", "日記"),
        ("そば", "蕎麦"),
        ("らーめん", "ラーメン"),
        ("からおけ", "カラオケ"),
    ];
    println!("  {:<15} {:<15} {:>6}  {}", "入力", "Top1マッチ", "スコア", "説明");
    for (input, desc) in &fp_inputs {
        let mut best = ("", 0.0_f32);
        for &c in &master {
            let score = m.calculate_similarity(input, c);
            if score > best.1 { best = (c, score); }
        }
        let risk = if best.1 > 0.7 { "⚠ 偽陽性" } else if best.1 > 0.5 { "△ 注意" } else { "OK" };
        println!("  {:<15} {:<15} {:>6.3}  {} [{}]", input, best.0, best.1, desc, risk);
    }

    // === 5. 子音グループの網羅性 ===
    println!("\n--- 5. 未定義の子音ペアの確認 ---\n");
    let consonant_pairs = vec![
        ("ky", "gy", "キャ/ギャ"),
        ("hy", "by", "ヒャ/ビャ"),
        ("hy", "py", "ヒャ/ピャ"),
        ("ny", "my", "ニャ/ミャ"),
        ("r", "d", "ラ行/ダ行"),
        ("r", "n", "ラ行/ナ行"),
        ("w", "b", "ワ行/バ行"),
        ("v", "b", "ヴァ行/バ行"),
        ("f", "v", "ファ行/ヴァ行"),
    ];
    println!("  子音ペアの実効コストをテスト(同じ母音で比較):");
    for (c1, c2, desc) in &consonant_pairs {
        // 同じ母音'a'で比較
        let s1 = format!("{}ア", kana_from_consonant(c1));
        let s2 = format!("{}ア", kana_from_consonant(c2));
        if !s1.is_empty() && !s2.is_empty() {
            let score = m.calculate_similarity(&s1, &s2);
            let status = if score > 0.8 { "近い" } else if score > 0.6 { "中間" } else { "遠い" };
            println!("  {:<5} vs {:<5} ({:<10}): {:<8} vs {:<8} = {:.3} [{}]",
                c1, c2, desc, s1, s2, score, status);
        }
    }
    println!("\n  問題: 拗音系(ky/gy, hy/by等)の清濁ペアが未定義の可能性。");
    println!("       v/b(ヴァ/バ)も外来語で頻出だが未定義だと高コストになる。");
}

fn kana_from_consonant(c: &str) -> String {
    match c {
        "k" => "カ", "g" => "ガ", "s" => "サ", "z" => "ザ",
        "t" => "タ", "d" => "ダ", "n" => "ナ", "h" => "ハ",
        "b" => "バ", "p" => "パ", "m" => "マ", "r" => "ラ",
        "w" => "ワ", "f" => "ファ", "v" => "ヴァ",
        "ky" => "キャ", "gy" => "ギャ",
        "hy" => "ヒャ", "by" => "ビャ", "py" => "ピャ",
        "ny" => "ニャ", "my" => "ミャ",
        _ => "",
    }.to_string()
}