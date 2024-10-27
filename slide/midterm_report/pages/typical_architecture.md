---
title: 一般的な型検査のアーキテクチャ図
---

<h1>一般的な型検査のアーキテクチャ</h1>
<img src="/figures/typical_architecture.png" alt="一般的な型検査のアーキテクチャ図">

<div
  v-click="1"
  v-motion
  :initial="{ x: -100 }"
  :enter="{ x: 200, y:-200, transition: 'easeInOut' }"
  class="inline-flex flex-col p-16 text-lg bg-white shadow-md rounded-lg"
>
  <span><code>tsc</code> では</span>
  <span>
    <span v-mark="{ at: 2, color: 'red', type: 'circle' }">synthesis & checking</span>
    が
  </span>
  <span>ボトルネックである</span>
</div>
