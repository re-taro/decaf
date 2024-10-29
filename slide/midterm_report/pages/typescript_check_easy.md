---
title: TypeScript の型検査の仕組み（簡略版）
transition: fade
---

<h1>TypeScript の型検査の仕組み（簡略版）</h1>
<img
  class="mx-auto transition-width"
  :class="$clicks === 0 ? 'w-xl' : 'w-sm'"
  src="/figures/typescript_check_easy.png"
  alt="TypeScript の型検査の仕組み（簡略版）"
/>

<div v-click class="flex flex-col">

- スケーリング制限
  - <span v-mark="{ at: 2, color: 'red', type: 'underline' }">シングルスレッド</span>
- CPU 負荷
  - <span v-mark="{ at: 2, color: 'red', type: 'underline' }">歴史的経緯によるレガシーな実装</span>
- メモリ圧迫
  - <span v-mark="{ at: 2, color: 'red', type: 'underline' }">型情報の重複保持</span>
  - 構造的部分型による比較コスト

</div>
