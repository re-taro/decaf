---
title: 比較：TypeScript と decaf の型検査の仕組み
---

<h1>比較：TypeScript と decaf の型検査の仕組み</h1>
<img
  class="mx-auto w-sm transition-width"
  src="/figures/decaf_check.png"
  alt="decaf の型検査の仕組み"
/>

<div class="grid grid-cols-2">
  <div class="flex flex-col">

  - スケーリング制限
    - <span v-mark="{ at: 0, color: 'red', type: 'underline' }">シングルスレッド</span>
  - CPU 負荷
    - <span v-mark="{ at: 0, color: 'red', type: 'underline' }">歴史的経緯によるレガシーな実装</span>
  - メモリ圧迫
    - <span v-mark="{ at: 0, color: 'red', type: 'underline' }">型情報の重複保持</span>
    - 構造的部分型による比較コスト

  </div>
  <Arrow x1="380" y1="420" x2="480" y2="420" />
  <div class="flrx flex-col">

  - シングルスレッド
    - <span v-mark="{ at: 1, color: 'red', type: 'underline' }">Rust によるマルチスレッド化</span>
  - レガシーな実装
    - <span v-mark="{ at: 2, color: 'red', type: 'underline' }">TyeID の導入</span>による現代的な型検査手法を実装できるアーキテクチャ
  - 型情報の重複保持
    - <span v-mark="{ at: 3, color: 'red', type: 'underline' }">型情報を不可分に保持</span>することで、重複を排除
  </div>
</div>
