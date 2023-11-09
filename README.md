# システム設計の面接試験 13章 オートコンプリートシステムの設計

トライ木による実装(シングルノードでシャーディングなし)

- ルートは空文字
- 各ノードは単語(検索クエリ)、もしくは接頭辞文字列を表す
- 各ノードで上位5個の検索クエリをキャッシュする

# 実験

重複ありで1000万($10^7$)ワード入れる。1秒もかからずに各ノードにおける上位5個の検索クエリを更新できる。上位5つの検索クエリの取得も高速にできる。

```sh
cargo run
```

```
process time: 0.652
auto complete list for `twi`
twillings: 370257
twier: 367178
twirliest: 365272
twiggier: 363195
twigged: 355758

auto complete list for `bug`
bugler: 352875
bugger: 341458
bugle: 338338
bugging: 331927
bugs: 331503
```

各ノードで上位5つだけ検索クエリをもって、葉から順番に上位5つの検索クエリを決めていけば良い。
計算量は$O(NK \log{NK})$。$N$はノード数で、$K$は各ノードで持つ上位$K$個の検索クエリ。

更新処理はatcoderの以下の問題と似てると思った。
https://atcoder.jp/contests/abc239/tasks/abc239_e

