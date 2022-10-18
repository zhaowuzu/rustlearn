## 安装Target
1.linux64位安装：
   + `rustup target add x86_64-unknown-linux-musl`
   + `rustup target list`

2.可支持的平台汇总：<https://doc.rust-lang.org/nightly/rustc/platform-support.html#platform-support>
   
## 交叉编译配置
路径：/.cargo/config.toml
内容：
```
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
rustflags = ["-C", "linker-flavor=ld.lld"]
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```
## 生成对应平台的可执行文件
项目目录执行：
1. windows
   + release: cargo build --release --target=x86_64-pc-windows-msvc
   + debug：cargo build --target=x86_64-pc-windows-msvc
2. linux
   + release: cargo build --release --target=x86_64-unknown-linux-musl
   + debug：cargo build --target=x86_64-unknown-linux-musl