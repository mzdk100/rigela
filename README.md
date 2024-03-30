# RigelA（名称取自猎户座餐宿七的英文）

简体中文 (zh-CN) | [English (en)](README-EN.md)

<!-- TOC -->

* [RigelA（名称取自猎户座餐宿七的英文）](#rigela名称取自猎户座餐宿七的英文)
    * [简介](#简介)
    * [构建和运行](#构建和运行)
    * [开发文档](#开发文档)
    * [开发贡献](#开发贡献)
    * [许可协议](#许可协议)

<!-- TOC -->

## 简介

本项目是使用rust编写的一个读屏(Screen Reader)项目![logo](logo.ico)。
名称中首字母r和rust的首字母相同，且猎户座在 12 月份是最佳观赏时间，rust注重性能、编码效率和安全，让我们展开想象吧！
中文名称暂定`雷革读屏`，如果大家有更好的想法欢迎告诉我们。，“雷”闪电般的速度，“革”功能革新（也可以使用“格”代替）。
关于名称可以参考[维基百科餐宿七的介绍](https://zh.wikipedia.org/wiki/%E5%8F%83%E5%AE%BF%E4%B8%83)
关于rust环境搭建可以参考相关文档，如果您对rust语言不熟悉，强烈推荐从官方文档或权威的书籍学习。

## 构建和运行

1. 前期准备
   首先您需要下载必要的msvc（如果您电脑中已经有Visual
   Studio，则跳过此步骤），可以访问[此处](https://visualstudio.microsoft.com/zh-hans/downloads/)获取VS安装程序；
   接着我们需要拥有rust的环境，rust的环境搭建可以参考[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn)
   中的介绍；
   假设您已经具备上述条件，在构建程序之前我们还需要下载rust 的x86架构的工具链，运行下面命令进行安装
    ```shell
    rustup target add i686-pc-windows-msvc
    rustup update
    ```
2. 克隆仓库
    ```shell
    git clone https://gitcode.net/mzdk100/rigela.git
    ```
   或者
    ```shell
    git clone https://github.com/mzdk100/rigela.git
    ```
3. 构建调试版本并运行
    ```shell
    cd rigela
    cargo mkenv
    cargo dev
    ```
   其中mkenv用来创建一个专用的构建程序，因为有同时构建32位和64位程序的需要，cargo本身是无法做到这一点的。
   请注意：我们使用"cargo dev"运行而不是"cargo run"，否则您将碰到这样的错误“error: only one `--target` argument is
   supported”。
   如果您执意想用"cargo run"命令，您可以添加"--target x86_64-pc-windows-msvc"额外的参数来实现：
    ```shell
    cargo build --target i686-pc-windows-msvc
    cargo run --target x86_64-pc-windows-msvc
    ```
   其中第一行表示先构建32位的目标，因为第二行需要依赖他，这两行命令不可以反过来写，因此为了简化书写，我们创建了一个dev的短命令。
4. 构建发布版本
   发布版本的程序比调试版本的程序体积更小，运行更快，但构建速度较慢：
    ```shell
    cargo rel
    ```

## 开发文档

我们呼吁对开源读屏项目感兴趣的开发者踊跃参与进来，开发文档也不断完善中，如果您想参与本项目的贡献，请参考下一节。

1. 本项目提供的开发文档：
    - [读屏主程序模块参考文档](main/README.md)
    - [用于非微软官方提供的辅助功能技术的实现参考文档](a11y/README)
    - [用于32位应用的代理模块参考文档](proxy32/README.md)
    - [RigelA资源增量更新参考文档](resources/README.md)
    - [Windows API高级封装的实现参考文档](win-wrap/README.md)
2. 如果您是编程新手，您还需要学习Rust语言，了解Rust的常用库：
    - [Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn)
    - [Rust语言圣经](https://course.rs/about-book.html)
    - [Rust 参考手册（中文翻译非官方）](https://minstrel1977.gitee.io/rust-reference/)
    - [Rust for Windows 仓库](https://github.com/microsoft/windows-rs)
    - [Rust for Windows 文档](https://microsoft.github.io/windows-docs-rs/)
    - [Rust的异步运行时tokio仓库](https://github.com/tokio-rs/tokio)

## 开发贡献

强大的功能离不开快速功能代码的迭代，呼吁有能力的朋友都可以参与到本项目的研发中。
如果您是开发者，您可以fork本仓库到自己的账号中，然后克隆仓库进行研发，随后提交用于合并的请求。
如果您还是不知道如何开始，这里是我们给你准备的[贡献指南](CONTRIBUTING.md)。
我们有一个长期的规划路线，其中是所有用户最感兴趣的想法但还未实现，期待您的添砖加瓦：[发展路线](https://gitcode.net/mzdk100/rigela/-/issues/1)
如果您不会编程，或者程序不支持您当地的语言，你也可以针对国际化I18N做贡献，可以把一些文字翻译成您当地的语言，我们欢迎您的PR。

## 许可协议

本项目的开源许可基于Apache License 2.0，这意味着您（无论是个人还是公司）使用、修改或分发该项目的代码都必须遵守下面内容：
不得盗用RigelA项目的商标！
你分发本作品或衍生作品时，可以不再提供源码！但必须做到

1. 复制一份RigelA许可证到您的软件目录中；
2. 保留本软件的所有版权、专利等说明；
3. 改动的文件必须标出并给出说明；
4. NOTICE文件中的信息需要保留；
5. 在遵循本许可证的条件下，你可以继续许可；
6. 您对使用本软件项目承担全部责任，我们不提供任何担保。
