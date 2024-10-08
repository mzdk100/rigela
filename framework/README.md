# RigelaA框架层

# 介绍

框架层实现了一个轻量级的组件-系统（Component-System）架构模式，这种架构面向数据（而不是面向对象），遵循组合优于继承、可以用于构建可扩展和可维护的应用程序。
运行时使用轮询的方式不断调用每一个注册到框架中的系统(System)，然后每一个系统通过参数收集来访问框架中的每一个组件(Component)，而不是基于上下文(Context)的模型来和每个组件进行交互，因为是轮询机制，所以他可以处理更多的任务而不会受限于事件订阅的机制。
这种架构模式在游戏APP中非常常见，因为它允许开发者将应用程序分解为独立的组件和系统，从而提高代码的可维护性和可扩展性，但本箱子实现的(C-S)并不和游戏中的(C-S)完全相同，概念有所差异。
我们把这种架构引入到读屏应用中，废除了上下文和事件订阅的机制，可以实现更加复杂且利于定制的功能。

## 解决哪些问题
1. 在事件订阅中，容易出现重复的事件，我们处理起来比较困难；
2. 在事件订阅中，某些事件没有按预期到达，从而产生BUG；
3. 在上下文模型中，无法方便的定义全局变量；
4. 在以前，组件之间的共享会很棘手，因为他往往需要使用一些引用计数的数据结构(Arc,Weak)，并且可能导致循环引用；
5. 性能问题，在以前我们使用tokio来实现多线程和异步编程，但在实践中我们发现这是不必要的，因为读屏其实在IO读写方面不是性能瓶颈；
6. 基于async/await的异步编程需要在普通函数和异步函数之间切换，会导致代码不够整洁；
7. 数据的征用，使用std::sync::Mutex一不小心就会死锁。


# 示例

## 基本示例
演示如何定义自己的组件和系统：
```shell
cargo run --package rigela-framework --example basic --target x86_64-pc-windows-msvc
```

## 内置组件示例
演示如何加载默认插件并处理程序退出。
Com1组件添加到框架时，初始内部值是10，随着countdown系统运行10次后，组件的内部值变为0，然后退出程序。
```shell
cargo run --package rigela-framework --example builtin --target x86_64-pc-windows-msvc
```

# 框架基础层

## 结构体 AppRunner
AppRunner 是应用程序运行器的核心结构体，它包含三个主要部分：
1. components：一个 HashMap，用于存储应用程序中的组件。键是组件的类型ID，值是一个 Mutex 包装的 Box<dyn Any>，用于确保线程安全。
2. systems：一个 Vec，用于存储应用程序中的系统。系统是执行特定任务的函数或对象。
3. plugins，用于存储一些插件，插件可以对一些组件和系统进行整合包装，隐藏实现细节，使用更方便。

### 方法
1. new()：
   - 构造函数，用于创建一个新的 AppRunner 实例，初始化 components 和 systems。
2. add_component<C: Component + 'static>(&mut self, component: C) -> &mut Self：
   - 这个方法用于向应用程序添加一个新的组件。它将组件包装在 Mutex 中，并存储在 components 中。
3. get_component<C: Component + 'static>(&self) -> Comp<C>：
   - 这个方法用于获取指定类型的组件。它首先通过类型ID从 components 中查找组件，如果找到，则返回一个 Comp 对象，该对象封装了组件的 Mutex 锁。
   - 如果找不到组件，它会记录调用位置并引发一个 panic。
4. remove_component<C: Component + 'static>(&mut self) -> &mut Self：
   - 此函数用于删除已经注册到框架中的组件。
5. add_system<P>(&mut self, system: impl ToSystem<P> + 'static) -> &mut Self：
   - 这个方法用于向应用程序添加一个新的系统。系统必须实现 ToSystem trait，并且是 'static 类型。ToSystem trait 应该定义一个 to_system 方法，该方法将系统转换为 System 类型。
6. remove_system<P>(&mut self, system: impl ToSystem<P> + 'static) -> &mut Self：
   - 这个方法移除注册到框架中的系统。
7. run(&mut self)：
   - 这个方法用于启动应用程序的主循环。在循环中，它依次调用每个系统，并执行其任务。在调试模式下，它还会在每次循环后暂停500毫秒，以便开发者可以观察应用程序的行为。

### 注意事项
- 线程安全：由于 components 是一个 HashMap，其中每个值都是一个 Mutex 包装的 Box<dyn Any>，因此 AppRunner 是线程安全的。这意味着可以在多个线程中同时访问和修改组件。
- 类型安全：通过使用 TypeId 和 Any trait，AppRunner 可以在运行时动态地处理不同类型的组件。但是，这也意味着在编译时无法进行类型检查，可能会引入运行时错误。
- 错误处理：如果尝试获取一个不存在的组件，get_component 方法会引发 panic。在实际应用中，可能需要更优雅的错误处理机制。

## 组件(Component)
组件是数据的集合，严格意义上讲只有变量（没有函数），但可以有getter和setter函数，Component之间不可以直接通信。
Component trait用于定义一个组件，他可以实现struct、enum等。
Comp用于包装组件类型，被Comp包装的数据充当函数的参数，这样函数就具有了System特性，简单来说就是Comp告诉系统需要收集（查询）哪些组件共系统内部使用。

## 系统(System)
系统用来制定APP的运行规则，只有函数，没有变量。System之间的执行顺序需要严格制定，System之间不可以直接相互调用。
一个 System只关心某一个固定的Component组合，这个组合集合称为tuple。
一个Component会被不同System区别对待，因为每个System用到的数据可能只有其中一部分，且不一定相同。


# 框架内置功能

## 内置组件
1. Terminator，程序终结器，用于随时退出主线程循环；

## 内置插件
1. DefaultPlugin，包含常用的组件和系统；

## 内置系统
1. terminate，用于处理程序退出的系统，此系统包含在DefaultPlugin中，如果没有使用此插件则需要自行导入；

