<!--suppress HtmlDeprecatedAttribute -->
<div align="center">

![Gejang Logo][gejang-logo]

### Gejang

A free and <ins>⚠️ incomplete ⚠️</ins> UCI chess engine.

[![GitHub Release][github-release]][latest-release]
[![GitHub Actions][github-actions-workflow-status]][github-actions]

</div>

## Requirements

- [Rust][rust-install]

## Installation

```shell
cargo install --git https://github.com/yoonthegoon/gejang.git
```

## Usage

### ⚠️ Warning ⚠️

Gejang is incomplete and not yet ready for use.
This section refers to how Gejang will be used once it is complete.

---

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

> position startpos moves e2e4 e7e5
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


[gejang-logo]: /assets/logo.svg

[github-release]: https://img.shields.io/github/v/release/yoonthegoon/gejang?include_prereleases

[latest-release]: https://github.com/yoonthegoon/gejang/releases/latest

[github-actions-workflow-status]: https://img.shields.io/github/actions/workflow/status/yoonthegoon/gejang/rust.yml

[github-actions]: https://github.com/yoonthegoon/gejang/actions

[rust-install]: https://www.rust-lang.org/tools/install

[new-issue]: https://github.com/yoonthegoon/gejang/issues/new/choose

[new-discussion]: https://github.com/yoonthegoon/gejang/discussions/new/choose

[chess-programming-wiki]: https://www.chessprogramming.org/Main_Page

[chess-coding-adventure]: https://github.com/SebLague/Chess-Coding-Adventure

[stockfish]: https://github.com/official-stockfish/Stockfish

[chess-engine-testers]: https://discord.gg/EN25UBJ8C5

[gpl-v3-logo]: /assets/gpl-v3-logo.svg

[license]: LICENSE
