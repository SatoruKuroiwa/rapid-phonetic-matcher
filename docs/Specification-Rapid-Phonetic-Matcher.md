# Specification: `rapid_phonetic_matcher` (Rust Library)

## 1. 目的

音声認識（STT）によって生じる「音は似ているが漢字や表記が異なる誤変換（例：刈る美 vs カルビー）」を、日本語の音響的類似度に基づいて補正・マッチングするための Rust ライブラリ。

## 2. コア・ロジック：重み付き音素編集距離

単なる文字列の編集距離（Levenshtein distance）ではなく、日本語の音素（Phoneme）に分解した上で、**「人間や STT が聞き間違えやすい音のペア」の置換コストを低く設定**した独自のアルゴリズムを実装する。

### 2.1. 前処理と音素化 (Normalization & Phoneticization)

1. **正規化:** カタカナへの統一、長音（ー）の扱い（削除または直前の母音の延長）、促音（ッ）の正規化。
2. **音素分解:** カタカナを「子音 + 母音」のペアに分解する。
* 例：`カルビー`  `k a r u b i i`
* 例：`カルディー`  `k a r u d i i`



### 2.2. 混乱行列 (Cost Matrix) の定義

以下のペアの置換コストをデフォルトの `1.0` よりも低く設定する。

* **有声破裂音グループ (b, d, g):** 互いの置換コストを `0.3` とする。
* **清音・濁音・半濁音ペア (h/b/p, s/z, k/g, t/d):** 置換コストを `0.5` とする。
* **鼻音ペア (m, n):** 置換コストを `0.4` とする。
* **母音の近似性:** 日本語の母音（a, i, u, e, o）の置換は一律 `0.8` とする（子音よりはマシだが、聞き間違いの要因になるため）。

## 3. インターフェース要求 (API Design)

### 3.1. 主要な構造体と関数

```rust
pub struct PhoneticMatcher {
    // 内部にコストマトリックスやキャッシュを保持
}

impl PhoneticMatcher {
    /// 2つのカタカナ文字列の類似度を 0.0 ~ 1.0 で返す
    pub fn calculate_similarity(&self, input: &str, candidate: &str) -> f32;

    /// 候補リストの中から上位 N 件をスコア付きで返す
    pub fn find_top_matches(&self, input: &str, candidates: Vec<&str>, limit: usize) -> Vec<MatchResult>;
}

pub struct MatchResult {
    pub text: String,
    pub score: f32, // 1.0 が完全一致
}

```

## 4. テストケース (Unit Test Requirements)

以下のケースで、単純な編集距離よりも高いスコア（吸着力）を発揮することを確認する。

1. **有声破裂音の混同:** `カルミ` (刈る美) vs `カルビ` (カルビー)  **High Similarity**
2. **類似固有名詞の識別:** `カルビー` vs `カルディー`  **High Similarity** (ただし完全一致よりは低い)
3. **清音・濁音の揺らぎ:** `バレット` vs `パレット`  **Moderate-High Similarity**
4. **長音・促音の無視:** `センター` vs `センタ` / `トラック` vs `トラク`  **High Similarity**

## 5. 実装上の注意点（Claude Code への指示）

* **パフォーマンス:** 数千件のマスタと突合するため、計算量は最小限に。可能な限り事前に音素化したデータをキャッシュできる設計にする。
* **日本語特性:** `ディ (di)` や `シュ (shu)` などの多文字カタカナ（合拗音）を正しく 1 つの音素ユニットとして扱うこと。
* **依存関係:** `unicode-normalization` や、必要であれば形態素解析（`lindera` 等）を検討するが、ライブラリ自体は軽量に保つ。

