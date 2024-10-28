---
title: 現状と今後
---

<h1>現状と今後</h1>

<h2>現状の結果</h2>
<p class="text-lg">TypeScript のほぼ全ての構文をパースできる</p>
<p class="text-lg">一方で型検査は概要で述べたアーキテクチャに基づいて実装中</p>

```ts
function id(a) {
	return a
}

const d: 3 = id(2)
//              ^ Type 2 is not assignable to type 3
```

<h2>今後の実装予定</h2>
<p class="text-lg">型検査できる構文を増やす</p>
<p class="text-lg">プレイグラウンドを作成し、フィードバックを受けられる状態を整える</p>
