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
* mutableで宣言した変数を変更しない場合にwarningを出す。
* エラーやワーニングを制御可能
```
#[deny(unused_mut)]
#[warning(unused_mut)]
#[allow(unused_mut)]
```
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
* 無限ループはwhile文ではなく、loop文で記述する
* for文は下記のような記法となる。また、右に指定した範囲を含む・含まないを制御できる
```
for i in 1..5 {print!("{}", i);} // 1,2,3,4
for i in 1..=5 {print!("{}", i);} // 1,2,3,4,5
```
* for文に評価式を指定する場合の挙動(for文の上限・下限を評価、ループの実行回数はこの評価された値に依存する)
```
let mut limit = 4;
// 1 ~ 6
for i in 1..limit + 2 {
    print!("({},{})", i, limit); // "(1,4)(2,3)(3,2)(4,1)(5,0)"
    limit -= 1;
}
print!("{}", limit); // "-1"
```
* 配列の宣言は定数でなければならない(`let n = 3; let ary = [0; n]`はエラー)
* 空の配列やベクタを宣言する場合、 `let ary = [0, 0];`のように初期値に適当な値、要素数は0にする
* 配列やベクタは `let ary = [1,2,3]; print!("{:?}", ary);` によって一度に出力可能
* 配列の代入演算はコピー。ベクタの代入演算はコピーではなく移動
```
// 配列の場合
let mut vec1 = [1,2,3];
println!("{:?}", vec1);
let vec2 = [3,4,5];
println!("{:?}", vec2);
vec1 = vec2;
println!("{:?}", vec1);
println!("{:?}", vec2);
```
```
// vectorの場合
let mut vec1 = vec![1,2,3];
println!("{:?}", vec1);
let vec2 = vec![3,4,5];
println!("{:?}", vec2);
vec1 = vec2;
println!("{:?}", vec1);
println!("{:?}", vec2); // vec2は移動しているのでエラー
```
* 型(オーバーフローするとコンパイルエラーになる？？)
```
// i = 符号あり整数, u = 符号なし整数
let a: i8 = 1;
let b: i16 = 1;
let c: i32 = 1; // 型指定されず、型推論しても結論が出ない整数型はi32がデフォルトで使用される
let d: i64 = 1;
let e: i128 = 1;
let f: u8 = 1;
let g: u16 = 1;
let h: u32 = 1;
let i: u64 = 1;
let j: u128 = 1;
let k: isize = 1;
let l: usize = 1; // 配列やベクタのインデックスに使用可能なのはusize型のみ
let m: f32 = 1.;
let n: f64 = 1.; // 型指定されず、型推論しても結論が出ない浮動小数点型はf64がデフォルトで使用される
let o: bool = true; // true or false
let p: char = 'a'; // C言語と異なり4バイトの型。Unicode文字列を格納できる
let q: () = (); // 空タプル型。C言語のvoid型に近い
let r: [char; 3] = ['a', 'b', 'c'];
let s: [f32; 10] = [0.; 10];
let t: Vec<char> = vec!['a', 'b', 'c'];
let u: Vec<i32> = vec![0; 400];

print!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {:?} {:?} {:?} {:?} {:?}", a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u);
// 'var as type' という構文で明示的に型変換可能
// 変換後の型で表現できない範囲の値である場合はコンパイルエラー
print!("sum = {}", a as i8 + f as i8 + n as i8);
```
* 定数型はconstで宣言する`const N: usize = 1;`
