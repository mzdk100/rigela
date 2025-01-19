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

use crate::{
    commander::keyboard::{combo_keys::ComboKey, keys::Keys},
    context::{Context, ContextAccessor},
};
use arc_swap::ArcSwap;
use log::error;
use native_windows_gui::NoticeSender;
use rigela_utils::fs::{get_rigela_program_directory, read_file, write_file};
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::{args, current_exe},
    error::Error,
    fs::File,
    io::{copy, Read, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak,
    },
    time::Duration,
};
use tokio::time::sleep;
use win_wrap::{
    common::{LRESULT, SW_SHOWNORMAL},
    ext::LParamExt,
    hook::{KbdLlHookStruct, WindowsHook, HOOK_TYPE_KEYBOARD_LL, LLKHF_EXTENDED},
    input::{HOTKEYF_ALT, HOTKEYF_CONTROL, HOTKEYF_EXT, HOTKEYF_SHIFT, WM_KEYDOWN, WM_SYSKEYDOWN},
    registry::{
        reg_close_key, reg_delete_value, reg_open_key_ex, reg_set_value_ex, HKEY_CURRENT_USER,
        KEY_WRITE, REG_SZ,
    },
    shell::ShellLink,
};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

const HELP_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/main/docs/user/help.md?format=json&viewer=simple";
const CHANGELOGS_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/main/docs/user/changelogs.md?format=json&viewer=simple";
//noinspection HttpUrlsUsage
const UPDATER_BIN_URL: &str = "http://api.zhumang.vip:8080/rigela/rigela_x64/updater.exe";

pub(crate) const HELP_DIR: &str = "resources/RigelA 帮助文档.txt";
pub(crate) const CHANGELOGS_DIR: &str = "resources/RigelA 更新日志.txt";
const DOCS_MD5_FILE: &str = "resources/docs_md5.toml";
pub(crate) const UPDATE_BIN_DIR: &str = "libs/update.exe";

/// 文档哈希结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DocsMd5 {
    help_md5: String,
    changelogs_md5: String,
}

pub(crate) enum UpdateState {
    None,
    Updated,

    HasNewest,
}

/// 检测是否需要更新
pub(crate) async fn check_update() -> UpdateState {
    let newest_md5 = get_newest_docs_md5().await;
    let md5_file = get_rigela_program_directory().join(DOCS_MD5_FILE);

    // 如果尚未保存哈希值，则直接首次保存，返回未检测到更新，否则再比较哈希值
    if !md5_file.exists() {
        save_docs_md5(&newest_md5).await;
        save_docs(true).await;
        save_docs(false).await;

        return UpdateState::None;
    }

    let old_md5: DocsMd5 = match read_file(&md5_file).await {
        Ok(ref d) => match toml::from_str(d) {
            Ok(x) => x,
            Err(_) => {
                error!("Can't read docs_md5.toml");
                return UpdateState::None;
            }
        },
        Err(e) => {
            error!("Can't read docs_md5.toml. {}", e);
            return UpdateState::None;
        }
    };

    // 如果更新日志有变动，立即返回可以更新
    if old_md5.changelogs_md5 != newest_md5.changelogs_md5 {
        // 检查当前进程是否被更新器调用，如果更新器成功更新了主程序，会使用--updated命令行参数调用
        if !args().find(|i| i == "--updated").is_none() {
            save_docs(false).await;
            save_docs_md5(&newest_md5).await;
            // 更新流程完毕
            return UpdateState::Updated;
        }
        return UpdateState::HasNewest;
    }

    // 如果帮助文档有变动，保存更新后的帮助文档，
    if old_md5.help_md5 != newest_md5.help_md5 {
        save_docs(true).await;
        save_docs_md5(&newest_md5).await;
    }

    UpdateState::None
}

/// 获取最新的docs_md5
pub(crate) async fn get_newest_docs_md5() -> DocsMd5 {
    let help_md5 = match get_gitcode_data(HELP_URL).await {
        Ok(x) => x.last_commit_sha,
        Err(e) => {
            error!("Can't get help data. {}", e);
            String::new()
        }
    };
    let changelogs_md5 = match get_gitcode_data(CHANGELOGS_URL).await {
        Ok(x) => x.last_commit_sha,
        Err(e) => {
            error!("Can't get changelogs data. {}", e);
            String::new()
        }
    };

    DocsMd5 {
        help_md5,
        changelogs_md5,
    }
}

/// 保存新的docs_md5
pub(crate) async fn save_docs_md5(md5: &DocsMd5) {
    let docs_md5_path = get_rigela_program_directory().join(DOCS_MD5_FILE);
    let data = toml::to_string(md5).unwrap_or_else(|_| {
        error!("数据格式错误.");
        String::new()
    });

    write_file(&docs_md5_path, data.as_bytes())
        .await
        .unwrap_or_else(|_| error!("保存md5出错"));
}

/// 保存最新的文档。
pub(crate) async fn save_docs(help_or_changelogs: bool) {
    let (path, url) = match help_or_changelogs {
        true => (get_rigela_program_directory().join(HELP_DIR), HELP_URL),
        false => (
            get_rigela_program_directory().join(CHANGELOGS_DIR),
            CHANGELOGS_URL,
        ),
    };

    write_file(&path, parse_html_node(url, "blob-content").await.as_bytes())
        .await
        .unwrap_or_else(|_| error!("保存文档出错"));
}

// 异步获取gitcode文件数据
async fn get_gitcode_data(url: &str) -> Result<GitcodeFileData, Box<dyn Error>> {
    Ok(reqwest::get(url).await?.json().await?)
}

// 解析网页的指定节点
async fn parse_html_node(url: &str, node: &str) -> String {
    let html = match get_gitcode_data(url).await {
        Ok(data) => data.html,
        Err(_) => "".to_string(),
    };

    Document::from(html.as_str())
        .find(Class(node))
        .map(|node| node.text())
        .collect::<Vec<String>>()
        .join("\n")
}

//noinspection RsUnresolvedPath
/// 确认更新器存在
pub(crate) async fn confirm_update_exists() -> Result<(), Box<dyn Error>> {
    let path = get_rigela_program_directory().join(UPDATE_BIN_DIR);
    if path.exists() {
        Ok(())
    } else {
        let resp = reqwest::get(UPDATER_BIN_URL).await?;
        write_file(&path, &resp.bytes().await?).await?;
        Ok(())
    }
}

//noinspection RsUnresolvedPath
/// 备份数据
pub(crate) fn backup_data(target: &PathBuf) -> Result<(), Box<dyn Error>> {
    let zip_file_path = Path::new(target.to_str().unwrap());
    let zip_file = File::create(&zip_file_path)?;

    let mut zip = ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::DEFLATE);

    let mut files_to_compress: Vec<PathBuf> = vec![];
    let resources_path = get_rigela_program_directory().join("resources");
    for entry in resources_path.read_dir()? {
        let path = entry?.path();
        if path.is_file() {
            files_to_compress.push(path);
        }
    }
    files_to_compress.push(get_rigela_program_directory().join("config.toml"));

    for file_path in &files_to_compress {
        let file = File::open(file_path)?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        zip.start_file(file_name, options)?;

        let mut buffer = Vec::new();
        copy(&mut file.take(u64::MAX), &mut buffer)?;

        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    Ok(())
}

/// 还原备份数据
pub(crate) fn restore_data(source: &PathBuf) -> Result<(), Box<dyn Error>> {
    let zip_file_path = Path::new(source.as_os_str());
    let zip_file = File::open(zip_file_path)?;

    let mut archive = ZipArchive::new(zip_file)?;
    let extraction_dir = get_rigela_program_directory();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();

        let target_path = match file_name.as_str() {
            "config.toml" => extraction_dir.join(file_name),
            _ => extraction_dir.join("resources").join(file_name),
        };

        let mut output_file = File::create(&target_path)?;
        copy(&mut file, &mut output_file)?;
    }

    Ok(())
}

/**
 * 创建读屏程序的快捷方式。
 * `link_path` 快捷方式的路径（扩展名是.lnk）。
 * `hotkey` 快捷方式的热键。
 * */
pub(crate) fn create_shortcut_link(link_path: String, hotkey: &[Keys]) -> bool {
    let mut modifier = 0;
    if hotkey.contains(&Keys::VkCtrl) {
        modifier |= HOTKEYF_CONTROL;
    }
    if hotkey.contains(&Keys::VkAlt) {
        modifier |= HOTKEYF_ALT;
    }
    if hotkey.contains(&Keys::VkShift) {
        modifier |= HOTKEYF_SHIFT;
    }
    if hotkey.contains(&Keys::VkWin) {
        modifier |= HOTKEYF_EXT;
    }
    let Some(key) = hotkey
        .iter()
        .find(|k| ![Keys::VkAlt, Keys::VkShift, Keys::VkCtrl, Keys::VkWin].contains(k))
    else {
        return false;
    };
    let Some(key) = key.get_code() else {
        return false;
    };
    let exe_path = current_exe().unwrap();
    let directory = exe_path.parent().unwrap().to_str();
    let link = ShellLink::new();
    link.set_description(env!("CARGO_PKG_DESCRIPTION").to_string())
        .set_path(exe_path.to_str().unwrap().to_string())
        .set_relative_path(link_path.clone())
        .set_arguments("--shortcut-link".to_string())
        .set_hotkey(modifier, key)
        .set_show_cmd(SW_SHOWNORMAL)
        .set_working_directory(directory.unwrap().to_string());
    if let Some(file) = link.open_file() {
        file.save(Some(link_path), true);
        return true;
    }
    false
}

//noinspection DuplicatedCode
// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation
#[derive(Serialize, Deserialize)]
struct GitcodeFileData {
    #[serde(rename = "tree_path")]
    tree_path: String,

    #[serde(rename = "extension")]
    extension: String,

    #[serde(rename = "last_commit_sha")]
    pub last_commit_sha: String,

    #[serde(rename = "blame_path")]
    blame_path: String,

    #[serde(rename = "simple_viewer")]
    simple_viewer: String,

    #[serde(rename = "show_viewer_switcher")]
    show_viewer_switcher: bool,

    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "mime_type")]
    mime_type: String,

    #[serde(rename = "binary")]
    binary: bool,

    #[serde(rename = "commits_path")]
    commits_path: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "html")]
    pub html: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "permalink")]
    permalink: String,

    #[serde(rename = "raw_path")]
    raw_path: String,
}

/**
 * 添加开机自启
 * `enable` true添加,false删除。
 * */
pub fn set_startup_registry(enable: bool) -> win_wrap::common::Result<()> {
    let program_name = env!("CARGO_PKG_NAME");
    let path = current_exe().unwrap();
    let path = path
        .to_str()
        .unwrap()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let path: Vec<u8> = path
        .iter()
        .flat_map(|&x| x.to_le_bytes().to_vec())
        .collect();

    let software_key = r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"#;
    let hkey = reg_open_key_ex(HKEY_CURRENT_USER, Some(software_key), None, KEY_WRITE);

    if enable {
        reg_set_value_ex(hkey, Some(program_name), None, REG_SZ, Some(&path))?;
    } else {
        reg_delete_value(hkey, Some(program_name))?;
    }
    reg_close_key(hkey);

    Ok(())
}

/**
 * 设置键盘钩子
 * `keys` 产生好的键位列表
 * `senders` 通知发送者，`senders[0]`为完成的通知，`senders[1]`为取消
 * */
pub(crate) fn set_hook(
    context: Weak<Context>,
    keys: Arc<ArcSwap<Option<ComboKey>>>,
    senders: &[NoticeSender; 2],
) -> WindowsHook {
    let hotkeys = keys.clone();
    let finish_sender = senders[0];
    let cancel_sender = senders[1];
    let is_started = AtomicBool::new(false);

    fn start(context: Weak<Context>, sender: NoticeSender) {
        context.get_work_runtime().spawn(async move {
            sleep(Duration::from_secs(1)).await;
            sender.notice();
        });
    }

    let key_track = Arc::new(Mutex::new(HashMap::<Keys, bool>::new()));
    WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, _next| {
        let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;
        let info: &KbdLlHookStruct = l_param.to();
        let is_extended = info.flags.contains(LLKHF_EXTENDED);
        let key: Keys = (info.vkCode, is_extended).into();
        let cur_key = key.trans_rigela();

        {
            key_track.lock().unwrap().insert(cur_key.clone(), pressed);
        }

        let keys: ComboKey = key_track
            .lock()
            .unwrap()
            .iter()
            .filter(|(k, p)| **k == cur_key || **p)
            .map(|(x, _)| *x)
            .collect::<Vec<Keys>>()
            .into();

        let mng = context.get_commander().get_keyboard_manager().clone();
        let cancel_keys = [Keys::VkEscape, Keys::VkReturn];
        match pressed {
            true if !cur_key.is_modifierkey() => {
                if !is_started.load(Ordering::Relaxed) {
                    start(context.clone(), finish_sender.clone());
                    is_started.store(true, Ordering::Relaxed);
                }

                hotkeys.store(Arc::new(mng.process_combo_key(&keys, pressed)));

                if cancel_keys.contains(&cur_key) {
                    cancel_sender.notice()
                }
            }
            false => {
                // todo
            }
            _ => {}
        }

        LRESULT(1)
    })
}

/**
 * 设置键盘钩子简化版
 * `keys` 产生好的键位列表。
 * `senders` 通知发送者，`senders[0]`为完成的通知，`senders[1]`为取消
 * */
pub(crate) fn set_hook_simple(
    keys: Arc<ArcSwap<ComboKey>>,
    senders: &[NoticeSender; 2],
) -> WindowsHook {
    let hotkeys = keys.clone();
    let finish_sender = senders[0];
    let cancel_sender = senders[1];

    let key_track = Arc::new(Mutex::new(HashMap::<Keys, bool>::new()));
    WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, _next| {
        let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;
        let info: &KbdLlHookStruct = l_param.to();
        let is_extended = info.flags.contains(LLKHF_EXTENDED);
        let key: Keys = (info.vkCode, is_extended).into();
        let cur_key = key.trans_rigela();

        {
            key_track.lock().unwrap().insert(cur_key.clone(), pressed);
        }

        // 当前已经按下的键位
        let keys: ComboKey = key_track
            .lock()
            .unwrap()
            .iter()
            .filter(|(k, p)| **k == cur_key || **p)
            .map(|(x, _)| *x)
            .collect::<Vec<Keys>>()
            .into();

        // 有一个非辅助见键位松开，完成读取
        let cancel_keys = [Keys::VkEscape, Keys::VkReturn];
        if !pressed && !cur_key.is_modifierkey() {
            match cancel_keys.contains(&cur_key) {
                true => cancel_sender.notice(),

                false => {
                    hotkeys.store(Arc::new(keys));
                    finish_sender.notice();
                }
            }
        }

        LRESULT(1)
    })
}
