## ⚠️ Status ⚠️

Gejang is incomplete and not yet ready for use. \
There are still many features that make a chess engine missing. \
The [usage](#usage) section only currently descirbes how Gejang is planned to be used.

---

<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

![Gejang Logo][gejang-logo]

### Gejang

A free UCI chess engine.

[![GitHub Release][github-release]][latest-release]
[![GitHub Actions][github-actions-workflow-status]][github-actions]

</div>

## Requirements

- [Cargo][cargo]

## Installation

```shell
cargo install --git https://github.com/yoonthegoon/gejang.git
```

## Usage

Gejang is normally used with a GUI that supports the UCI protocol.
Refer to your GUI's documentation on how to use a UCI engine.
Use `which gejang` to find the path to the executable.

Gejang can also be used in the terminal by running `gejang` and entering UCI commands.

```shell
$ gejang
> uci
id name Gejang
id author Yunis Yilmaz
uciok

> isready
readyok

> position startpos
> go depth 10
...
bestmove e2e4

> position startpos moves e2e4 e7e5
> go movetime 1000
...
bestmove g1f3

> quit
```

## Support

If you find something isn't working or doesn't seem right, or want to suggest a new feature,
please [open an issue][new-issue].
If you simply have a question, please [open a discussion][new-discussion].

## Contributing

This project is currently in its early stages and not yet open to contributions.
Once the project is more mature, contributions will be welcome.

## Acknowledgment

- [Chess Programming Wiki][chess-programming-wiki]
- [Sebastian Lague's Chess Coding Adventure][chess-coding-adventure]
- [Stockfish][stockfish]
- [Chess Engine Testers Discord Server][chess-engine-testers]

## License

[![GNU GPLv3][gpl-v3-logo]][license]

<!-- markdownlint-enable MD033 MD041 -->

[cargo]:                          https://doc.rust-lang.org/cargo/getting-started/installation.html
[chess-coding-adventure]:         https://github.com/SebLague/Chess-Coding-Adventure
[chess-engine-testers]:           https://discord.gg/EN25UBJ8C5
[chess-programming-wiki]:         https://www.chessprogramming.org/Main_Page
[gejang-logo]:                    /assets/logo.svg
[github-actions]:                 https://github.com/yoonthegoon/gejang/actions
[github-actions-workflow-status]: https://img.shields.io/github/actions/workflow/status/yoonthegoon/gejang/rust.yml
[github-release]:                 https://img.shields.io/github/v/release/yoonthegoon/gejang?include_prereleases
[gpl-v3-logo]:                    /assets/gpl-v3-logo.svg
[latest-release]:                 https://github.com/yoonthegoon/gejang/releases/latest
[license]:                        LICENSE
[new-discussion]:                 https://github.com/yoonthegoon/gejang/discussions/new/choose
[new-issue]:                      https://github.com/yoonthegoon/gejang/issues/new/choose
[stockfish]:                      https://github.com/official-stockfish/Stockfish
