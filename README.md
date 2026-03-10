# rapid_phonetic_matcher

日本語の音響的類似度に基づいて文字列をマッチングする Rust ライブラリ。

音声認識（STT）の誤変換を補正するために設計されています。単純な編集距離ではなく、日本語の音素に分解した上で「人間やSTTが聞き間違えやすい音のペア」の置換コストを低く設定した重み付き音素編集距離を使用します。

## 特長

- **音響的類似度** — 清濁混同（カ↔ガ）、鼻音混同（マ↔ナ）、有声破裂音混同（バ↔ダ↔ガ）などを高類似度と判定
- **長音・促音の寛容マッチング** — 「ビール」と「ビル」、「マッチ」と「マチ」を高スコアで一致
- **別名（略称）対応** — 正式名称と略称の両方でマッチング（「やまと運輸」←→「やまと」）
- **アルファベット・数字対応** — アルファベットをカタカナ文字読みに自動変換
- **事前計算による高速化** — 大量マスタの繰り返しマッチングに対応
- **漢字入力対応**（オプション）— lindera 形態素解析で漢字をカタカナ読みに変換

## インストール

```toml
[dependencies]
rapid_phonetic_matcher = { git = "https://github.com/SatoruKuroiwa/rapid-phonetic-matcher" }
```

漢字入力対応が必要な場合：

```toml
[dependencies]
rapid_phonetic_matcher = { git = "https://github.com/SatoruKuroiwa/rapid-phonetic-matcher", features = ["kanji"] }
```

## 使い方

### 2つの文字列の類似度を計算

```rust
use rapid_phonetic_matcher::PhoneticMatcher;

let matcher = PhoneticMatcher::new();

// 清濁混同: 高類似度
let score = matcher.calculate_similarity("タマゴ", "タマコ");
assert!(score > 0.8);

// 鼻音混同: 高類似度
let score = matcher.calculate_similarity("カニ", "カミ");
assert!(score > 0.8);
```

### 候補リストから上位N件を検索

```rust
use rapid_phonetic_matcher::PhoneticMatcher;

let matcher = PhoneticMatcher::new();
let candidates = vec!["サクラ", "サカナ", "ササミ", "タケノコ"];

let results = matcher.find_top_matches("サグラ", &candidates, 3);
for r in &results {
    println!("{}: {:.3} ({:?})", r.text, r.score, r.confidence);
}
// → サクラ: 0.850 (High)  — k/g 清濁混同を高類似度と判定
```

### 別名（略称）を使ったマッチング

```rust
use rapid_phonetic_matcher::{PhoneticMatcher, AliasEntry, PrecomputedAliases};

// マスタ構築（起動時に1回）
let entries = vec![
    AliasEntry::new("さくら商事株式会社", &["さくらしょうじかぶしきがいしゃ", "さくらしょうじ"]),
    AliasEntry::new("はなまる食品工業", &["はなまるしょくひんこうぎょう", "はなまる"]),
    AliasEntry::new("みどり物産",       &["みどりぶっさん", "みどり"]),
];
let precomputed = PrecomputedAliases::new(&entries);

// マッチング実行（略称「はなまる」の清濁混同でもマッチ）
let matcher = PhoneticMatcher::new();
let results = matcher.find_top_matches_with_aliases_precomputed("はなまる", &precomputed, 3);
// → はなまる食品工業 (1.000, Exact)
```

### DB/CSVからのデータ読み込み

```rust
use rapid_phonetic_matcher::AliasEntry;

// String を直接渡せる from_strings コンストラクタ
let entry = AliasEntry::from_strings(
    "さくら商事株式会社".to_string(),
    vec!["さくらしょうじかぶしきがいしゃ".to_string(), "さくらしょうじ".to_string()],
);
```

### スコアフィルタ（偽陽性排除）

```rust
use rapid_phonetic_matcher::PhoneticMatcher;

let matcher = PhoneticMatcher::new();
let candidates = vec!["サクラ", "ヒマワリ", "タンポポ"];

// スコア 0.7 未満の候補を除外
let results = matcher.find_matches_filtered("らーめん", &candidates, 3, 0.7);
assert!(results.is_empty()); // 無関係な入力はマッチしない
```

## 信頼度レベル

| レベル | スコア範囲 | 意味 |
|--------|-----------|------|
| `Exact` | ≥ 0.95 | 完全一致またはほぼ同一 |
| `High` | ≥ 0.80 | 高い信頼度（清濁混同など） |
| `Medium` | ≥ 0.60 | 中程度の信頼度 |
| `Low` | ≥ 0.40 | 低い信頼度 |
| `NoMatch` | < 0.40 | マッチなし |

## CLIツール

簡易テスト用のコマンドラインツールが付属しています。

```bash
# ビルド
cargo build --bin rpm

# 2つの文字列を比較
cargo run --bin rpm -- タマゴ タマコ
# → 0.850  High  タマゴ vs タマコ

# 候補をランキング
cargo run --bin rpm -- サグラ サクラ サカナ ササミ
# → 類似度順にソートして表示
```

## 音響コスト行列

| 音の関係 | 置換コスト | 例 |
|---------|-----------|-----|
| 有声破裂音同士 (b/d/g) | 0.3 | バ↔ダ↔ガ |
| 清濁ペア (k/g, t/d, s/z) | 0.5 | カ↔ガ、タ↔ダ |
| 鼻音 (m/n) | 0.4 | マ↔ナ |
| 外来音 (v/b) | 0.3 | ヴァ↔バ |
| 母音違い | 0.8 | ア↔イ |
| 長音の挿入/削除 | 0.3 | ビール↔ビル |
| 通常音素の挿入/削除 | 1.0 | — |

## ライセンス

MIT License