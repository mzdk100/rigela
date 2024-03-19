/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use std::fmt::{Debug, Formatter};

pub use windows::Win32::UI::Shell::{
    Common::ITEMIDLIST, FOLDERID_AccountPictures, FOLDERID_AddNewPrograms, FOLDERID_AdminTools,
    FOLDERID_AllAppMods, FOLDERID_AppCaptures, FOLDERID_AppDataDesktop, FOLDERID_AppDataDocuments,
    FOLDERID_AppDataFavorites, FOLDERID_AppDataProgramData, FOLDERID_AppUpdates,
    FOLDERID_ApplicationShortcuts, FOLDERID_AppsFolder, FOLDERID_CDBurning, FOLDERID_CameraRoll,
    FOLDERID_CameraRollLibrary, FOLDERID_ChangeRemovePrograms, FOLDERID_CommonAdminTools,
    FOLDERID_CommonOEMLinks, FOLDERID_CommonPrograms, FOLDERID_CommonStartMenu,
    FOLDERID_CommonStartMenuPlaces, FOLDERID_CommonStartup, FOLDERID_CommonTemplates,
    FOLDERID_ComputerFolder, FOLDERID_ConflictFolder, FOLDERID_ConnectionsFolder,
    FOLDERID_Contacts, FOLDERID_ControlPanelFolder, FOLDERID_Cookies, FOLDERID_CurrentAppMods,
    FOLDERID_Desktop, FOLDERID_DevelopmentFiles, FOLDERID_Device, FOLDERID_DeviceMetadataStore,
    FOLDERID_Documents, FOLDERID_DocumentsLibrary, FOLDERID_Downloads, FOLDERID_Favorites,
    FOLDERID_Fonts, FOLDERID_GameTasks, FOLDERID_Games, FOLDERID_History, FOLDERID_HomeGroup,
    FOLDERID_HomeGroupCurrentUser, FOLDERID_ImplicitAppShortcuts, FOLDERID_InternetCache,
    FOLDERID_InternetFolder, FOLDERID_Libraries, FOLDERID_Links, FOLDERID_LocalAppData,
    FOLDERID_LocalAppDataLow, FOLDERID_LocalDocuments, FOLDERID_LocalDownloads,
    FOLDERID_LocalMusic, FOLDERID_LocalPictures, FOLDERID_LocalStorage, FOLDERID_LocalVideos,
    FOLDERID_LocalizedResourcesDir, FOLDERID_Music, FOLDERID_MusicLibrary, FOLDERID_NetHood,
    FOLDERID_NetworkFolder, FOLDERID_Objects3D, FOLDERID_OneDrive, FOLDERID_OriginalImages,
    FOLDERID_PhotoAlbums, FOLDERID_Pictures, FOLDERID_PicturesLibrary, FOLDERID_Playlists,
    FOLDERID_PrintHood, FOLDERID_PrintersFolder, FOLDERID_Profile, FOLDERID_ProgramData,
    FOLDERID_ProgramFiles, FOLDERID_ProgramFilesCommon, FOLDERID_ProgramFilesCommonX64,
    FOLDERID_ProgramFilesCommonX86, FOLDERID_ProgramFilesX64, FOLDERID_ProgramFilesX86,
    FOLDERID_Programs, FOLDERID_Public, FOLDERID_PublicDesktop, FOLDERID_PublicDocuments,
    FOLDERID_PublicDownloads, FOLDERID_PublicGameTasks, FOLDERID_PublicLibraries,
    FOLDERID_PublicMusic, FOLDERID_PublicPictures, FOLDERID_PublicRingtones,
    FOLDERID_PublicUserTiles, FOLDERID_PublicVideos, FOLDERID_QuickLaunch, FOLDERID_Recent,
    FOLDERID_RecordedCalls, FOLDERID_RecordedTVLibrary, FOLDERID_RecycleBinFolder,
    FOLDERID_ResourceDir, FOLDERID_RetailDemo, FOLDERID_Ringtones, FOLDERID_RoamedTileImages,
    FOLDERID_RoamingAppData, FOLDERID_RoamingTiles, FOLDERID_SampleMusic, FOLDERID_SamplePictures,
    FOLDERID_SamplePlaylists, FOLDERID_SampleVideos, FOLDERID_SavedGames, FOLDERID_SavedPictures,
    FOLDERID_SavedPicturesLibrary, FOLDERID_SavedSearches, FOLDERID_Screenshots,
    FOLDERID_SearchHistory, FOLDERID_SearchHome, FOLDERID_SearchTemplates, FOLDERID_SendTo,
    FOLDERID_SidebarDefaultParts, FOLDERID_SidebarParts, FOLDERID_SkyDrive,
    FOLDERID_SkyDriveCameraRoll, FOLDERID_SkyDriveDocuments, FOLDERID_SkyDriveMusic,
    FOLDERID_SkyDrivePictures, FOLDERID_StartMenu, FOLDERID_StartMenuAllPrograms, FOLDERID_Startup,
    FOLDERID_SyncManagerFolder, FOLDERID_SyncResultsFolder, FOLDERID_SyncSetupFolder,
    FOLDERID_System, FOLDERID_SystemX86, FOLDERID_Templates, FOLDERID_UserPinned,
    FOLDERID_UserProfiles, FOLDERID_UserProgramFiles, FOLDERID_UserProgramFilesCommon,
    FOLDERID_UsersFiles, FOLDERID_UsersLibraries, FOLDERID_Videos, FOLDERID_VideosLibrary,
    FOLDERID_Windows, FOLDERID_SEARCH_CSC, FOLDERID_SEARCH_MAPI, KF_FLAG_ALIAS_ONLY,
    KF_FLAG_CREATE, KF_FLAG_DEFAULT, KF_FLAG_DEFAULT_PATH, KF_FLAG_DONT_UNEXPAND,
    KF_FLAG_DONT_VERIFY, KF_FLAG_FORCE_APPCONTAINER_REDIRECTION,
    KF_FLAG_FORCE_APP_DATA_REDIRECTION, KF_FLAG_FORCE_PACKAGE_REDIRECTION, KF_FLAG_INIT,
    KF_FLAG_NOT_PARENT_RELATIVE, KF_FLAG_NO_ALIAS, KF_FLAG_NO_APPCONTAINER_REDIRECTION,
    KF_FLAG_NO_PACKAGE_REDIRECTION, KF_FLAG_RETURN_FILTER_REDIRECTION_TARGET,
    KF_FLAG_SIMPLE_IDLIST, KNOWN_FOLDER_FLAG, SLGP_FLAGS, SLGP_RAWPATH, SLGP_RELATIVEPRIORITY,
    SLGP_SHORTPATH, SLGP_UNCPRIORITY, SLR_ANY_MATCH, SLR_FLAGS, SLR_INVOKE_MSI, SLR_KNOWNFOLDER,
    SLR_MACHINE_IN_LOCAL_TARGET, SLR_NOLINKINFO, SLR_NONE, SLR_NOSEARCH, SLR_NOTRACK, SLR_NOUPDATE,
    SLR_NO_OBJECT_ID, SLR_NO_UI, SLR_NO_UI_WITH_MSG_PUMP, SLR_OFFER_DELETE_WITHOUT_FILE,
    SLR_UPDATE, SLR_UPDATE_MACHINE_AND_SID,
};
use windows::{
    core::{Interface, GUID, HSTRING},
    Win32::{
        Foundation::MAX_PATH,
        Storage::FileSystem::WIN32_FIND_DATAW,
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::Shell::{IShellLinkW, SHGetKnownFolderPath, ShellLink as SL},
    },
};

use crate::{
    com::persist::PersistFile,
    common::{HANDLE, HWND, SHOW_WINDOW_CMD},
    input::VIRTUAL_KEY,
};

/**
 * 查询由文件夹的 KNOWNFOLDERID 标识的已知文件夹的完整路径。
 * 此函数替换 get_folder_path。 该旧函数现在只是 get_known_folder_path 的包装器。
 * `rfid` 对标识文件夹的 KNOWNFOLDERID 的引用。
 * `flags` 指定特殊检索选项的标志。 此值可以为 0;否则，一个或多个 KNOWN_FOLDER_FLAG 值。
 * `h_token` 表示特定用户的访问令牌。如果此参数为NULL（这是最常见的用法），则该函数会请求当前用户的已知文件夹。通过传递该用户的h_token请求特定用户的文件夹。这通常在具有足够权限检索给定用户的令牌的服务上下文中完成。必须使用TOKEN_QUERY和TOKEN_IMPERSONATE权限打开该令牌。在某些情况下，还需要包含TOKEN_DUPLICATE。除了传递用户的h_token外还必须装载该特定用户的注册表配置单元。有关访问控制问题的进一步讨论，请参阅访问控制。为h_token 参数分配值 -1 表示默认用户。 这允许 get_known_folder_path 的客户端查找文件夹位置 (，例如默认用户的 桌面 文件夹) 。 创建任何新用户帐户时，默认用户用户配置文件将重复，并包含文档和桌面等特殊文件夹。 添加到“默认用户”文件夹的任何项目也会显示在任何新用户帐户中。 请注意，访问“默认用户”文件夹需要管理员权限。
 * */
pub fn get_known_folder_path(
    rfid: &GUID,
    flags: KNOWN_FOLDER_FLAG,
    h_token: Option<HANDLE>,
) -> Option<String> {
    match unsafe { SHGetKnownFolderPath(rfid, flags, h_token.unwrap_or(HANDLE::default())) } {
        Ok(p) => match unsafe { p.to_string() } {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

/// 公开用于创建、修改和解析 Shell 链接的方法。
pub struct ShellLink(IShellLinkW);

/// https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinkw
impl ShellLink {
    /**
     * 创建一个快捷连接对象。
     * */
    pub fn new() -> Self {
        let link = unsafe { CoCreateInstance::<_, IShellLinkW>(&SL, None, CLSCTX_INPROC_SERVER) }
            .expect("Can't create the shell link.");
        Self(link)
    }

    /**
     * 设置 Shell 链接对象的说明。说明可以是任何应用程序定义的字符串。
     * `description` 包含新说明的字符串。
     * */
    pub fn set_description(&self, description: String) -> &Self {
        unsafe {
            self.0
                .SetDescription(&HSTRING::from(description))
                .unwrap_or(())
        }
        self
    }

    /// 获取 Shell 链接对象的说明字符串。
    pub fn get_description(&self) -> String {
        let mut buf: [u16; 1024] = [0; 1024];
        unsafe { self.0.GetDescription(&mut buf).unwrap_or(()) };
        String::from_utf16_lossy(&buf)
            .trim_matches('\0')
            .to_string()
    }

    /**
     * 设置 Shell 链接对象的目标的路径和文件名。
     * `path` 文件的新路径。
     * */
    pub fn set_path(&self, path: String) -> &Self {
        unsafe {
            self.0.SetPath(&HSTRING::from(path)).unwrap_or(());
        }
        self
    }

    //noinspection SpellCheckingInspection
    /**
     * 获取 Shell 链接对象的目标的路径和文件名。
     *
     * `flags` 指定要查询的路径信息类型的标志。此参数可以是以下值的组合。
     * - SLGP_SHORTPATH 查询标准短（8.3 格式）文件名。
     * - SLGP_UNCPRIORITY 支持;不要使用。
     * - SLGP_RAWPATH 查询原始路径名。原始路径可能不存在，可能包含需要扩展的环境变量。
     * - SLGP_RELATIVEPRIORITY Windows Vista 及更高版本。如果可能，检索快捷方式目标的路径（如果可能），该路径相对于上一个对 IShellLink：：SetRelativePath 的调用所设置的路径。
     * */
    pub fn get_path(&self, flags: SLGP_FLAGS) -> (WIN32_FIND_DATAW, String) {
        unsafe {
            let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
            let mut fd = std::mem::zeroed();
            self.0
                .GetPath(&mut buf, &mut fd, flags.0 as u32)
                .unwrap_or(());
            (
                fd,
                String::from_utf16_lossy(&buf)
                    .trim_matches('\0')
                    .to_string(),
            )
        }
    }

    //noinspection StructuralWrap
    //noinspection StructuralWrap
    /**
     * 设置命令行管理程序链接对象的命令行参数。
     * 在创建指向将特殊标志作为参数的应用程序（如编译器）的链接时，此方法非常有用。
     * `args` 包含新命令行的参数。
     * */
    pub fn set_arguments(&self, args: String) -> &Self {
        unsafe {
            self.0.SetArguments(&HSTRING::from(args)).unwrap_or(());
        }
        self
    }

    /**
     * 获取与 Shell 链接对象关联的命令行参数。
     * */
    pub fn get_arguments(&self) -> String {
        unsafe {
            let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
            self.0.GetArguments(&mut buf).unwrap_or(());
            String::from_utf16_lossy(&buf)
                .trim_matches('\0')
                .to_string()
        }
    }

    /**
     * 设置 Shell 链接对象的键盘快捷方式（热键）。
     * 设置键盘快捷键允许用户通过按特定的键组合来激活对象。
     * `key` 虚拟键。
     * `flags` 修饰键标志，0表示没有修饰键，可以是已下值的组合：
     * - HOTKEYF_ALT Alt 键
     * - HOTKEYF_CONTROL CTRL 键
     * - HOTKEYF_EXT 扩展密钥
     * - HOTKEYF_SHIFT 换档键
     * */
    pub fn set_hotkey(&self, flags: u32, key: VIRTUAL_KEY) -> &ShellLink {
        unsafe { self.0.SetHotkey((flags as u16) << 8 | key.0) }.unwrap_or(());
        self
    }

    /**
     * 获取 Shell 链接对象的键盘快捷方式（热键）。
     * 返回键盘快捷方式(修饰符标志,虚拟键代码)。修饰符标志可以是以下值的组合。
     * - HOTKEYF_ALT Alt 键
     * - HOTKEYF_CONTROL CTRL 键
     * - HOTKEYF_EXT 扩展密钥
     * - HOTKEYF_SHIFT 换档键
     * */
    pub fn get_hotkey(&self) -> (u32, VIRTUAL_KEY) {
        let hotkey = unsafe { self.0.GetHotkey() }.unwrap_or(0);
        ((hotkey >> 8) as u32, VIRTUAL_KEY(hotkey))
    }

    //noinspection StructuralWrap
    /**
     * 设置 Shell 链接对象的图标的位置（路径和索引）。
     * `path` 包含图标的文件的路径。
     * `index` 图标索引。
     * */
    pub fn set_icon_location(&self, path: String, index: i32) -> &ShellLink {
        unsafe { self.0.SetIconLocation(&HSTRING::from(path), index) }.unwrap_or(());
        self
    }

    /**
     * 获取 Shell 链接对象的图标的位置（路径和索引）。
     * */
    pub fn get_icon_location(&self) -> (String, i32) {
        unsafe {
            let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
            let mut icon = std::mem::zeroed();
            self.0.GetIconLocation(&mut buf, &mut icon).unwrap_or(());
            (
                String::from_utf16_lossy(&buf)
                    .trim_matches('\0')
                    .to_string(),
                icon,
            )
        }
    }

    //noinspection StructuralWrap
    /**
     * 设置指向 Shell 链接对象的项标识符列表 （PIDL） 的指针。
     * 当应用程序需要将命令行管理程序链接设置为非文件对象（如控制面板应用程序、打印机或其他计算机）时，此方法非常有用。
     * `list` 对象的完全限定 IDL。
     * */
    pub fn set_id_list(&self, list: Option<ITEMIDLIST>) -> &ShellLink {
        unsafe {
            if let Some(list) = list {
                self.0.SetIDList(&list)
            } else {
                self.0.SetIDList(std::ptr::null())
            }
        }
            .unwrap_or(());
        self
    }

    /**
     * 获取 Shell 链接对象的目标的项标识符列表。
     * */
    pub fn get_id_list(&self) -> Option<ITEMIDLIST> {
        unsafe {
            if let Ok(p) = self.0.GetIDList() {
                return Some(*p);
            }
        }
        None
    }

    //noinspection StructuralWrap
    /**
     * 设置 Shell 链接对象的相对路径。
     * 客户端通常定义一个相对链接，当它可能与其目标一起移动时，导致绝对路径变得无效。set_relative_path 方法可用于帮助链接解析过程根据目标和相对路径之间的公共路径前缀查找其目标。为了协助解析过程，客户端应设置相对路径作为链接创建过程的一部分。
     * `path` 其中包含快捷方式文件的完全限定路径，应相对于该路径执行快捷方式解析。它应该是文件名，而不是文件夹名称。
     * */
    pub fn set_relative_path(&self, path: String) -> &Self {
        // https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setrelativepath
        unsafe {
            self.0
                .SetRelativePath(&HSTRING::from(path), 0)
                .unwrap_or(());
        }
        self
    }

    //noinspection StructuralWrap
    /**
     * 设置 Shell 链接对象的 show 命令。show 命令设置窗口的初始显示状态。
     * `show_cmd` 接受以下 ShowWindow 命令之一。
     * - SW_SHOWNORMAL 激活并显示一个窗口。如果窗口最小化或最大化，系统会将其恢复到其原始大小和位置。应用程序应在首次显示窗口时指定此标志。
     * - SW_SHOWMAXIMIZED 激活窗口并将其显示为最大化窗口。
     * - SW_SHOWMINNOACTIVE 以最小化状态显示窗口，使当前活动窗口保持活动状态。
     * */
    pub fn set_show_cmd(&self, show_cmd: SHOW_WINDOW_CMD) -> &ShellLink {
        unsafe { self.0.SetShowCmd(show_cmd) }.unwrap_or(());
        self
    }

    /**
     * 获取 Shell 链接对象的 show 命令。
     * show 命令用于设置相应对象的初始 show 状态。这是  show_window  中描述的SW_xxx值之一。
     * */
    pub fn get_show_cmd(&self) -> SHOW_WINDOW_CMD {
        unsafe { self.0.GetShowCmd().unwrap_or(SHOW_WINDOW_CMD::default()) }
    }

    //noinspection StructuralWrap
    /**
     * 设置 Shell 链接对象的工作目录的名称。
     * 除非目标需要工作目录，否则工作目录是可选的。例如，如果应用程序创建指向 Microsoft Word 文档的 Shell 链接，该文档使用驻留在不同目录中的模板，则应用程序将使用此方法设置工作目录。
     * `dir` 包含新工作目录的名称。
     * */
    pub fn set_working_directory(&self, dir: String) -> &Self {
        unsafe {
            self.0
                .SetWorkingDirectory(&HSTRING::from(dir))
                .unwrap_or(());
        }
        self
    }

    /**
     * 获取 Shell 链接对象的工作目录的名称。
     * */
    pub fn get_working_directory(&self) -> String {
        unsafe {
            let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
            self.0.GetWorkingDirectory(&mut buf).unwrap_or(());
            String::from_utf16_lossy(&buf)
                .trim_matches('\0')
                .to_string()
        }
    }

    //noinspection StructuralWrap
    /**
     * 尝试查找命令行管理程序链接的目标，即使该链接已被移动或重命名。
     * 创建链接后，目标的名称或位置可能会更改。resolve 方法首先检索与链接关联的路径。如果对象不再存在或已重命名，resolve 将尝试查找它。如果成功，并且满足以下条件，则从中加载链接对象的文件将更新以反映链接对象的新状态。
     * • 设置了SLR_UPDATE标志。
     * • 目标已移动或重命名，更新了 Shell 链接对象的内部状态以引用新目标。
     * • 命令行管理程序链接对象是通过 PersistFile 从文件加载的。
     * 客户端还可以调用 PersistFile::is_dirty 方法来确定链接对象是否已更改以及文件是否需要更新。
     * resolve有两种查找目标对象的方法。首先是分布式链接跟踪服务。如果该服务可用，它可以查找位于 NTFS 版本 5.0 卷上并已移动到该卷上其他位置的对象。它还可以查找已移动到另一个 NTFS 版本 5.0 卷（包括其他计算机上的卷）的对象。若要禁止使用此服务，请设置SLR_NOTRACK标志。
     * 如果分布式链接跟踪不可用或找不到链接对象，则resolve会尝试使用搜索启发式方法查找该对象。它首先在对象的最后一个已知目录中查找具有不同名称但属性和文件创建时间相同的对象。接下来，它以递归方式搜索对象最后一个已知目录附近的子目录。它查找具有相同名称或创建时间的对象。最后，resolve在桌面和其他本地卷上查找匹配对象。若要禁止使用搜索启发式方法，请设置SLR_NOSEARCH标志。
     * 如果这两种方法都失败，系统将显示一个对话框，提示用户输入位置。若要禁止显示该对话框，请设置SLR_NO_UI标志。
     * `h_wnd` 命令行管理程序将用作对话框父级的窗口的句柄。如果命令行管理程序在解析命令行管理程序链接时需要提示用户输入详细信息，则命令行管理程序将显示该对话框。
     * `flags` 操作标志。此参数可以是以下值的组合。
     * - SLR_NO_UI （0x0001） 如果无法解析链接，则不显示对话框。设置SLR_NO_UI时，可以将 fFlags 的高阶字设置为超时值，该值指定解析链接所花费的最大时间。如果无法在超时持续时间内解析链接，则返回该函数。如果高阶字设置为零，则超时持续时间将设置为默认值 3,000 毫秒（3 秒）。若要指定值，请将 fFlags 的高位字设置为所需的超时持续时间（以毫秒为单位）。
     * - SLR_ANY_MATCH （0x0002） 未使用。
     * - SLR_UPDATE （0x0004） 如果链接对象已更改，请更新其路径和标识符列表。如果设置了SLR_UPDATE，则无需调用 IPersistFile：：IsDirty 来确定链接对象是否已更改。
     * - SLR_NOUPDATE （0x0008） 请勿更新链接信息。
     * - SLR_NOSEARCH （0x0010） 不要执行搜索启发式方法。
     * - SLR_NOTRACK （0x0020） 不要使用分布式链接跟踪。
     * - SLR_NOLINKINFO （0x0040） 禁用分布式链接跟踪。默认情况下，分布式链接跟踪会根据卷名称跨多个设备跟踪可移动媒体。它还使用 UNC 路径来跟踪驱动器号已更改的远程文件系统。设置SLR_NOLINKINFO将禁用这两种类型的跟踪。
     * - SLR_INVOKE_MSI （0x0080） 调用 Windows Installer。
     * - SLR_NO_UI_WITH_MSG_PUMP （0x0101） Windows XP 及更高版本。
     * - SLR_OFFER_DELETE_WITHOUT_FILE （0x0200） Windows 7 及更高版本。当此方法无法解析快捷方式时，提供删除快捷方式的选项，即使快捷方式不是文件的快捷方式也是如此。
     * - SLR_KNOWNFOLDER （0x0400） Windows 7 及更高版本。如果目标是已知文件夹，并且已知文件夹已重定向，则报告为脏文件夹。仅当原始目标路径是文件系统路径或 ID 列表，而不是别名已知文件夹 ID 列表时，这才有效。
     * - SLR_MACHINE_IN_LOCAL_TARGET （0x0800） Windows 7 及更高版本。解析 UNC 目标中指向本地计算机的计算机名称。此值与 SLDF_KEEP_LOCAL_IDLIST_FOR_UNC_TARGET 一起使用。
     * - SLR_UPDATE_MACHINE_AND_SID （0x1000） Windows 7 及更高版本。如有必要，请更新计算机 GUID 和用户 SID。
     * */
    pub fn resolve(&self, h_wnd: HWND, flags: SLR_FLAGS) -> &Self {
        unsafe { self.0.Resolve(h_wnd, flags.0 as u32) }.unwrap_or(());
        self
    }

    /**
     * 打开对象文件。
     * */
    pub fn open_file(&self) -> Option<PersistFile> {
        if let Ok(f) = self.0.cast() {
            return Some(PersistFile::from_raw(f));
        }
        None
    }
}

impl Debug for ShellLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShellLink()")
    }
}

#[cfg(test)]
mod test_shell {
    use crate::{
        com::co_initialize_multi_thread,
        common::{HWND, SW_RESTORE},
        input::VK_A,
        shell::{
            get_known_folder_path, FOLDERID_Desktop, FOLDERID_Profile, ShellLink, KF_FLAG_DEFAULT,
            SLGP_SHORTPATH, SLR_NOSEARCH,
        },
    };

    #[test]
    fn main() {
        co_initialize_multi_thread().ok().unwrap();
        let link = ShellLink::new();
        dbg!(link.set_description("rigela".to_string()).get_description());
        dbg!(link
            .set_path("D:\\rigela.exe".to_string())
            .set_relative_path("D:\\rigela.lnk".to_string())
            .get_path(SLGP_SHORTPATH));
        dbg!(link.set_arguments("shell".to_string()).get_arguments());
        dbg!(link.set_hotkey(0, VK_A).get_hotkey());
        dbg!(link
            .set_icon_location("logo.ico".to_string(), 0)
            .get_icon_location());
        // dbg!(link.set_id_list(None).get_id_list());
        dbg!(link.set_show_cmd(SW_RESTORE).get_show_cmd());
        dbg!(link
            .set_working_directory("D:\\".to_string())
            .get_working_directory());
        link.resolve(HWND::default(), SLR_NOSEARCH);
        if let Some(file) = link.open_file() {
            file.save(Some("D:\\rigela.lnk".to_string()), true);
        }
        dbg!(get_known_folder_path(
            &FOLDERID_Desktop,
            KF_FLAG_DEFAULT,
            None,
        ));
        dbg!(get_known_folder_path(
            &FOLDERID_Profile,
            KF_FLAG_DEFAULT,
            None,
        ));

        dbg!(link);
    }
}
