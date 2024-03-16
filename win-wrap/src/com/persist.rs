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
use windows::{
    core::HSTRING,
    Win32::System::Com::IPersistFile,
};
use crate::common::{FALSE, TRUE};
pub use windows::Win32::System::Com::{STGM, STGM_CONVERT, STGM_CREATE, STGM_DELETEONRELEASE, STGM_DIRECT, STGM_DIRECT_SWMR, STGM_FAILIFTHERE, STGM_NOSCRATCH, STGM_NOSNAPSHOT, STGM_PRIORITY, STGM_READ, STGM_READWRITE, STGM_SHARE_DENY_NONE, STGM_SHARE_DENY_READ, STGM_SHARE_DENY_WRITE, STGM_SHARE_EXCLUSIVE, STGM_TRANSACTED, STGM_WRITE};

/// 允许从磁盘文件加载对象或将其保存到磁盘文件，而不是存储对象或流。由于打开文件所需的信息因应用程序而异，因此对象上的 load 实现还必须打开其磁盘文件。
pub struct PersistFile(IPersistFile);

// https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersistfile
impl PersistFile {
    pub(crate) fn from_raw(raw: IPersistFile) -> Self {
        Self(raw)
    }

    /**
     * 查询与对象关联的文件的当前名称。如果没有当前工作文件，此方法将检索对象的默认保存提示。
     * 返回的文件名是加载文档时在调用 load 时指定的名称;如果文档已保存到其他文件，则在 save_completed 中。
     * 如果对象没有当前工作文件，则该对象应提供默认提示，该提示将显示在“另存为”对话框中。例如，字处理器对象的默认保存提示可以是“*.txt”。
     * OLE 不调用 get_cur_file 方法。应用程序不会调用此方法，除非它们也调用此接口的 save 方法。
     * 在保存对象时，可以在调用save之前调用此方法，以确定对象是否具有关联的文件。如果此方法成功返回，则可以使用NULL文件名和remember参数的true值调用save，以告知对象将自身保存到其当前文件。如果此方法失败，则可以使用返回的保存提示来要求最终用户提供文件名。然后，可以使用用户输入的文件名调用save以执行另存为操作。
     * */
    pub fn get_cur_file(&self) -> Option<String> {
        unsafe {
            if let Ok(f) = self.0.GetCurFile() {
                return match f.to_string() {
                    Ok(s) => Some(s.clone()),
                    Err(_) => None
                };
            }
        }
        return None;
    }

    /**
     * 确定对象自上次保存到其当前文件以来是否已更改。
     * 使用此方法可确定是否应在关闭对象之前保存该对象。在  save  方法中有条件地清除对象的脏标志。
     * */
    pub fn is_dirty(&self) -> bool {
        unsafe { self.0.IsDirty() }.is_ok()
    }

    /**
     * 打开指定的文件并根据文件内容初始化对象。
     * load 从指定文件加载对象。此方法仅用于初始化，不会向最终用户显示对象。它不等同于用户选择“文件打开”命令时发生的情况。
     * 文件名字对象中的  bind_to_object  方法调用此方法以在名字对象绑定操作期间加载对象 （当链接对象运行时） 。通常，应用程序不会直接调用此方法。
     * `filename` 要打开的文件的绝对路径。
     * `mode` 打开文件时要使用的访问模式。可能的值取自 STGM 枚举。该方法可以将此值视为建议，并在必要时添加更多限制性权限。如果 mode 为 0，则实现应使用用户打开文件时使用的任何默认权限打开文件。
     * */
    pub fn load(&self, filename: String, mode: STGM) {
        unsafe { self.0.Load(&HSTRING::from(filename), mode) }.unwrap_or(())
    }

    /**
     * 将对象的副本保存到指定的文件。
     * 可以通过以下三种方式之一调用此方法将对象保存到指定文件：
     * 实现者必须检测调用方请求的保存操作类型。如果 filename 参数为 NULL，则请求 save。如果 filename 参数不为 NULL，请使用 remember 参数的值来区分“另存为”和“另存副本为”。
     * 在“保存”或“另存为”操作中，save 在保存后清除内部脏标志，并将 AdviseSink::on_save 通知发送到任何咨询连接。此外，在这些操作中，对象处于 NoScribble 模式，直到它收到 save_completed 调用。在 NoScribble 模式下，对象不得写入文件。
     * 在另存为方案中，实现还应将 AdviseSink::on_rename 通知发送到任何咨询连接。
     * 在“另存为”方案中，实现不会在保存后清除内部脏标志。
     * OLE 不调用 save。通常，应用程序不会调用它，除非它们将对象直接保存到文件中，这通常留给最终用户。
     * `filename` 应将对象保存到的文件的绝对路径。如果 filename 为 NULL，则对象应将其数据保存到当前文件（如果有）。
     * `remember` 指示是否将filename参数用作当前工作文件。如果为true，则filename将成为当前文件，并且对象应在保存后清除其脏标志。如果为false，则此保存操作是“将副本另存为...”操作。在这种情况下，当前文件保持不变，对象不应清除其脏标志。如果filename为NULL，则实现应忽略remember标志。
     * */
    pub fn save(&self, filename: Option<String>, remember: bool) {
        let remember = if remember {
            TRUE
        } else {
            FALSE
        };
        unsafe {
            match filename {
                None => self.0.Save(None, remember),
                Some(f) => self.0.Save(&HSTRING::from(f), remember)
            }
        }.unwrap_or(());
    }

    /**
     * 通知对象它可以写入其文件。它通过通知对象它可以从 NoScribble 模式（在该模式下不得写入其文件）恢复到正常模式（在该模式下可以）来实现此目的。组件在收到 save 调用时进入 NoScribble 模式。
     * 当完成对save的调用时，将调用save_completed，并且保存的文件现在是当前工作文件（已使用“保存”或“另存为”操作保存）。对save的调用将对象置于NoScribble模式，因此它无法写入其文件。调用save_completed时，对象将恢复为正常模式，在该模式下，它可以自由地写入其文件。
     * OLE 不调用 save_completed 方法。通常，除非应用程序将对象直接保存到文件中，否则应用程序不会调用它，该操作通常留给最终用户。
     * `filename` 之前保存对象的文件的绝对路径。
     * */
    pub fn save_completed(&self, filename: String) {
        unsafe { self.0.SaveCompleted(&HSTRING::from(filename)) }.unwrap_or(())
    }
}

impl Debug for PersistFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PersistFile()")
    }
}