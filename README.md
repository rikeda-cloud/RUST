# RUST
## 使い方
1. `cargo new PROJECT_NAME` RUSTプロジェクトを作成<br>(`cargo new PROJECT_NAME --lib` ライブラリを作成するプロジェクトを作成)
2. `git init && git add * && git commit -m "[init]"` 最初のコミットを作成
3. `git remote add origin gitのリポジトリURL && git push -u origin master` リポジトリの初期化
4. `cd PROJECT_NAME && cargo run` プロジェクトを開始
5. `cargo check` コンパイルの確認
6. `cargo build` ビルドのみ実行(`cargo build --release` 最適化を行いビルドする)
7. `cargo run` ビルドと実行
8. `cargo test` 記述されたテストの実行
9. `rustc *.rs` コンパイルする

## 命名規則(RUSTは言語レベルで命名規則が指定されている)
1. 定数名はアッパースネークケース
2. 型名はアッパーキャメルケース
3. 変数名やプリミティブ型、フィールド識別子(構造体内のメンバ変数名)はスネークケース

## memo
### 基本
* RUSTでは整数と浮動小数点数の四則演算はできない(1 + 0.1)。整数を浮動小数点リテラルに変換すれば可能(1. + 0.1)
* `let val = 1;`や`let val; val = 1;`で宣言・初期化する
* `let val = 1; val = 2`;はコンパイルエラーになる。`let mut val = 1; val = 2;`ならコンパイル可能
* mutableで宣言した変数を変更しない場合にwarningを出す
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
* 変数のアドレスを表示
```
let b1 = true;
let b2 = true;
let b3 = true;
print!("{:p} {:p} {:p}", &b1, &b2, &b3);
```

### データ型
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
* 列挙型は`enum EnumName {XXX, YYY, ZZZ}` で宣言する
* switch文のように使用可能なmatch分がある。列挙型の比較に使用(列挙型は==演算子で比較不可能)
```
match e {
    EnumName::XXX => print!("XXX"),
    EnumName::YYY => print!("YYY"),
    EnumName::ZZZ => print!("ZZZ"),
    _ => {}, // 全ての列挙型を書くのが冗長なときは全てとマッチする_でdefaultに似た処理を記述可能
}

let val = 3;
match val {
    1 => print!("1"),
    2 => print!("2"),
    _ => print!("other"),
}
// 複雑な enum & match
#[allow(dead_code)]
enum E {V1(i8), V2(char, char), V3, }
let val = E::V2('a', 'b');
match val {
    E::V1(1) => print!("v1 1"),
    E::V1(_) => print!("v1 _"),
    E::V2('b', _) => print!("v2 b _"),
    E::V2(_, 'b') => print!("v2 _ b"),
    E::V2(_, _) => print!("v2 _ _"),
    E::V3 => print!("v3"),
}
// 変数を含むmatch文
let val = E::v1(42);
match val {
    E::v1(1) => print!("v1 1"),
    E::v1(n) => print!("v1 {}", n),
    _ => print("_"),
}
```
* タプル
```
let data: (i32, f64, char) = (32, 64., 'c');
print!("{} {} {}", data.0, data.1, data.2);
print!("{:?}", data);
// let i: usize = 1;
// print!("{}", data.i); タプルは変数インデックスでアクセス不可能のためコンパイルエラー
```
* 構造体
```
struct Data {
    v1: i32,
    v2: f64,
    v3: char,
}
let data = Data {
    v1: 32,
    v2: 64.,
    v3: 'c',
}
print!("{} {} {}", data.v1, data.v2, data.v3);
```
* タプル構造体
```
struct Data (i32, f64, char);
let data = Data(32, 64., 'c');
print!("{} {} {}", data.0, data.1, data.2);
```

### 関数
* RUSTでは関数内で関数を定義することが可能
* 関数の定義は現在のスコープ無いにあれば呼び出しよりも後に記述されていても良い
* 関数の引数の型は必須(変数の型は型推論により省略することが可能)
* 関数の最後に記述された値がreturnされる。値がない場合は空タプルがreturnされる
* 関数の最後にreturnを置くことも可能だが、RUSTでは良くないスタイルとされる
* 関数の途中で処理を終了する目的でreturn文を使用する
* 関数の返り値にはタプル、配列、ベクタ等も使用可能
* 関数の引数は値渡しと参照渡しが使用可能
```
fn main() {
    let mut v = 1;
    println!("{}", val(v));
    reff(&mut v);
    println!("{}", v);
}
fn val(v: i8) -> i8 {v + 1}
fn reff(v: &mut i8) {*v += 1;}
```
* ジェネリック関数(C++では関数テンプレート)
```
fn main() {
    let num1 = 1.1;
    let num2 = 2.1;
    println!("{}", f('1', num1, num2));
    println!("{}", f::<f32>('2', num1, num2));
    println!("{}", f::<f64>('a', num1, num2));
}

fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == '1' {return num1;}
    num2
}
```
* ジェネリック構造体(C++ではクラステンプレート)
```
fn main() {
    let car = Car::<i8, f32> { //　構造体のインスタンス化時は型推論を使用することも可能
        name: 'c',
        val1: 1,
        val2: 2.2,
    };
    println!("{} {} {}", car.name, car.val1, car.val2);
}
struct Car<T1, T2> {
    name: char,
    val1: T1,
    val2: T2,
}
```
* ジェネリックな列挙型
```
enum E<Type1, Type2> {
    A(Type1),
    B(Type2),
    C,
}
```
* Option列挙体(組み込みでSomeとNoneを持つ列挙体が標準ライブラリで提供されている)
```
fn main() {
    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        match item {
            Some(number) => print!("{}, ", number),
            None => print!("#, "),
        }
    }
}
let mut v = vec![11, 22, 33];
for _ in 0..v.len() {
    print!("{}, ", v.pop().unwrap())
}

```
* Result列挙体(組み込みでOkとErrを持つ列挙体が標準ライブラリで提供されている)
```
fn divide(num: f64, den: 64) -> Result<f64, String> {
    if den == 0. { Err(format!("Divide zero")}
    Ok(num / den)
}

match divide(2., 0.) {
    Ok(val) => ...,
    Err(msg) => ...,
}

let result: Result<f64, String> = divide(2., 0.);
if result.is_ok() {
    println!("SUCCESS: {}", result.unwrap());
} else {
    println!("ERROR: {}", result.is_err());
}
```

## メモリ割り当て
* 静的割り当て・・・staticキーワードを使用。変数の型の指定が必須。値の変更不可
* スタック割り当て・・・letキーワードを使用。数MB程度。プリミティブ型や配列などのコンパイル時にサイズが判明するもののみでベクタのようなものはスタックに割り当てることは不可能
* ヒープ割り当て・・・`Box::new`によって割り当てる。メモリはスコープを抜けた時に自動的に開放される。freeする関数は用意されていない！！

## use
```
use std::mem; // C++のusing std;と同様
use std::mem::*; // ワイルドカードでまとめることも可能
mem::size_of<i8>();
```

## クロージャ
* 型推論可能・１つの式が本体・クロージャ外のスコープの変数にアクセス可能なインライン無名関数
```
let mut ary = [2, 3, 1, 0];
ary.sort_by(|a, b| b.cmp(a)); // |引数, 引数| 式　のような形式で記述する
ary.sort_by(|a: &i8, b: &i8| -> std::cmp::Ordering {b.cmp(a)}); // |引数: 型, 引数: 型| -> 戻り値 {式}　のような形式でもOK
print!("{:?}", ary);
```

## 文字列型
* str型・・・文字列のリテラルを使用する時の型。内部では文字列へのポインタとサイズが格納されている。Cのように終端文字はない
* String型・・・動的に操作可能な文字列型。C＋＋のstring型と同様
```
// 空のString変数を作成する方法は String::new(), String::from(""), format!("") などもある
let mut s: String = "".to_string(); // &strからString型に変換する処理
s.push('1'); // 文字を追加
s.push('2');
s.insert(0, '0'); // 文字を位置を指定して挿入
s.pop(); // 末尾の文字を削除
s.remove(0); // 位置を指定して文字を削除
println!("{}", s);
```
* 文字列の連結
```
fn main() {
    let s1: String = "123".to_string();
    let s2: &str = "abc";
    println!("{}", format!("{}{}", s1, s1));
    println!("{}", format!("{}{}", s1, s2));
    println!("{}", format!("{}{}", s2, s1));
    println!("{}", format!("{}{}", s2, s2));
}
```
* += 演算による文字列の連結
```
fn main() {
    let mut s1: String = "123".to_string();
    let s2: &str = "abc";
    let s3: String = "000".to_string();
    s1 += s2;
    println!("{}", s1);
    s1 += &s3; // String同士を連結する場合は String += &String;にする
    println!("{}", s1);
}
```
## スライス
```
fn main() {
    let ary = [1,3,4,6,2,9,0];
    println!("min = {}", min(&ary[2..5]));
    println!("min = {}", min(&ary[0..6]));
    println!("min = {}", min(&[1,3,4,6,2,9,0][0..6])); // 上の式と同じ
}

fn min(ary: &[i8]) -> i8 {
    let mut mini = ary[0];
    for i in 1..ary.len() {
        if ary[i] < mini {
            mini = ary[i];
        }
    }
    mini
}
```
* 文字列のスライス
```
fn main() {
    let s = "my name is".to_string();
    println!("{}", &s[3..7]);
}
```
* 下限、上限のみのスライス
```
fn main() {
    let ary = [1, 2, 3, 4, 5];
    let half_size = ary.len() / 2;
    let ary_to = &ary[..half_size];
    let ary_from = &ary[half_size..];
    println!("{:?}", ary_to);
    println!("{:?}", ary_from);
}
```
## イテレータ
* 文字列のイテレータ
```
fn main() {
    let s = "あいうえお234カキクケコ";
    print_chars1(s);
    print_chars2(s);
}
fn print_chars1(s: &str) {
    let mut iter = s.chars();
    loop {
        match iter.next() {
            Some(c) => print!("{} ", c),
            None => {
                println!("END");
                break;
            }
        }
    }
}
fn print_chars2(s: &str) {
    for c in s.chars() {
        print!("{} ", c);
    }
}
```
* 数値型のイテレータ
```
fn main() {
    let vector = vec![10, 20, 30, 40, 50];
    let array = [10, 20, 30, 40, 50];
    let range = &vector[0..vector.len()];
    print_iter(&vector);
    print_iter(&array);
    print_iter(&range);
}

fn print_iter(items: &[i32]) {
    for item in items.into_iter() {
        print!("{} ", item);
    }
    println!();
}
```
* iterator adapter(イテレータを受け取り、イテレータを返す関数)
```
fn main() {
    let ary = [1, 2, 10, 0];
    for item in ary.into_iter().map(|x| x + 1) {
        print!("{} ", item);
    }
    println!();
    for item in ary.into_iter().filter(|x_ref| 1 <= *x_ref) {
        print!("{} ", item);
    }
    println!();
    for (i, item) in ary.into_iter().enumerate() {
        print!("[{},{}] ", i, item);
    }
    println!();
}
```
* iterator consumer(イテレータを受け取り、イテレータを返さない関数)
```
fn main() {
    let ary = [1, 2, 0, 3];
    let string: String = "あいうえおカキクケコ".to_string();
    let strings = [
        "abc".to_string(),
        "Abc".to_string(),
        "".to_string(),
        "123".to_string(),
    ];
    println!("{}", ary.iter().any(|x| *x < 0));
    println!("{}", ary.iter().all(|x| 0 <= *x));
    println!("count = {}; len = {}", string.chars().count(), string.len());
    println!("{}", ary.iter().sum::<i32>());

    match ary.iter().max() {
        Some(n) => println!("max = {}", n),
        None => print!("empty"),
    }
    match ary.iter().min() {
        Some(n) => println!("min = {}", n),
        None => print!("empty"),
    }
    match strings.iter().max() {
        Some(string) => println!("max = \"{}\"", string),
        None => print!("empty"),
    }
    match strings.iter().min() {
        Some(string) => println!("min = \"{}\"", string),
        None => print!("empty"),
    }
}
```
* collectによる変換
```
fn main() {
    let int_ary = [1, 2, 3];
    let char_ary = ['1', '2', '3'];
    let vector: Vec<i32> = int_ary.into_iter().collect();
    let string: String = char_ary.into_iter().collect();
    let char_vector: Vec<char> = string.chars().into_iter().collect();
    let byte_vector: Vec<u8> = string.bytes().into_iter().collect();
    let double_int_ary: Vec<i32> = int_ary.into_iter().filter(|x| *x > 0).map(|x| x * 2).collect(); // iterator chain
    println!("{:?}", vector);
    println!("{:?}", string);
    println!("{:?}", char_vector);
    println!("{:?}", byte_vector);
    println!("{:?}", double_int_ary);
}
```

## コマンドライン引数
```
fn main() {
    let args: std::env::Args = std::env::args();
    for arg in args {
        print!("[{}]", arg);
    }
    println!();
    for arg in std::env::args() {
        print!("[{}]", arg);
    }
}
```

## exitステータス
```
fn main() {
    std::process::exit(1);
}
```

## 環境変数
```
fn main() {
    for var in std::env::vars() {
        println!("[{}] = [{}]", var.0, var.1);
    }
    println!();

    println!("{:?}", std::env::var("abc"));
    std::env::set_var("abc", "abc");
    println!("{:?}", std::env::var("abc"));

    print!(
        "{}",
        if std::env::var("OK").is_ok() {
            "OK"
        } else {
            "NO"
        }
    );
    std::env::set_var("OK", "OK");
    print!(
        ", {}",
        match std::env::var("OK") {
            Ok(value) => value,
            Err(err) => format!("Error: {}", err),
        }
    );
}
```

## Result型の省略記法(Result型を返す関数の呼び出し元に?を置くと正常の場合は処理を続け、異常の場合は処理を行わない)
```
fn main () {
    print!("{:?} ", f1(10));   
    print!("{:?} ", f1(0));   
    print!("{:?} ", f1(-1));   
}

fn f1(x: i32) -> Result<i32, String> {
    Ok(f2(x * 2)? as i32 * 3)
}
fn f2(x: i32) -> Result<i32, String> {
    if x >= 0 {
        Ok(x * 4)
    } else {
        Err("Error!!".to_string())
    }
}
```

## 型の変換
```
fn main() {
    // T -> String
    let int_str: String = 42.to_string();
    let float_str: String = 4.2.to_string();
    let bool_str: String = true.to_string();
    println!("{} {} {}", int_str, float_str, bool_str);

    // String -> T
    println!("{:?}", "true".parse::<bool>());
    println!("{:?}", "42".parse::<i32>());
    println!("{:?}", "4.2f".parse::<f32>());
}
```

## ファイルへの書き込み・読み込み
```
fn main() {
    let filename: String = "data1.txt".to_string();
    write_to_file(&filename, "abc".as_bytes()).unwrap();
    println!("{}", read_from_file(&filename).unwrap());
}

fn write_to_file(filename: &String, data: &[u8]) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut file = std::fs::File::create(filename).unwrap();
    file.write_all(data)
}

fn read_from_file(filename: &String) -> Result<String, String> {
    use std::io::Read;
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err("Error".to_string()),
    }
}
```
