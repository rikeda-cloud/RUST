# Tuning

## RUST
### 使い方
1. `cargo new PROJECT_NAME` RUSTプロジェクトを作成<br>(`cargo new PROJECT_NAME --lib` ライブラリを作成するプロジェクトを作成)
2. `git init && git add * && git commit -m "[init]"` 最初のコミットを作成
3. `git remote add origin gitのリポジトリURL && git push -u origin master` リポジトリの初期化
4. `cd PROJECT_NAME && cargo run` プロジェクトを開始
5. `cargo check` コンパイルの確認
6. `cargo build` ビルドのみ実行(`cargo build --release` 最適化を行いビルドする)
7. `cargo run` ビルドと実行
8. `cargo test` 記述されたテストの実行
9. `rustc *.rs` コンパイルする

### memo
* RUSTでは整数と浮動小数点数の四則演算はできない(1 + 0.1)。整数を浮動小数点リテラルに変換すれば可能(1. + 0.1)
* `let val = 1;`や`let val; val = 1;`で宣言・初期化する
* `let val = 1; val = 2`;はコンパイルエラーになる。`let mut val = 1; val = 2;`ならコンパイル可能
* mutableで宣言した変数を変更しない場合にwarningを出す。`#[allow(unused_mut)]`を該当の関数の前に記述するとwarnが出なくなる
* 変数名をアンダーバー始まりにするとunused_variablies警告が出ない
* 変数名をアンダーバーのみにするとその変数を評価した場合にコンパイルエラーとなる
* `1 == true` のような型の異なる比較はコンパイルエラー
* 変数の初期化後に型の異なる値を代入するとコンパイルエラー(回避策として、同じ変数名で宣言し直す(シャドーイングという))
```
let mut a = 1; // 整数型、mutable
a = 2;
// a = 3.33 コンパイルエラー
let a = 3.33; //　浮動小数型、immutable
```
* RUSTでは標準ライブラリはインクルードせずに使用可能
```
let s = "abc";
print!("len({}) = {}", s, str::len(s)); // プロシージャ形式
print!("len({}) = {}", s, s.len()); // オブジェクト指向形式
```
