# 贡献指南
欢迎使用RigelA！

## 从哪里开始
我们非常欢迎所有喜欢为RigelA项目做出贡献的朋友。
您不仅可以贡献代码，还可以贡献bug报告、评论、问题、答案，或者只是随手简单的一次分享。
如果你喜欢贡献代码，可以清除一个bug，但如果你不知道从哪里开始，这里有一些提示。
- [发展路线](https://gitcode.net/mzdk100/rigela/-/issues/1)
你可以从我们的规划路线中挑选一些东西。我们在这个简单的问题线程中保持项目的进度。它有新的模型提案或开发更新等。
- [问题跟踪](https://gitcode.net/mzdk100/rigela/-/issues/)
这是一个查找功能请求和bug的地方。
- 也可以随意提出新的功能或想法。我们总是对新事物持开放态度。

## 提交 ✨**PR**✨
如果您有新功能、要消除的bug，请继续提交✨**PR**✨。
请使用以下步骤提交✨**PR**✨。如果您在此过程中遇到问题，请告诉我们。
1. Fork RigelA[https://gitcode.net/mzdk100/rigela/]，点击fork按钮，把这个项目分叉到自己的账号中。
2. 克隆您fork的RigelA，并将主仓库添加到您的上游远程。
    ```sh
    git clone https://gitcode.net/<您的账号名称>/rigela.git
    cd rigela
    git remote add upstream https://gitcode.net/mzdk100/rigela.git
    ```
3. 设置开发环境
    您需要安装必要的msvc编译器和rust环境，然后创建一个新分支。
    ```sh
    git checkout -b 您的新分支
    ```
4. 在您的新分支中实现功能和更改代码、消除BUG。
5. 编写“tests”测试套件。重要的是要表明您的代码是有效的，考虑了边缘情况，并告知其他人预期用途。
6. 运行测试，看看您的更新是否与项目的其余部分可以正常协同工作。您可以在实施更改时多次重复此步骤，以确保您的方向正确。
7. 使用rustfmt格式化您的代码，让您的代码看起来符合rust的代码样式。
    ```sh
    rustfmt
    ```
8. 当情况良好时，添加新文件并提交您的更改。
    ```sh
    git add my_file1.rs my_file2.rs ...
    git commit
    ```
    定期将项目的本地副本与上游代码同步以跟上最新地更新是一种很好的做法。
    ```sh
    git fetch upstream
    git rebase upstream/master
    # 或用于开发版本
    git rebase upstream/dev
    ```
9.向``dev``分支提交PR。把你的分支推到 fork 上。
    ```sh
    git push -u origin 您的新分支
    ```
    然后转到你的fork的Github页面，点击“合并请求”发送你的✨**PR**✨。
    请设置✨**PR**✨的目标分支到`dev``，因为我们使用`dev```来处理下一个版本。
10. 让我们讨论吧直到完美。💪
    我们可能会要求您进行某些更改，这些更改将显示在✨**PR**✨的页面。
11. 一旦事情看起来完美，我们将其合并到“dev”分支，并为下一个版本做好准备。

## 需要帮助
如果您遇到困难需要帮助，欢迎和我取得联系。
