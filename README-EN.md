# RigelA (the name is taken from the Rigel-A of Orion)

English (en) | [简体中文 (zh-CN)](README.md)

<!-- TOC -->
* [RigelA (the name is taken from the Rigel-A of Orion)](#rigela-the-name-is-taken-from-the-rigel-a-of-orion)
  * [Introduction](#introduction)
  * [Build and run](#build-and-run)
  * [Donation Support](#donation-support)
  * [Development documentation](#development-documentation)
  * [Development Contribution](#development-contribution)
  * [agreement](#agreement)
<!-- TOC -->

* [Official Site](http://rigela.site)
* [GitHub](https://github.com/mzdk100/rigela)
* [GitCode](https://gitcode.com/mzdk100/rigela)

## Introduction

This project is a Screen Reader project written in rust![logo](images/logo.ico).
The first letter r in the name is the same as the first letter of rust, and Orion is the best viewing time in December(This project was initiated in December 2023).
Rust focuses on performance, coding efficiency and security, let us imagine!
For the name, please refer
to [Wikipedia’s introduction to Rigel A](https://zh.wikipedia.org/wiki/%E5%8F%83%E5%AE%BF%E4%B8%83)
Regarding creating the rust environment: you can refer to relevant documents. If you are not familiar with the rust
language, it is highly recommended to learn from official documents or authoritative books.

## Build and run

1. Preliminary preparation
   First you need to download the necessary msvc (if you already have Visual Studio on your computer, skip this step),
   you can visit [here](https://visualstudio.microsoft.com/zh-hans/downloads/) to get VS Installer;
   Next we need to have a rust environment. To build the rust environment, you can refer to the introduction
   in [Rust Programming Language] (https://kaisery.github.io/trpl-zh-cn);
   Assuming that you already have the above conditions, before building the program we also need to download the rust
   x86 architecture tool chain and run the following command to install it.
    ```shell
    rustup target add i686-pc-windows-msvc
    rustup update
    ```
2. Clone the repository
    ```shell
    git clone https://gitcode.com/mzdk100/rigela.git
    ```
   or
    ```shell
    git clone https://github.com/mzdk100/rigela.git
    ```
3. Build the debug version and run it
    ```shell
    cd rigela
    cargo mkenv
    cargo dev
    ```
   Among them, mkenv is used to create a dedicated build program. Because there is a need to build 32-bit and 64-bit
   programs at the same time, cargo itself cannot do this.
   Please note: we use `cargo dev` to run instead of `cargo run` otherwise you will encounter errors like (error: only
   one `--target` argument is supported).
   If you insist on using the "cargo run" command, you can add the "--target x86_64-pc-windows-msvc" additional argument
   to achieve this:
    ```shell
    cargo build --target i686-pc-windows-msvc
    cargo run --target x86_64-pc-windows-msvc
    ```
   The first line means building the 32-bit target first, because the second line needs to rely on it. These two lines
   of commands cannot be written in reverse, so to simplify writing, we created a short command `dev`.
4. Build the released version
   The released version of the program has a smaller volume and runs faster than the debug version, but the build speed
   is slower:
    ```shell
    cargo rel
    ```

## Donation Support

This project is free for anyone in any country, but if you can financially support our developers, you can enjoy more personalized services from us.
For example, if you encounter any difficulties while using this software, you can submit an issue, and we will prioritize solving it for you.
Below is the current list of core contributors to this project (sorted alphabetically), and you can use the QR code to pay and contact anyone of them:
- lwboy<liwenboy2008@126.com>: [WeChat Payment QR Code](images/lw_weixin.png), [Alipay Payment QR Code](images/lw_alipay.jpg)
- SmileSky<mzdk100@foxmail.com>: [WeChat Payment QR Code](images/lq_weixin.jpg), [Alipay Payment QR Code](images/lq_alipay.jpg)


## Development documentation

We call on developers who are interested in the open source screen reader project to actively participate. The
development documentation is constantly being improved. If you want to contribute to this project, please refer to the
next section.

1. Development documents provided by this project:
    - [Reference document for screen reader main program module](main/README.md)
    - [Implementation Reference Document for Non Microsoft Official Accessibility Technology](a11y/README)
    - [Proxy module reference document for 32-bit applications](proxy32/README.md)
    - [RigelA resource incremental update reference document](resources/README.md)
2. If you are new to programming, you also need to learn the Rust language and understand the common libraries of Rust:
    - [Rust programming language Simplified Chinese version](https://kaisery.github.io/trpl-zh-cn)
    - [Secret Handbook For Rust Beginners](https://rust-book.junmajinlong.com/)
    - [Rust Language Bible](https://course.rs/about-book.html)
    - [The Rust Reference](https://doc.rust-lang.org/stable/reference/)
    - [Rust for Windows repository](https://github.com/microsoft/windows-rs)
    - [Rust for Windows Documentation](https://microsoft.github.io/windows-docs-rs/)
    - [Rust's asynchronous runtime tokio repository](https://github.com/tokio-rs/tokio)

## Development Contribution

Powerful functions of program are inseparable from rapid function code iteration. We call on capable friends to
participate in the research and development of this project.
If you are a developer, you can fork this repository into your own account, then clone the repository for research and
development, and then submit a request for merging.
If you still don’t know how to get started, here is the [Contribution Guide](CONTRIBUTING.md) we have prepared for you.
We have a long-term planning route, which includes the ideas that all users are most interested in but have not yet been
implemented. We look forward to your contributions: [Development Route](https://gitcode.net/mzdk100/rigela/-/issues/1)
If you don't know how to program, or the program does not support your local language, you can also contribute to the
internationalization of I18N and translate some text into your local language. We welcome your PR.

## agreement

The open source license of this project is based on the Apache License 2.0, which means that you (whether an individual
or a company) using, modifying or distributing the code of this project must comply with the following:
Do not misappropriate the trademark of RigelA project!
When you distribute this work or derivative works, you can no longer need to provide the source code to us! But you must
be agreed:

1. Copy a RigelA license to your software directory;
2. Retain all copyrights, patents and other instructions for this software;
3. Changed documents must be marked and explained;
4. The information in the NOTICE file needs to be retained;
5. You may continue to license subject to the conditions of this License;
6. You bear full responsibility for the use of this software project, and we do not provide any guarantee.