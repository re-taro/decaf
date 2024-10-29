---
title: 現代 web 開発における TypeScript の重要性
---

<h1>現代 web 開発における TypeScript の重要性</h1>

<h2 v-click="1">TypeScript とは</h2>
<div v-click="1" class="my-4">
  <span class="text-lg">Microsoft が開発している JavaScript に静的型付けとクラスベースオブジェクト指向を加えたスーパーセット</span>
</div>

<h2 v-click="2">web 開発において無くてはならない物</h2>
<div v-click="2" class="flex flex-col my-4">
  <span class="text-lg">プログラミングをするうえでほぼ必ずと言って必要な Language Server や linter</span>
  <span class="text-lg">これらは TypeScript Compiler （<code>tsc</code>）が内部で使用されている</span>
</div>

<h2 v-click="3"><code>tsc</code> の課題</h2>
<div v-click="3" class="my-4">
  <span v-mark="{ at: 4, color: 'red', type: 'underline' }" class="text-lg"><code>tsc</code> による解析と検査はコストが高い</span>
</div>
