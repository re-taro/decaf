---
marp: true
paginate: true
theme: re-taro
math: katex
header: 静的型付けの意味における健全性に焦点を当てた TypeScript 型検査機
footer: 2024-01-21 | 卒業研究発表
---

<!-- _class: title -->

# 静的型付けの意味における健全性に焦点を当てた TypeScript 型検査機

## 糸川 倫太朗 / 青山研究室

---

## 背景 | TypeScript とは

Microsoft によって開発された ECMAScript 5 を拡張したプログラミング言語

- 型アノテーション、型エイリアス、関数オーバーロードのサポート
- 静的型検査を通じてプログラムの誤りを検出可能
- `any` 型により、型チェックを回避可能
  - JavaScript からの移行を容易にする柔軟性を提供
- 漸進的型付け（Gradual Typing）を謳っている

---

## 背景 | Gradual Typing の基準

- 型アノテーションが完全なプログラムは静的型付けシステムと同じ挙動
- 型アノテーションが全て `?` 型のプログラムは動的型付けシステムと同じ挙動
- 型エラーはランタイムで必ず検出可能
- 型キャストにおけるランタイム型エラーの防止
- 型アノテーションを減らしても静的検査は成功し、動作は変わらない

---

## 背景 | Gradual Typing の基準

TypeScript は Gradual Typing の基準を満たしていない

- 健全性: 型エラーがランタイムで検出されない

---

## 研究の目的

TypeScript の柔軟性と静的型検査による安全性を両立させる

- 「省略された型アノテーションを補完した上での静的型検査」という柔軟性を実現
- 健全性を保つこと

---

## decaf の特徴

- 型アノテーションの省略を許容するも、型を補完して静的型検査を行う
  - 依存型を用いた型推論
- 実行時に起こり得る挙動を、型検査機によって静的に検出
- Literal widening が起こらない
  ```ts
  const one = 1;
  const two = 2;
  const four: 4 = one + two; // Error: Type 3 is not assignable to type 4
  ```

---

## 型の取り扱い

decafは型を**その値が取りうる可能性の集合**として捉える

```ts
async function f(name: string) {
	const response = await fetch(`/post/${name}`).then(res => res.json());
	if (response.data instanceof Date) {
		return response.data;
	}
	else {
		return response.data;
	}
}
```

---

```ts
async function f(name: string) {
	const response = await fetch(`/post/${name}`).then(res => res.json());
	if (response.data instanceof Date) {
		return response.data;
	}
	else {
		return response.data;
	}
}
```

- `response` の型は宣言時は、`any` であり、型の全集合である
- 条件 `response.data instanceof Date` が真である場合、`response.data` の型は `Date` である
- 条件 `response.data instanceof Date` が偽である場合、`response.data` の型は `Date` の補集合 である

---

## 型の取り扱い

構文解析木を走査しながら型の集合を広げていくことで、
**実行時に起こりうるすべての可能性を型チェック時に考慮できる**

### 従来の型検査

分岐の評価は、実行されるかされないかのみを考慮している

### decaf の型検査

分岐を非決定論的に扱い、すべての分岐を評価し、型を拡大する

---

## 実験の概要

decaf はより厳格な型システムを実現するために tsc よりも多くの経路を型検査する

そのため、decaf は tsc よりも型検査に時間がかかると予想される

---

## 実験の内容

decaf のテストスイートをソースコードに落とした `simple.tsx` と、それの 10 回、20 回、40 回結合した `middle.tsx`、`complex.tsx`、`very_complex.tsx` の計 4 つのファイルを用意した

それぞれに対して decaf と tsc で 20 回型検査をし、その検査速度の平均を比較した

---

## 結果

**表: decaf と tsc の平均の型検査時間比較**

| **n**     | 1      | 10     | 20     | 40     |
| --------- | ------ | ------ | ------ | ------ |
| **decaf** | 19.47  | 162.54 | 484.79 | 1610   |
| **tsc**   | 272.78 | 323.83 | 371.18 | 451.96 |

プログラムのサイズが大きくなると decaf の型検査速度は遅くなる

---

## 考察

**tsc:** 計算量はプログラムのサイズ$n$に対して$O(n)$の傾向を示している（$y = 4.52n + 275$, $R^2 = 0.993$）

**decaf:** 計算量はプログラムのサイズ$n$に対して$O(n^2)$の傾向を示している（$y = 0.799n^2 + 7.58n + 8.68$, $R^2 = 1.00$）

--> decaf の型検査速度は tsc よりも遅い場合があるが，それは decaf の型検査速度はプログラムのサイズに対してより高次の計算量を持つことが原因

---

## 考察

### 高次の計算量を持つ理由

tsc は、TypeScript のパースと型検査を同じパスで行うため、型検査の計算量はプログラムのサイズに対して線形である

decaf は、型検査を独立したパスで行うため、型検査の計算量はプログラムのサイズに対して二次である

---

## 考察

tsc: $y = 4.52n + 275$, $R^2 = 0.993$

decaf: $y = 0.799n^2 + 7.58n + 8.68$, $R^2 = 1.00$

y 切片はそれぞれのコンパイラの初期化時間を表しており，decaf は tsc よりも初期化時間が短いことがわかる

--> simple.tsx のような 2000 行程度の小規模なプログラムにおいては，decaf は tsc よりも高速に型検査でき，一般的なケースで decaf が優位性を持つ

---

## まとめ

型検査を通して健全性を保証するための TypeScript 型検査機を実装した

プログラムの挙動を静的に解析できる型システムを構築したことで、ランタイムエラーを事前に検出できるようになった

TypeScript では静的解析の際、検知できずにランタイムエラーとして現れる問題を、decaf では静的解析時に検知できるようになった
