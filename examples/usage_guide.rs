//! マスタデータから STT 入力にマッチするエントリを検索する実用例
//!
//! 想定フロー:
//!   1. アプリ起動時にCSV/DBからマスタを読み込み、PrecomputedAliases を構築
//!   2. STT からテキストが届くたびにマッチング実行
//!   3. 上位3件を返す

use rapid_phonetic_matcher::{AliasEntry, PhoneticMatcher, PrecomputedAliases};

/// CSV/DBから読み込んだマスタを想定した構造体
struct MasterRecord {
    display_name: String,
    readings: Vec<String>, // 正式読み, 略称, 別名 ...
}

/// マスタを構築する（実際にはCSV/DBから読み込む）
fn load_master() -> Vec<MasterRecord> {
    vec![
        MasterRecord {
            display_name: "さくら商事株式会社".into(),
            readings: vec!["さくらしょうじかぶしきがいしゃ".into(), "さくらしょうじ".into()],
        },
        MasterRecord {
            display_name: "はなまる食品工業".into(),
            readings: vec!["はなまるしょくひんこうぎょう".into(), "はなまる".into()],
        },
        MasterRecord {
            display_name: "みどり物産".into(),
            readings: vec!["みどりぶっさん".into(), "みどり".into()],
        },
        MasterRecord {
            display_name: "あおぞらサービス".into(),
            readings: vec!["あおぞらさーびす".into(), "あおぞら".into()],
        },
        MasterRecord {
            display_name: "たかはし金属工業".into(),
            readings: vec!["たかはしきんぞくこうぎょう".into(), "たかはしきんぞく".into()],
        },
        MasterRecord {
            display_name: "やまと運輸".into(),
            readings: vec!["やまとうんゆ".into(), "やまと".into()],
        },
        MasterRecord {
            display_name: "ひかりシステム".into(),
            readings: vec!["ひかりしすてむ".into(), "ひかり".into()],
        },
        MasterRecord {
            display_name: "ふじやまホールディングス".into(),
            readings: vec!["ふじやまほーるでぃんぐす".into(), "ふじやま".into()],
        },
    ]
}

fn main() {
    // =====================================================
    //  Step 1: 起動時にマスタを読み込み、事前計算する
    // =====================================================
    let master = load_master();

    // MasterRecord → AliasEntry に変換
    let entries: Vec<AliasEntry> = master
        .iter()
        .map(|r| AliasEntry::from_strings(r.display_name.clone(), r.readings.clone()))
        .collect();

    // 音素データを事前計算（起動時に1回だけ）
    let precomputed = PrecomputedAliases::new(&entries);
    let matcher = PhoneticMatcher::new();

    println!("マスタ: {} 件登録済み\n", master.len());

    // =====================================================
    //  Step 2: STT入力が届くたびにマッチング
    // =====================================================
    let stt_inputs = vec![
        "さぐらしょうじ",    // さくら商事（k/g混同）
        "はなまる",          // はなまる食品工業（略称そのまま）
        "はななる",          // はなまる食品工業（m/n混同）
        "みとり",            // みどり物産（d/t混同 + 略称）
        "だかはしきんぞく",   // たかはし金属工業（t/d混同）
        "やなと",            // やまと運輸（m/n混同 + 略称）
        "ひがりしすてむ",    // ひかりシステム（k/g混同）
        "ふじやな",          // ふじやまHD（m/n混同 + 略称）
    ];

    println!("{:<25} {:<25} {:>6}  {:?}", "STT入力", "マッチ結果", "Score", "Confidence");
    println!("{}", "=".repeat(80));

    for input in &stt_inputs {
        // 上位3件を取得
        let results = matcher.find_top_matches_with_aliases_precomputed(
            input,
            &precomputed,
            3,
        );

        // 1位を表示
        let top = &results[0];
        println!("{:<25} {:<25} {:>6.3}  {:?}", input, top.text, top.score, top.confidence);

        // 2位以降（スコア差が小さい場合は注意喚起）
        for r in results.iter().skip(1) {
            let gap = top.score - r.score;
            if gap < 0.1 {
                println!("  ⚠ 僅差: {:<20} {:>6.3}  (差: {:.3})", r.text, r.score, gap);
            }
        }
    }

    // =====================================================
    //  Step 3: フィルタ付きマッチング（偽陽性排除）
    // =====================================================
    println!("\n--- フィルタ付き: min_score=0.7 ---\n");

    let filtered_inputs = vec!["らーめん", "はなまる", "やなと"];

    for input in &filtered_inputs {
        let results = matcher.find_matches_with_aliases_filtered(
            input,
            &entries,
            3,
            0.7,  // スコア0.7未満は除外
        );

        if results.is_empty() {
            println!("{:<25} → マッチなし（全候補がスコア0.7未満）", input);
        } else {
            for r in &results {
                println!("{:<25} → {:<25} {:.3}  {:?}", input, r.text, r.score, r.confidence);
            }
        }
    }
}