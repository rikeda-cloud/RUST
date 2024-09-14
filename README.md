# WebCameraStreamingServer

## 使い方
1. 必要なパッケージのインストールや環境変数の設定を行う
```
// Debian系の場合
RUN apt-get update && apt-get install -y \
	llvm-dev \
	clang \
	libclang-dev \
	libopencv-dev \
	pkg-config

export PKG_CONFIG_PATH=/usr/lib/pkgconfig   // 環境に合わせて設定
export LIBCLANG_PATH=/usr/lib/llvm-14/lib/  // 環境に合わせて設定
export DEV_NUMBER=14                        // Webカメラのデバイスナンバーを設定。この環境変数が無い場合は0を使用
```
2. リポジトリのclone && プロジェクトのルートへ移動
3. build (`cargo build --release`)
4. バイナリファイルの実行 (`./target/release/frame`)
5. ブラウザで `http://localhost:8080` にアクセス
6. GUIで画像処理のノード間をつなぎ、カメラまでつなぐとストリーミング映像に画像処理が適応される(画像処理のチェーンは複数つなぐことが可能)

## 使用可能な画像処理機能
* `canny` -> canny法を用いたエッジ検出
* `binary` -> 画像を二値化
* `face` -> 画像から顔を検出し、枠で囲む
* `white_balance` -> 光の色合いを補正
* `superpixel` -> 画像セグメンテーション
* `haar_like` -> 画像の白黒差が最も激しい箇所を抽出
* `removed_red` -> 画像のREDチャネルを0に変換
* `removed_green` -> 画像のGREENチャネルを0に変換
* `removed_BLUE` -> 画像のBLUEチャネルを0に変換
* `text` -> 画像に写る文字列を検出
* `gray` -> grayscaleに変換
* `reverse` -> 画像の左右を反転
* `eye` -> 画像から目を検出し、枠で囲む
