# Bevy Tutorial Minesweeper

see original tutorial article and repository

- https://dev.to/qongzi/bevy-minesweeper-introduction-4l7f
- https://gitlab.com/qonfucius/minesweeper-tutorial/

## to Run

```
trunk serve
```

## コンパイルの使い分け

```
#開発ビルド(デバッグ ALL ON)
cargo build

# リリースビルド
cargo build --release

# リリースビルド(開発用デバッグツールON)
cargo build --release -C debug_assert

#
cargo test

#
cargo bench
```

## Files

- src
- assets
- dist
- scripts
  - prepare_build_install_package -> to build
- .cargo
  - config.toml.example -> if you want use it, copy this to .cargo/config.toml
- Makefile.toml -> for `cargo make` command
- index.html -> for Browser

## Structure
