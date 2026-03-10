//! 企業マスタから STT 入力にマッチする企業名を検索する実用例
//!
//! 想定フロー:
//!   1. アプリ起動時にCSV/DBから企業マスタを読み込み、PrecomputedAliases を構築
//!   2. STT からテキストが届くたびにマッチング実行
//!   3. 上位3件を返す

use rapid_phonetic_matcher::{AliasEntry, PhoneticMatcher, PrecomputedAliases};

/// CSVやDBから読み込んだ企業マスタを想定した構造体
struct CompanyRecord {
    id: u32,
    display_name: String,
    readings: Vec<String>, // 正式読み, 略称, 別名 ...
}

/// 企業マスタを構築する（実際にはCSV/DBから読み込む）
fn load_company_master() -> Vec<CompanyRecord> {
    vec![
        CompanyRecord {
            id: 1,
            display_name: "トヨタ自動車".into(),
            readings: vec!["とよたじどうしゃ".into(), "とよた".into()],
        },
        CompanyRecord {
            id: 2,
            display_name: "本田技研工業".into(),
            readings: vec!["ほんだぎけんこうぎょう".into(), "ほんだ".into()],
        },
        CompanyRecord {
            id: 3,
            display_name: "日産自動車".into(),
            readings: vec!["にっさんじどうしゃ".into(), "にっさん".into()],
        },
        CompanyRecord {
            id: 4,
            display_name: "パナソニック".into(),
            readings: vec!["ぱなそにっく".into()],
        },
        CompanyRecord {
            id: 5,
            display_name: "ソニー".into(),
            readings: vec!["そにー".into()],
        },
        CompanyRecord {
            id: 6,
            display_name: "カルビー".into(),
            readings: vec!["かるびー".into()],
        },
        CompanyRecord {
            id: 7,
            display_name: "カルピス".into(),
            readings: vec!["かるぴす".into()],
        },
        CompanyRecord {
            id: 8,
            display_name: "P&G".into(),
            readings: vec!["ぴーあんどじー".into()],
        },
        CompanyRecord {
            id: 9,
            display_name: "IBM".into(),
            readings: vec!["あいびーえむ".into()],
        },
        CompanyRecord {
            id: 10,
            display_name: "3M".into(),
            readings: vec!["すりーえむ".into()],
        },
        CompanyRecord {
            id: 11,
            display_name: "マクドナルド".into(),
            readings: vec!["まくどなるど".into(), "まっく".into(), "まくど".into()],
        },
        CompanyRecord {
            id: 12,
            display_name: "スターバックス".into(),
            readings: vec!["すたーばっくす".into(), "すたば".into()],
        },
        CompanyRecord {
            id: 13,
            display_name: "ダイキン工業".into(),
            readings: vec!["だいきんこうぎょう".into(), "だいきん".into()],
        },
        CompanyRecord {
            id: 14,
            display_name: "ユニクロ".into(),
            readings: vec!["ゆにくろ".into()],
        },
        CompanyRecord {
            id: 15,
            display_name: "NTTドコモ".into(),
            readings: vec!["えぬてぃーてぃーどこも".into(), "どこも".into()],
        },
    ]
}

fn main() {
    // =====================================================
    //  Step 1: 起動時にマスタを読み込み、事前計算する
    // =====================================================
    let master = load_company_master();

    // CompanyRecord → AliasEntry に変換
    let entries: Vec<AliasEntry> = master
        .iter()
        .map(|r| AliasEntry::from_strings(r.display_name.clone(), r.readings.clone()))
        .collect();

    // 音素データを事前計算（起動時に1回だけ）
    let precomputed = PrecomputedAliases::new(&entries);
    let matcher = PhoneticMatcher::new();

    println!("企業マスタ: {} 件登録済み\n", master.len());

    // =====================================================
    //  Step 2: STT入力が届くたびにマッチング
    // =====================================================
    let stt_inputs = vec![
        "どよだ",           // トヨタ（t/d混同 + 略称）
        "ほんた",           // ホンダ（d/t混同 + 略称）
        "かるみー",          // カルビー（m/b混同）
        "ばなそにっく",       // パナソニック（p/b混同）
        "IBM",             // IBM（アルファベット入力）
        "P&G",             // P&G（アルファベット+記号）
        "3M",              // 3M（数字+アルファベット）
        "すたば",           // スターバックス（略称）
        "まっく",           // マクドナルド（略称）
        "だいぎん",          // ダイキン（k/g混同 + 略称）
        "そに",             // ソニー（長音省略）
        "ゆにぐろ",          // ユニクロ（k/g混同）
    ];

    println!("{:<20} {:<20} {:>6}  {:?}", "STT入力", "マッチ結果", "Score", "Confidence");
    println!("{}", "=".repeat(70));

    for input in &stt_inputs {
        // 上位3件を取得
        let results = matcher.find_top_matches_with_aliases_precomputed(
            input,
            &precomputed,
            3,
        );

        // 1位を表示
        let top = &results[0];
        println!("{:<20} {:<20} {:>6.3}  {:?}", input, top.text, top.score, top.confidence);

        // 2位以降（スコア差が小さい場合は注意喚起）
        for r in results.iter().skip(1) {
            let gap = top.score - r.score;
            if gap < 0.1 {
                println!("  ⚠ 僅差: {:<16} {:>6.3}  (差: {:.3})", r.text, r.score, gap);
            }
        }
    }

    // =====================================================
    //  Step 3: フィルタ付きマッチング（偽陽性排除）
    // =====================================================
    println!("\n--- フィルタ付き: min_score=0.7 ---\n");

    let filtered_inputs = vec!["らーめん", "かるみー", "ほんた"];

    for input in &filtered_inputs {
        let results = matcher.find_matches_with_aliases_filtered(
            input,
            &entries,
            3,
            0.7,  // スコア0.7未満は除外
        );

        if results.is_empty() {
            println!("{:<20} → マッチなし（全候補がスコア0.7未満）", input);
        } else {
            for r in &results {
                println!("{:<20} → {:<20} {:.3}  {:?}", input, r.text, r.score, r.confidence);
            }
        }
    }
}