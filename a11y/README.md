# 无障碍（ACCESSIBILITY,A11Y,辅助功能）

## 简介

这个箱子主要提供了一些来自非微软官方提供的辅助功能技术的封装，我们的目标是尽可能做到封装的全面并可以开箱急用。
支持的是IA2([IAccessible2](https://en.wikipedia.org/wiki/IAccessible2))
和JAB([Java Access Bridge](https://en.wikipedia.org/wiki/Java_Access_Bridge))。
其中IA2自2013年以来已经没有新的变化或改进，使用UIA技术作为替代方案是最佳选择。
JAB技术基于Open JDK的WindowsAccessBridge.dll动态库实现，Open JDK有很多的发行版本，具体接口可能有所差异，如果当前JAB的实现无法满足需求，请提交相关Issue给我们。
IA2技术在一些古老的应用程序上表现的较好，例如IE浏览器，而JAB是针对由Java编写的应用程序提供辅助功能支持的接口，例如可以支持一些编程工具（Android
Studio,IntelliJ IDEA...)、抓包工具Charles等UI界面的信息获取。
很多应用程序使用的GUI框架和库没有实现AT（[Assistive Technology，辅助技术](https://en.wikipedia.org/wiki/Assistive_technology)
），因此无论是哪一种辅助功能API都是无效的，例如python使用的TCL/TK图形化界面。
在这种情况下，我们将尝试探索更多的可能，例如针对某些GUI框架进行单独适配，或者实现一个更通用的AT接口和标准，持续拓展和改进UI界面可访问的能力，这也是本箱子的主要功能。

## JAB API

Java Access Bridge API 使您能够为使用 Java 应用程序的 Microsoft Windows 操作系统开发辅助技术应用程序。它包含本机方法，使您能够查看和操作有关
Java 应用程序中 GUI 元素的信息，这些信息通过 Java Access Bridge 转发到您的辅助技术应用程序。
[原始接口参考](https://docs.oracle.com/javase/9/access/jaapi.htm#JSACC-GUID-C10D11B0-F588-43FA-BBDE-70E9085E9AFF)

## IA2 API

IAccessible2是用于Microsoft Windows应用程序的可访问性API。IAccessible2最初由IBM以Missouri项目的代号开发，IAccessible
2现在由自由标准组织（现在是Linux基金会的一部分）负责。它被定位为Microsoft新的UI自动化API的替代品。
[原始接口参考](https://accessibility.linuxfoundation.org/a11yspecs/ia2/docs/html/interface_i_accessible2.html)

## Scintilla API

Scintilla并不是一个辅助技术，他是一个用于Win32、GTK和macOS的免费源代码编辑器组件。
Scintilla的开发最初是为了改进PythonWin中的文本编辑器。在对PythonWin使用的Richedit控件中的问题感到沮丧之后，看起来最好的方法是编写一个新的编辑控件。Richedit和其他类似控件的最大问题是，它们将样式更改视为对文档的重要持久更改，因此这些更改将保存到撤消堆栈中并设置文档的脏标志。对于源代码，样式不应该被持久化，因为它可以被机械地重新创建。
Scintilla已经被运用到很多产品中，被大家熟知的就是Notepad++，轻量级：启动速度快，内存占用少，支持许多编程语言的语法高亮显示和代码提示。
本箱子仅实现对Scintilla控件的外部访问支持（文本、图像信息的获取，控件自动化），不提供创建和销毁的功能。
[原始接口文档](https://www.scintilla.org/ScintillaDoc.html)
