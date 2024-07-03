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
