# ビルドステージ
FROM rust:latest AS build

# opencv関連のパッケージのインストール
RUN apt-get update && apt-get install -y \
	llvm-dev \
	clang \
	libclang-dev \
	libopencv-dev \
	pkg-config && \
	rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib/

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

# # 本番用ステージ
FROM rust:latest AS production
COPY --from=build /usr/src/myapp/target/release/frame /usr/local/bin
ENTRYPOINT ["/usr/local/bin/frame"]
