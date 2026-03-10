use rapid_phonetic_matcher::PhoneticMatcher;

fn main() {
    let m = PhoneticMatcher::new();

    println!("--- アルファベット+数字の企業名マッチング ---\n");
    let cases = vec![
        // 数字+アルファベット → 英語読み
        ("3M", "スリーエム", "3M: 英語読み"),
        ("3M", "サンエム", "3M: 日本語読みとの比較"),
        ("7-Eleven", "セブンイレブン", "7-Eleven"),
        ("B2B", "ビーツービー", "B2B"),
        ("P2P", "ピーツーピー", "P2P"),
        ("G7", "ジーセブン", "G7"),
        ("Y2K", "ワイツーケー", "Y2K"),
        ("4℃", "ヨンドシー", "4℃ (数字のみ→日本語読み)"),

        // 既存の確認
        ("P&G", "ピーアンドジー", "P&G"),
        ("IBM", "アイビーエム", "IBM"),
        ("NTT", "エヌティーティー", "NTT"),
        ("AT&T", "エーティーアンドティー", "AT&T"),

        // カナ入力はそのまま
        ("スリーエム", "スリーエム", "カナ入力(変化なし)"),
    ];

    println!("{:<15} vs {:<25} = {:>6}  {}", "入力", "マスタ", "スコア", "説明");
    println!("{}", "-".repeat(70));
    for (a, b, desc) in &cases {
        let score = m.calculate_similarity(a, b);
        println!("{:<15} vs {:<25} = {:>6.3}  {}", a, b, score, desc);
    }
}