use rapid_phonetic_matcher::PhoneticMatcher;

fn main() {
    let m = PhoneticMatcher::new();

    println!("=============================================");
    println!("  アルファベット企業名の現状確認");
    println!("=============================================\n");

    // === 1. 現状の挙動確認 ===
    println!("--- 1. 現状: アルファベット入力の挙動 ---\n");
    let cases: Vec<(&str, &str, &str)> = vec![
        ("P&G", "ピーアンドジー", "P&G vs カナ読み"),
        ("IBM", "アイビーエム", "IBM vs カナ読み"),
        ("NTT", "エヌティーティー", "NTT vs カナ読み"),
        ("JR", "ジェイアール", "JR vs カナ読み"),
        ("NEC", "エヌイーシー", "NEC vs カナ読み"),
        ("BMW", "ビーエムダブリュー", "BMW vs カナ読み"),
        ("KDDI", "ケーディーディーアイ", "KDDI vs カナ読み"),
        ("LINE", "ライン", "LINE vs カナ読み"),
        ("SONY", "ソニー", "SONY vs カナ読み"),
        ("Toyota", "トヨタ", "Toyota vs カナ読み"),
    ];

    println!("{:<15} {:<25} {:>6}  {}", "入力", "候補", "スコア", "説明");
    println!("{}", "-".repeat(65));
    for (a, b, desc) in &cases {
        let score = m.calculate_similarity(a, b);
        println!("{:<15} {:<25} {:>6.3}  {}", a, b, score, desc);
    }

    // === 2. STTの出力パターン ===
    println!("\n--- 2. STTが出力しうるパターン ---\n");
    println!("企業名「P&G」の場合、STTの出力として想定されるパターン:");
    println!("  (a) 「ピーアンドジー」 — カタカナ読み (現状対応済み)");
    println!("  (b) 「P&G」           — アルファベットそのまま (現状非対応)");
    println!("  (c) 「ぴーあんどじー」 — ひらがな (現状対応済み)");
    println!("  (d) 「ピーアンドジ」   — 長音省略 (現状対応済み)");
    println!("  (e) 「ピーエンドジー」 — 聞き間違い (現状対応済み)");
    println!();

    let pg_variants = vec![
        ("ピーアンドジー", "カタカナ正式"),
        ("ぴーあんどじー", "ひらがな"),
        ("ピーアンドジ", "長音省略"),
        ("ピーエンドジー", "アンド→エンド"),
        ("ビーアンドジー", "P/B混同"),
    ];
    let target = "ピーアンドジー";
    println!("  マスタ: {}\n", target);
    println!("  {:<25} {:>6}  {}", "入力", "スコア", "パターン");
    for (input, desc) in &pg_variants {
        let score = m.calculate_similarity(input, target);
        println!("  {:<25} {:>6.3}  {}", input, score, desc);
    }

    // === 3. アルファベット→カタカナ変換テーブルの案 ===
    println!("\n--- 3. アルファベット→カタカナ変換の提案 ---\n");
    println!("  対応案: normalizer層でアルファベットをカタカナ読みに変換する\n");

    let alphabet_readings: Vec<(&str, &str)> = vec![
        ("A", "エー"), ("B", "ビー"), ("C", "シー"), ("D", "ディー"),
        ("E", "イー"), ("F", "エフ"), ("G", "ジー"), ("H", "エイチ"),
        ("I", "アイ"), ("J", "ジェイ"), ("K", "ケー"), ("L", "エル"),
        ("M", "エム"), ("N", "エヌ"), ("O", "オー"), ("P", "ピー"),
        ("Q", "キュー"), ("R", "アール"), ("S", "エス"), ("T", "ティー"),
        ("U", "ユー"), ("V", "ブイ"), ("W", "ダブリュー"), ("X", "エックス"),
        ("Y", "ワイ"), ("Z", "ゼット"),
    ];

    println!("  変換テーブル例:");
    for chunk in alphabet_readings.chunks(6) {
        let line: Vec<String> = chunk.iter().map(|(a, k)| format!("{} → {}", a, k)).collect();
        println!("    {}", line.join("  "));
    }

    println!("\n  記号変換:");
    println!("    & → アンド");
    println!("    + → プラス");
    println!("    . → (削除)");

    // === 4. 変換後のシミュレーション ===
    println!("\n--- 4. 手動変換シミュレーション ---\n");
    let simulated: Vec<(&str, &str, &str, &str)> = vec![
        ("P&G", "ピーアンドジー", "ピーアンドジー", "完全一致になるはず"),
        ("IBM", "アイビーエム", "アイビーエム", "完全一致になるはず"),
        ("NTT", "エヌティーティー", "エヌティーティー", "完全一致になるはず"),
        ("NEC", "エヌイーシー", "エヌイーシー", "完全一致になるはず"),
        ("BMW", "ビーエムダブリュー", "ビーエムダブリュー", "完全一致になるはず"),
        ("JR", "ジェイアール", "ジェイアール", "完全一致になるはず"),
    ];

    println!("  {:<10} {:<25} {:<25} {:>6}  {}",
        "元入力", "変換後(想定)", "マスタ", "スコア", "備考");
    println!("  {}", "-".repeat(85));
    for (orig, converted, master, note) in &simulated {
        let score = m.calculate_similarity(converted, master);
        println!("  {:<10} {:<25} {:<25} {:>6.3}  {}",
            orig, converted, master, score, note);
    }

    // === 5. 課題の洗い出し ===
    println!("\n--- 5. アルファベット対応の課題 ---\n");
    println!("  (1) 単語として読む場合 vs 1文字ずつ読む場合");
    println!("      LINE → 「ライン」(単語読み) ≠ 「エルアイエヌイー」(文字読み)");
    println!("      SONY → 「ソニー」(単語読み) ≠ 「エスオーエヌワイ」(文字読み)");
    println!("      → 略称(IBM, NTT等)は文字読み、ブランド名(LINE, SONY等)は単語読み");
    println!("      → 自動判別は困難。AliasEntry方式で両方登録が現実的");
    println!();
    println!("  (2) 大文字/小文字の正規化");
    println!("      ibm = IBM = Ibm → すべて同じに正規化すべき");
    println!();
    println!("  (3) 記号の扱い");
    println!("      P&G, AT&T → & はアンド");
    println!("      Yahoo! → ! は削除");
    println!("      3M → 数字はスリーエム？ → 数字変換も必要");
    println!();
    println!("  (4) ローマ字読みの可能性");
    println!("      Tanaka → たなか (ローマ字→ひらがな変換)");
    println!("      → 苗字/地名のローマ字入力にも対応可能になる");
}