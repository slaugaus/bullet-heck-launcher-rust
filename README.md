# Overview

Killing 2 birds with 1 stone by learning the Rust language (mainly syntax) and a new GUI markup language, then rewriting the launcher program for [Bullet Heck](https://github.com/slaugaus/bullet-heck-again).

Like "Bullet Heck Again," my goal was to replicate the behavior of the original program, so it's largely similar (in appearance) to the original. Currently, only the Play and Credits buttons are fully functional; the Settings window is there visually, but has no useful behavior. (Bullet Heck Again doesn't support reading the settings from a file anyway, so I deprioritized file I/O until I could get the other end of it working.)

[Software Demo Video](https://youtu.be/dUTj8kmCuus)

# Development Environment

IDE: VS Code with a bunch of extensions, including  [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [Slint](https://marketplace.visualstudio.com/items?itemName=Slint.slint)

Language(s): Rust 1.70.0 with crates:
* slint 1.0.2
* slint-build 1.0.2
* toml 0.7.4 (currently unused)
* msgbox 0.7.0
... and the Slint markup language

# Useful Websites

- [Rust Book with quizzes](https://rust-book.cs.brown.edu/)
- [Slint Language Docs]()
- [slint crate docs](https://slint-ui.com/releases/1.0.2/docs/rust/slint/)
- [toml crate](https://crates.io/crates/toml)
- [Nuitka User Manual (for compiling the game)](https://nuitka.net/doc/user-manual.html)

# Future Work

- Functional settings - TOML file R/W
- Better visuals?
