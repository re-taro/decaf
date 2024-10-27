---
title: decaf の型検査の仕組み
---

<h1>decaf の型検査の仕組み</h1>
<img
  class="mx-auto transition-width"
  :class="$clicks === 0 ? 'w-xl' : 'w-sm'"
  src="/figures/decaf_check.png"
  alt="decaf の型検査の仕組み"
/>

<div v-click class="flex flex-col">

- TypeID の導入
  - 型検査機構と AST の橋渡し
  - 依存型や関数の引数の型の推論など<span v-mark="{ at: 2, color: 'red', type: 'underline' }">現代的な型検査手法を実装できる</span>
- 型を<span v-mark="{ at: 3, color: 'red', type: 'underline' }">その値が取りうる可能性の空間</span>として捉える
  - 条件型やループに差し掛かると型の空間が拡大
  - <span v-mark="{ at: 4, color: 'red', type: 'underline' }">実行時に起こりうるすべての可能性を型チェック時に考慮できる</span>

</div>
