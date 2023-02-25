# Dive Into Rust: A Crash Course

![Github stars](https://img.shields.io/github/stars/abcdabcd3899/Dive-Into-Rust.svg)
![Github language](https://img.shields.io/badge/language-Rust-green.svg)
![License](https://img.shields.io/github/license/abcdabcd3899/Dive-Into-Rust.svg)
![Stars](https://img.shields.io/github/forks/abcdabcd3899/Dive-Into-Rust.svg)

> good artists copy; great artists steal. - Pablo Picasso

## Introduction

The project's goal is to delve into rust features for languages and standard libraries. Coding is preferable to speaking.

## Installation

* Rust Installation

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Rust Codes Installation

```shell
rustup update
rustup toolchain install stable-aarch64-unknown-linux-gnu
rustup component add rust-src rustc-dev llvm-tools-preview # rust source code
```

You may read the source code as long as you have rust source code and rust-analyzer installed.

## How to run the project

I use vscode to create, build, execute, and clean the project in Ubuntu.
To make the project easier to use, I created the tasks.json and launch.json files. Using the keyboard shortcut "command + shift + B," you may choose the different commands in tasks.json. To debug the various source codes, press the F5 button.

## Replace Cargo Mirror

```shell
vim ~/.cargo/config
```

```shell
[http]
check-revoke = false

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
# registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

## Outline of Features

* Basic Grammar [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/basic_grammar_demo)
* Composition Data Type
  * struct [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/struct_demo)
  * enum [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/enum_demo)
* Module System
  * [Architecture 1](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/module_demo1)
  * [Architecture 2](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/module_demo2)
  * [Architecture 3](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/module_demo3)
  * [Workspace](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/workspace_demo)
* Generic and Trait [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/generic_trait_demo)
* Lifetime [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/lifetime_demo)
* Automation Tests
  * unit test [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/basic_grammar_demo)
  * integration tests [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/autotest_demo/tests)
* Smart Pointer [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/smart_pointer_demo)
* The Difference of ref_& [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main//ref_ampersand_demo)
* Active Mutable Borrow and Immutable Borrow [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/active_borrow_or_not_demo)
* Double Linked List [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/double_linked_list_demo)
* Closure [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/closure_demo)
* Memory Management [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/memory_management_demo)
* Concurrency Programming
  * Message Passing [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/message_passing_demo)
  * Shared Memory [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/shared_memory_demo) [Mutex Dead Lock](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/dead_lock_demo) [Condition Variable Dead Lock](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/condition_variable_demo)
* Everything is Pattern [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/match_demo)
* Asynchronous Programming [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/asynchronous_demo)
* Unsafe Programming and Foreign Function Interface (FFI) [Code](https://github.com/abcdabcd3899/Dive-Into-Rust/tree/main/unsafe_demo)

## crates

* Concurrency Programming
  * [crossbeam](https://github.com/crossbeam-rs/crossbeam)
  * [Condvar](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html)
  * [no_deadlocks](https://crates.io/crates/no_deadlocks)
  * [cooptex](https://github.com/shelbyd/cooptex)

## Contribute

Please open pull requests if you want to add new features.

## References

1. [Standard Library](https://doc.rust-lang.org/std/)
2. [The Rust Programming Languages](https://kaisery.github.io/trpl-zh-cn/)
3. [Rust by Example](https://rustwiki.org/zh-CN/rust-by-example/index.html)
4. [tikv](https://github.com/tikv/tikv)
5. [Learn Rust by writing Entirely Too Many linked lists](https://github.com/rust-unofficial/too-many-lists)
6. [libs.rs](https://lib.rs/)
7. [crates.io](https://crates.io/)

## Agreement

<img src='https://www.gnu.org/graphics/gplv3-127x51.png' width='127' height='51'/>

More information [document of agreement](/LICENSE)

<img src='https://raw.githubusercontent.com/EyreFree/EFArticles/master/res/cc-by-nc-nd.png' width='145.77' height='51'/>

[Attribution - Non-commercial - No interpretation](http://creativecommons.org/licenses/by-nc-nd/3.0/cn/)