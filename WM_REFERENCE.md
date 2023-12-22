# Windows消息常亮值参考

## 说明
Windows是一个消息（Message）驱动式系统，Windows消息提供了应用程序与应用程序之间、应用程序与Windows系统之间进行通讯的手段。
应用程序要实现的功能由消息来触发，并靠对消息的响应和处理来完成。
Windows系统中有两种消息队列，一种是系统消息队列，另一种是应用程序消息队列。
计算机的所有输入设备由 Windows监控，当一个事件发生时，Windows先将输入的消息放入系统消息队列中，然后再将输入的消息拷贝到相应的应用程序队列中，应用程序中的消息循环从它的消息队列中检索每一个消息并发送给相应的窗口函数中。
一个事件的发生，到达处理它的窗口函数必须经历上述过程。
值得注意的是消息的非抢先性，即不论事件的急与缓，总是按到达的先后排队(一些系统消息除外)，这就使得一些外部实时事件可能得不到及时的处理。
由于Windows本身是由消息驱动的，所以解密时跟踪一个消息会得到相当底层的答案。
举一个例子来说明这个问题，打开记事本程序，该程序有一个File菜单，那么，在运行该应用程序的时候，如果用户单击了File菜单里New命令时，这个动作将被Windows （而不是应用程序本身）所捕获。
Windows经过分析得知这个动作应该由上面所说的那个应用程序去处理，既然是这样，Windows就发送了个叫做WM_COMMAND的消息给应用程序，该消息所包含信息告诉应用程序："用户单击了New菜单"。
应用程序得知这一消息之后，采取相应的动作来响应它，这个过程称为消息处理。
Windows为每一个应用程序(确切地说是每一个线程)维护了相应的消息队列，应用程序的任务就是不停的从它的消息队列中获取消息，分析消息和处理消息，直到一条接到叫做WM_QUIT消息为止，这个过程通常是由一种叫做消息循环的程序结构来实现的。
消息本身是作为一个记录（也叫做结构体）传递给应用程序的，这个记录中包含了消息的类型以及其他信息。
例如，对于单击鼠标所产生的消息来说，这个记录中包含了单击鼠标时的坐标。
这个记录类型叫做MSG：
struct MSG {
hwnd: HWND, // 窗口句柄
message: u32, // 消息常量标识符
wParam: WPARAM, // 32位消息的特定附加信息
lParam: LPARAM, // 32位消息的特定附加信息
time: DWORD, // 消息创建时的时间
pt: POINT // 消息创建时的鼠标位置
}
消息中有什么？是否觉得一个消息记录中的信息像希腊语一样？
如果是这样，那么看一看下面的解释：
hwnd 32位的窗口句柄。窗口可以是任何类型的屏幕对象，因为Win32能够维护大多数可视对象的句柄(窗口、对话框、按钮、编辑框等)。
message 用于区别其他消息的常量值，这些常量可以是Windows单元中预定义的常量，也可以是自定义的常量。
wParam 通常是一个与消息有关的常量值，也可能是窗口或控件的句柄。
lParam 通常是一个指向内存中数据的指针。由于WParam、lParam和Pointer都是32位的，因此，它们之间可以相互转换。


## 窗口消息常亮表
WM_NULL = 0  空消息
WM_CREATE = 1  应用程序创建一个窗口
WM_DESTROY = 2  一个窗口被销毁
WM_MOVE = 3  移动一个窗口
WM_SIZE = 5  改变一个窗口的大小
WM_ACTIVATE = 6  一个窗口被激活或失去激活状态
WM_SETFOCUS = 7  获得焦点后
WM_KILLFOCUS = 8  失去焦点
WM_ENABLE = 10  改变enable状态
WM_SETREDRAW = 11  设置窗口是否能重画
WM_SETTEXT = 12  应用程序发送此消息来设置一个窗口的文本
WM_GETTEXT = 13  应用程序发送此消息来复制对应窗口的文本到缓冲区
WM_GETTEXTLENGTH = 14  得到与一个窗口有关的文本的长度（不包含空字符）
WM_PAINT = 15  要求一个窗口重画自己
WM_CLOSE = 16  当一个窗口或应用程序要关闭时发送一个信号
WM_QUERYENDSESSION = 17  当用户选择结束对话框或程序自己调用ExitWindows函数
WM_QUIT = 18  用来结束程序运行或当程序调用PostQuitMessage函数
WM_QUERYOPEN = 19  当用户窗口恢复以前的大小位置时，把此消息发送给某个图标
WM_ERASEBKGND = 20  当窗口背景必须被擦除时（例在窗口改变大小时）
WM_SYSCOLORCHANGE = 21  当系统颜色改变时，发送此消息给所有顶级窗口
WM_ENDSESSION = 22  当系统进程发出WM_QUERYENDSESSION消息后，此消息发送给应用程序，通知它对话是否结束
WM_SYSTEMERROR = 23  通知应用程序发生了系统错误
WM_SHOWWINDOW = 24  当隐藏或显示窗口是发送此消息给这个窗口
WM_ACTIVATEAPP = 28  发此消息给应用程序哪个窗口是激活的，哪个是非激活的；
WM_FONTCHANGE = 29  当系统的字体资源库变化时发送此消息给所有顶级窗口
WM_TIMECHANGE = 30  当系统的时间变化时发送此消息给所有顶级窗口
WM_CANCELMODE = 31  发送此消息来取消某种正在进行的模态（操作）
WM_SETCURSOR = 32  如果鼠标引起光标在某个窗口中移动且鼠标输入没有被捕获时，就发消息给某个窗口
WM_MOUSEACTIVATE = 33  当光标在某个非激活的窗口中而用户正按着鼠标的某个键发送此消息给当前窗口
WM_CHILDACTIVATE = 34  发送此消息给MDI子窗口当用户点击此窗口的标题栏，或当窗口被激活，移动，改变大小
WM_QUEUESYNC = 35  此消息由基于计算机的训练程序发送，通过WH_JOURNALPALYBACK的hook程序分离出用户输入消息
WM_GETMINMAXINFO = 36  此消息发送给窗口当它将要改变大小或位置；
WM_PAINTICON = 38  发送给最小化窗口当它图标将要被重画
WM_ICONERASEBKGND = 39  此消息发送给某个最小化窗口，仅当它在画图标前它的背景必须被重画
WM_NEXTDLGCTL = 40  发送此消息给一个对话框程序去更改焦点位置
WM_SPOOLERSTATUS = 42  每当打印管理列队增加或减少一条作业时发出此消息
WM_DRAWITEM = 43  当button，combobox，listbox，menu的可视外观改变时发送此消息给这些空件的所有者
WM_MEASUREITEM = 44  当button, combo box, list box, list view control, or menu item 被创建时发送此消息给控件的所有者
WM_DELETEITEM = 45  当列表框或组合框被销毁或某些项被删除，通过LB_DELETESTRING, LB_RESETCONTENT, CB_DELETESTRING, 或者CB_RESETCONTENT消息
WM_VKEYTOITEM = 46  此消息有一个LBS_WANTKEYBOARDINPUT风格的发出给它的所有者来响应WM_KEYDOWN消息
WM_CHARTOITEM = 47  此消息由一个LBS_WANTKEYBOARDINPUT风格的列表框发送给他的所有者来响应WM_CHAR消息
WM_SETFONT = 48  当绘制文本时程序发送此消息得到控件要用的颜色
WM_GETFONT = 49  应用程序发送此消息得到当前控件绘制文本的字体
WM_SETHOTKEY = 50  应用程序发送此消息让一个窗口与一个热键相关连
WM_GETHOTKEY = 51  应用程序发送此消息来判断热键与某个窗口是否有关联
WM_QUERYDRAGICON = 55  此消息发送给最小化窗口，当此窗口将要被拖放而它的类中没有定义图标，应用程序能返回一个图标或光标的句柄，当用户拖放图标时系统显示这个图标或光标
WM_COMPAREITEM = 57  发送此消息来判定combobox或listbox新增加的项的相对位置
WM_GETOBJECT = 61  获取一个绑定的com接口对象
WM_COMPACTING = 65  显示内存已经很少了
WM_WINDOWPOSCHANGING = 70  发送此消息给那个窗口的大小和位置将要被改变时，来调用SetWindowPos函数或其它窗口管理函数
WM_WINDOWPOSCHANGED = 71  发送此消息给那个窗口的大小和位置已经被改变时，来调用SetWindowPos函数或其它窗口管理函数
WM_POWER = 72  （适用于16位的windows）当系统将要进入暂停状态时发送此消息
WM_COPYDATA = 74  当一个应用程序传递数据给另一个应用程序时发送此消息
WM_CANCELJOURNAL = 75  当某个用户取消程序日志激活状态，提交此消息给程序
WM_NOTIFY = 78  当某个控件的某个事件已经发生或这个控件需要得到一些信息时，发送此消息给它的父窗口
WM_INPUTLANGCHANGEREQUEST = 80  当用户选择某种输入语言，或输入语言的热键改变
WM_INPUTLANGCHANGE = 81  当平台现场已经被改变后发送此消息给受影响的最顶级窗口
WM_TCARD = 82  当程序已经初始化windows帮助例程时发送此消息给应用程序
WM_HELP = 83  此消息显示用户按下了F1，如果某个菜单是激活的，就发送此消息个此窗口关联的菜单，否则就发送给有焦点的窗口，如果当前都没有焦点，就把此消息发送给当前激活的窗口
WM_USERCHANGED = 84  当用户已经登入或退出后发送此消息给所有的窗口，当用户登入或退出时系统更新用户的具体设置信息，在用户更新设置时系统马上发送此消息；
WM_NOTIFYFORMAT = 85  公用控件，自定义控件和他们的父窗口通过此消息来判断控件是使用ANSI还是UNICODE结构 在WM_NOTIFY消息，使用此控件能使某个控件与它的父控件之间进行相互通信
WM_CONTEXTMENU = 123  当用户某个窗口中点击了一下右键就发送此消息给这个窗口
WM_STYLECHANGING = 124  当调用SETWINDOWLONG函数将要改变一个或多个 窗口的风格时发送此消息给那个窗口
WM_STYLECHANGED = 125  当调用SETWINDOWLONG函数一个或多个 窗口的风格后发送此消息给那个窗口
WM_DISPLAYCHANGE = 126  当显示器的分辨率改变后发送此消息给所有的窗口
WM_GETICON = 127  此消息发送给某个窗口来返回与某个窗口有关连的大图标或小图标的句柄；
WM_SETICON = 128  程序发送此消息让一个新的大图标或小图标与某个窗口关联；
WM_NCCREATE = 129  当某个窗口第一次被创建时，此消息在WM_CREATE消息发送前发送；
WM_NCDESTROY = 130  此消息通知某个窗口，非客户区正在销毁
WM_NCCALCSIZE = 131  当某个窗口的客户区域必须被核算时发送此消息
WM_NCHITTEST = 132  移动鼠标，按住或释放鼠标时发生
WM_NCPAINT = 133  程序发送此消息给某个窗口当它（窗口）的框架必须被绘制时；
WM_NCACTIVATE = 134  此消息发送给某个窗口 仅当它的非客户区需要被改变来显示是激活还是非激活状态；
WM_GETDLGCODE = 135  发送此消息给某个与对话框程序关联的控件，windows控制方位键和TAB键使输入进入此控件，通过响应WM_GETDLGCODE消息，应用程序可以把他当成一个特殊的输入控件并能处理它
WM_NCMOUSEMOVE = 160  当光标在一个窗口的非客户区内移动时发送此消息给这个窗口 //非客户区为：窗体的标题栏及窗的边框体
WM_NCLBUTTONDOWN = 161  当光标在一个窗口的非客户区同时按下鼠标左键时提交此消息
WM_NCLBUTTONUP = 162  当用户释放鼠标左键同时光标某个窗口在非客户区十发送此消息；
WM_NCLBUTTONDBLCLK = 163  当用户双击鼠标左键同时光标某个窗口在非客户区十发送此消息
WM_NCRBUTTONDOWN = 164  当用户按下鼠标右键同时光标又在窗口的非客户区时发送此消息
WM_NCRBUTTONUP = 165  当用户释放鼠标右键同时光标又在窗口的非客户区时发送此消息
WM_NCRBUTTONDBLCLK = 166  当用户双击鼠标右键同时光标某个窗口在非客户区十发送此消息 
WM_NCMBUTTONDOWN = 167  当用户按下鼠标中键同时光标又在窗口的非客户区时发送此消息
WM_NCMBUTTONUP = 168  当用户释放鼠标中键同时光标又在窗口的非客户区时发送此消息
WM_NCMBUTTONDBLCLK = 169  当用户双击鼠标中键同时光标又在窗口的非客户区时发送此消息 
WM_KEYFIRST = 256  键盘消息筛选开始
WM_KEYDOWN = 256  按下一个键
WM_KEYUP = 257  释放一个键
WM_CHAR = 258  按下某键，并已发出WM_KEYDOWN， WM_KEYUP消息
WM_DEADCHAR = 259  当用TranslateMessage函数翻译WM_KEYUP消息时发送此消息给拥有焦点的窗口
WM_SYSKEYDOWN = 260  当用户按住ALT键同时按下其它键时提交此消息给拥有焦点的窗口
WM_SYSKEYUP =261  当用户释放一个键同时ALT 键还按着时提交此消息给拥有焦点的窗口
WM_SYSCHAR = 262  当WM_SYSKEYDOWN消息被TranslateMessage函数翻译后提交此消息给拥有焦点的窗口
WM_SYSDEADCHAR = 263  当WM_SYSKEYDOWN消息被TranslateMessage函数翻译后发送此消息给拥有焦点的窗口
WM_KEYLAST = 264  键盘消息筛选结束
WM_IME_STARTCOMPOSITION = 269
WM_IME_ENDCOMPOSITION = 270
WM_IME_COMPOSITION = 271
WM_IME_KEYLAST = 271
WM_INITDIALOG = 272  在一个对话框程序被显示前发送此消息给它，通常用此消息初始化控件和执行其它任务
WM_COMMAND = 273  当用户选择一条菜单命令项或当某个控件发送一条消息给它的父窗口，一个快捷键被翻译
WM_SYSCOMMAND = 274  当用户选择窗口菜单的一条命令或当用户选择最大化或最小化时那个窗口会收到此消息
WM_TIMER = 275  发生了定时器事件
WM_HSCROLL = 276  当一个窗口标准水平滚动条产生一个滚动事件时发送此消息给那个窗口，也发送给拥有它的控件
WM_VSCROLL = 277  当一个窗口标准垂直滚动条产生一个滚动事件时发送此消息给那个窗口也，发送给拥有它的控件
WM_INITMENU = 278  当一个菜单将要被激活时发送此消息，它发生在用户菜单条中的某项或按下某个菜单键，它允许程序在显示前更改菜单
WM_INITMENUPOPUP = 279  当一个下拉菜单或子菜单将要被激活时发送此消息，它允许程序在它显示前更改菜单，而不要改变全部
WM_MENUSELECT = 287  当用户选择一条菜单项时发送此消息给菜单的所有者（一般是窗口）
WM_MENUCHAR = 288  当菜单已被激活用户按下了某个键（不同于加速键），发送此消息给菜单的所有者
WM_ENTERIDLE = 289  当一个模态对话框或菜单进入空载状态时发送此消息给它的所有者，一个模态对话框或菜单进入空载状态就是在处理完一条或几条先前的消息后没有消息它的列队中等待
WM_MENURBUTTONUP = 290 
WM_MENUDRAG = 291 
WM_MENUGETOBJECT = 292 
WM_UNINITMENUPOPUP = 293 
WM_MENUCOMMAND = 294  
WM_CHANGEUISTATE = 295 
WM_UPDATEUISTATE = 296 
WM_QUERYUISTATE = 297 
WM_CTLCOLORMSGBOX = 306  在windows绘制消息框前发送此消息给消息框的所有者窗口，通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置消息框的文本和背景颜色 
WM_CTLCOLOREDIT = 307  当一个编辑型控件将要被绘制时发送此消息给它的父窗口；通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置编辑框的文本和背景颜色 
WM_CTLCOLORLISTBOX = 308  当一个列表框控件将要被绘制前发送此消息给它的父窗口；通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置列表框的文本和背景颜色 
WM_CTLCOLORBTN = 309  当一个按钮控件将要被绘制时发送此消息给它的父窗口；通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置按纽的文本和背景颜色 
WM_CTLCOLORDLG = 310  当一个对话框控件将要被绘制前发送此消息给它的父窗口；通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置对话框的文本背景颜色 
WM_CTLCOLORSCROLLBAR= 311  当一个滚动条控件将要被绘制时发送此消息给它的父窗口；通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置滚动条的背景颜色 
WM_CTLCOLORSTATIC = 312  当一个静态控件将要被绘制时发送此消息给它的父窗口；通过响应这条消息，所有者窗口可以通过使用给定的相关显示设备的句柄来设置静态控件的文本和背景颜色 
WM_MOUSEFIRST = 512  鼠标消息筛选开始
WM_MOUSEMOVE = 512  移动鼠标
WM_LBUTTONDOWN = 513  按下鼠标左键
WM_LBUTTONUP = 514  释放鼠标左键
WM_LBUTTONDBLCLK = 515  双击鼠标左键
WM_RBUTTONDOWN = 516  按下鼠标右键
WM_RBUTTONUP = 517  释放鼠标右键
WM_RBUTTONDBLCLK = 518  双击鼠标右键
WM_MBUTTONDOWN = 519  按下鼠标中键
WM_MBUTTONUP = 520  释放鼠标中键 
WM_MBUTTONDBLCLK = 521  双击鼠标中键
WM_MOUSEWHEEL = 522  当鼠标轮子转动时发送此消息个当前有焦点的控件
WM_MOUSELAST = 522  鼠标消息筛选结束
WM_PARENTNOTIFY = 528  当MDI子窗口被创建或被销毁，或用户按了一下鼠标键而光标在子窗口上时发送此消息给它的父窗口
WM_ENTERMENULOOP = 529  发送此消息通知应用程序的主窗口that已经进入了菜单循环模式
WM_EXITMENULOOP = 530  发送此消息通知应用程序的主窗口that已退出了菜单循环模式
WM_NEXTMENU = 531
WM_SIZING = 532  当用户正在调整窗口大小时发送此消息给窗口；通过此消息应用程序可以监视窗口大小和位置也可以修改他们
WM_CAPTURECHANGED = 533  发送此消息 给窗口当它失去捕获的鼠标时；
WM_MOVING = 534  当用户在移动窗口时发送此消息，通过此消息应用程序可以监视窗口大小和位置也可以修改他们；
WM_POWERBROADCAST = 536  此消息发送给应用程序来通知它有关电源管理事件；
WM_DEVICECHANGE = 537  当设备的硬件配置改变时发送此消息给应用程序或设备驱动程序
WM_MDICREATE = 544  应用程序发送此消息给多文档的客户窗口来创建一个MDI 子窗口
WM_MDIDESTROY = 545  应用程序发送此消息给多文档的客户窗口来关闭一个MDI 子窗口
WM_MDIACTIVATE = 546  应用程序发送此消息给多文档的客户窗口通知客户窗口激活另一个MDI子窗口，当客户窗口收到此消息后，它发出WM_MDIACTIVE消息给MDI子窗口（未激活）激活它；
WM_MDIRESTORE = 547  程序 发送此消息给MDI客户窗口让子窗口从最大最小化恢复到原来大小
WM_MDINEXT =548  程序 发送此消息给MDI客户窗口激活下一个或前一个窗口
WM_MDIMAXIMIZE = 549  程序发送此消息给MDI客户窗口来最大化一个MDI子窗口；
WM_MDITILE = 550  程序 发送此消息给MDI客户窗口以平铺方式重新排列所有MDI子窗口
WM_MDICASCADE = 551  程序 发送此消息给MDI客户窗口以层叠方式重新排列所有MDI子窗口
WM_MDIICONARRANGE = 552  程序 发送此消息给MDI客户窗口重新排列所有最小化的MDI子窗口
WM_MDIGETACTIVE = 553  程序 发送此消息给MDI客户窗口来找到激活的子窗口的句柄
WM_MDISETMENU = 560  程序 发送此消息给MDI客户窗口用MDI菜单代替子窗口的菜单
WM_ENTERSIZEMOVE = 561
WM_EXITSIZEMOVE = 562
WM_DROPFILES = 563
WM_MDIREFRESHMENU = 564
WM_IME_SETCONTEXT = 641
WM_IME_NOTIFY = 642
WM_IME_CONTROL = 643
WM_IME_COMPOSITIONFULL = 644
WM_IME_SELECT = 645
WM_IME_CHAR = 646
WM_IME_REQUEST = 648
WM_IME_KEYDOWN = 656
WM_IME_KEYUP = 657
WM_MOUSEHOVER = 673
WM_MOUSELEAVE = 675
WM_CUT = 768  程序发送此消息给一个编辑框或combobox来删除当前选择的文本
WM_COPY = 769  程序发送此消息给一个编辑框或combobox来复制当前选择的文本到剪贴板
WM_PASTE = 770  程序发送此消息给EditControl或combobox从剪贴板中得到数据
WM_CLEAR = 771  程序发送此消息给EditControl或combobox清除当前选择的内容；
WM_UNDO = 772  程序发送此消息给EditControl或combobox撤消最后一次操作
WM_RENDERFORMAT = 773
WM_RENDERALLFORMATS = 774
WM_DESTROYCLIPBOARD = 775  当调用ENPTYCLIPBOARD函数时 发送此消息给剪贴板的所有者
WM_DRAWCLIPBOARD = 776 当剪贴板的内容变化时发送此消息给剪贴板观察链的第一个窗口；它允许用剪贴板观察窗口来显示剪贴板的新内容；
WM_PAINTCLIPBOARD = 777  当剪贴板包含CF_OWNERDIPLAY格式的数据并且剪贴板观察窗口的客户区需要重画；
WM_VSCROLLCLIPBOARD = 778
WM_SIZECLIPBOARD = 779  当剪贴板包含CF_OWNERDIPLAY格式的数据并且剪贴板观察窗口的客户区域的大小已经改变是此消息通过剪贴板观察窗口发送给剪贴板的所有者
WM_ASKCBFORMATNAME = 780  通过剪贴板观察窗口发送此消息给剪贴板的所有者来请求一个CF_OWNERDISPLAY格式的剪贴板的名字
WM_CHANGECBCHAIN = 781  当一个窗口从剪贴板观察链中移去时发送此消息给剪贴板观察链的第一个窗口；
WM_HSCROLLCLIPBOARD = 782  此消息通过一个剪贴板观察窗口发送给剪贴板的所有者 ；它发生在当剪贴板包含CFOWNERDISPALY格式的数据并且有个事件在剪贴板观察窗的水平滚动条上；所有者应滚动剪贴板图象并更新滚动条的值；
WM_QUERYNEWPALETTE = 783  此消息发送给将要收到焦点的窗口，此消息能使窗口在收到焦点时同时有机会实现他的逻辑调色板
WM_PALETTEISCHANGING= 784  当一个应用程序正要实现它的逻辑调色板时发此消息通知所有的应用程序
WM_PALETTECHANGED = 785  此消息在一个拥有焦点的窗口实现它的逻辑调色板后发送此消息给所有顶级并重叠的窗口，以此来改变系统调色板
WM_HOTKEY = 786  当用户按下由REGISTERHOTKEY函数注册的热键时提交此消息
WM_PRINT = 791 应用程序发送此消息仅当WINDOWS或其它应用程序发出一个请求要求绘制一个应用程序的一部分；
WM_PRINTCLIENT = 792
WM_HANDHELDFIRST = 856
WM_HANDHELDLAST = 863
WM_PENWINFIRST = 896
WM_PENWINLAST = 911
WM_COALESCE_FIRST = 912
WM_COALESCE_LAST = 927
WM_DDE_FIRST = 992
WM_DDE_INITIATE = 992  一个DDE客户程序提交此消息开始一个与服务器程序的会话来响应那个指定的程序和主题名；
WM_DDE_TERMINATE = 993  一个DDE应用程序（无论是客户还是服务器）提交此消息来终止一个会话；
WM_DDE_ADVISE = 994  一个DDE客户程序提交此消息给一个DDE服务程序来请求服务器每当数据项改变时更新它
WM_DDE_UNADVISE = 995  一个DDE客户程序通过此消息通知一个DDE服务程序不更新指定的项或一个特殊的剪贴板格式的项
WM_DDE_ACK = 996  此消息通知一个DDE（动态数据交换）程序已收到并正在处理WM_DDE_POKE, WM_DDE_EXECUTE, WM_DDE_DATA, WM_DDE_ADVISE, WM_DDE_UNADVISE, or WM_DDE_INITIAT消息
WM_DDE_DATA = 997  一个DDE服务程序提交此消息给DDE客户程序来传递个一数据项给客户或通知客户的一条可用数据项
WM_DDE_REQUEST = 998  一个DDE客户程序提交此消息给一个DDE服务程序来请求一个数据项的值；
WM_DDE_POKE = 999  一个DDE客户程序提交此消息给一个DDE服务程序，客户使用此消息来请求服务器接收一个未经同意的数据项；服务器通过答复WM_DDE_ACK消息提示是否它接收这个数据项；
WM_DDE_EXECUTE = 1000  一个DDE客户程序提交此消息给一个DDE服务程序来发送一个字符串给服务器让它象串行命令一样被处理，服务器通过提交WM_DDE_ACK消息来作回应；
WM_DDE_LAST = 1000
WM_APP = 32768
WM_USER = 1024  此消息能帮助应用程序自定义私有消息；


## 通知消息常亮表
通知消息(Notification message)是指这样一种消息，一个窗口内的子控件发生了一些事情，需要通知父窗口。
通知消息只适用于标准的窗口控件如按钮、列表框、组合框、编辑框，以及Windows 95公共控件如树状视图、列表视图等。
例如，单击或双击一个控件、在控件中选择部分文本、操作控件的滚动条都会产生通知消息。

### 按扭
BN_CLICKED  用户单击了按钮
BN_DISABLE  按钮被禁止
BN_DOUBLECLICKED  用户双击了按钮
BN_HILITE  用户加亮了按钮
BN_PAINT  按钮应当重画
BN_UNHILITE  加亮应当去掉

### 组合框
CBN_CLOSEUP  组合框的列表框被关闭
CBN_DBLCLK  用户双击了一个字符串
CBN_DROPDOWN  组合框的列表框被拉出
CBN_EDITCHANGE  用户修改了编辑框中的文本
CBN_EDITUPDATE  编辑框内的文本即将更新
CBN_ERRSPACE  组合框内存不足
CBN_KILLFOCUS  组合框失去输入焦点
CBN_SELCHANGE  在组合框中选择了一项
CBN_SELENDCANCEL  用户的选择应当被取消
CBN_SELENDOK  用户的选择是合法的
CBN_SETFOCUS  组合框获得输入焦点

### 编辑框
EN_CHANGE  编辑框中的文本己更新
EN_ERRSPACE  编辑框内存不足
EN_HSCROLL  用户点击了水平滚动条
EN_KILLFOCUS  编辑框正在失去输入焦点
EN_MAXTEXT  插入的内容被截断
EN_SETFOCUS  编辑框获得输入焦点
EN_UPDATE  编辑框中的文本将要更新
EN_VSCROLL  用户点击了垂直滚动条消息含义

### 列表框
LBN_DBLCLK  用户双击了一项
LBN_ERRSPACE  列表框内存不够
LBN_KILLFOCUS  列表框正在失去输入焦点
LBN_SELCANCEL  选择被取消
LBN_SELCHANGE  选择了另一项
LBN_SETFOCUS  列表框获得输入焦点
