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

use windows::Win32::UI::Shell::SLGP_FLAGS;
pub use windows::Win32::UI::Shell::{
    SLGP_RAWPATH, SLGP_RELATIVEPRIORITY, SLGP_SHORTPATH, SLGP_UNCPRIORITY,
};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::MAX_PATH,
        Storage::FileSystem::WIN32_FIND_DATAW,
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::Shell::{IShellLinkW, ShellLink as SL},
    },
};

/// 公开用于创建、修改和解析 Shell 链接的方法。
#[derive(Debug)]
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
}

#[cfg(test)]
mod test_shell {
    use crate::{
        com::co_initialize_multi_thread,
        shell::{ShellLink, SLGP_SHORTPATH},
    };

    #[test]
    fn main() {
        co_initialize_multi_thread().unwrap_or(());
        let link = ShellLink::new();
        dbg!(link.set_description("rigela".to_string()).get_description());
        dbg!(link
            .set_path("D:\\rigela.exe".to_string())
            .get_path(SLGP_SHORTPATH));

        dbg!(link);
    }
}
