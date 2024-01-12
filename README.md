# RigelA（名称取自猎户座餐宿七的英文）
本项目是使用rust编写的一个读屏(Screen Reader)项目。
名称中首字母r和rust的首字母相同，且猎户座在12月份是最佳观赏时间，rust注重性能、编码效率和安全，让我们展开想象吧！
关于名称可以参考[维基百科餐宿七的介绍](https://zh.wikipedia.org/wiki/%E5%8F%83%E5%AE%BF%E4%B8%83)
关于rust环境搭建可以参考相关文档，如果您对rust语言不熟悉，强烈推荐从官方文档或权威的书籍学习。


## 构建和运行
```shell
git clone https://gitcode.net/mzdk100/rigela.git
cd rigela
cargo run
```


## 开发文档
我们呼吁对开源读屏项目感兴趣的开发者踊跃参与进来，开发文档也不断完善中，如果您想参与本项目的贡献，请参考下一节。
下面是本项目提供的开发文档：
[用于32位应用的代理模块参考文档](proxy32/README.md)
[RigelA资源增量更新参考文档](resources/README.md)
[Windows消息常亮表](WM_REFERENCE.md)
如果您是编程新手，您还需要学习Rust语言，了解Rust的常用库：
[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn)
[Rust语言圣经](https://course.rs/about-book.html)
[Rust for Windows 仓库](https://github.com/microsoft/windows-rs)
[Rust for Windows 文档](https://microsoft.github.io/windows-docs-rs/)
[Rust的异步运行时tokio仓库](https://github.com/tokio-rs/tokio)


## 开发贡献
强大的功能离不开快速功能代码的迭代，呼吁有能力的朋友都可以参与到本项目的研发中。
您可以fork本仓库到自己的账号中，然后克隆仓库进行研发，随后提交用于合并的请求。
如果您还是不知道如何开始，这里是我们给你准备的[贡献指南](CONTRIBUTING.md)。
我们有一个长期的规划路线，其中是所有用户最感兴趣的想法但还未实现，期待您的添砖加瓦：[发展路线](https://gitcode.net/mzdk100/rigela/-/issues/1)


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
