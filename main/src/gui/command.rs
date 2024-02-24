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

use crate::configs::config_operations::{
    apply_mouse_config, save_auto_check_update, save_lang, save_run_on_startup,
};
use crate::configs::general::Lang;
use crate::gui::utils::UpdateState;
use crate::{
    context::Context,
    gui::utils::{check_update, confirm_update_exists, HELP_DIR},
    talent::Talented,
};
use log::error;
use rigela_utils::get_program_directory;
use std::{env::args, process::Command, sync::Arc};
use win_wrap::common::{message_box, HWND, MB_OK};

/// 退出程序。
pub(crate) fn exit_cmd(context: Arc<Context>) {
    let ctx = context.clone();
    context.main_handler.spawn(async move {
        let talent = ctx.talent_provider.get_exit_talent();
        talent.perform(ctx.clone()).await;
    });
}

/// 打开帮助文档。
pub(crate) fn help_cmd(_context: Arc<Context>) {
    let help_path = get_program_directory().join(HELP_DIR);
    Command::new("notepad")
        .arg(help_path)
        .spawn()
        .expect("Failed to start notepad");
}

/// 打开设置窗口。
pub(crate) fn settings_cmd(context: Arc<Context>) {
    context.gui_provider.show_settings_form();
}

/// 检查更新。
#[allow(unused)]
pub(crate) fn check_update_cmd(context: Arc<Context>, auto: bool) {
    {
        #[cfg(debug_assertions)]
        return;
    }

    context.work_runtime.spawn(async move {
        match (auto, check_update().await) {
            (true, UpdateState::None) => {
                // 如果是自动检查更新，且检查失败，则不做任何操作
                return;
            }
            (false, UpdateState::None) => {
                // 手动检查, 未检测到更新需要弹窗提示
                message_box(HWND::default(), "当前版本已是最新版本！", "提示", MB_OK);
                return;
            }
            (_, UpdateState::Updated) => {
                message_box(HWND::default(), "您已经更新到最新版！", "提示", MB_OK);
                return;
            }
            _ => {}
        };

        // 启动更新器
        let child = match confirm_update_exists().await {
            Ok(_) => Command::new(get_program_directory().join("libs/update.exe"))
                .arg(args().nth(0).unwrap())
                .spawn(),
            Err(_) => {
                message_box(HWND::default(), "更新器不存在！", "提示", MB_OK);
                return;
            }
        };
        if let Err(e) = child {
            error!("Failed to start update.exe. {}", e);
        }
    });
}

/// 打开自定义热键窗口。
pub(crate) fn custom_hotkeys_cmd(context: Arc<Context>) {
    context.gui_provider.show_hotkeys_form();
}

/// 打开欢迎界面。
pub(crate) fn welcome_form_cmd(context: Arc<Context>) {
    context.gui_provider.show_welcome_form();
}

/// 打开捐赠界面。
pub(crate) fn donate_cmd(_context: Arc<Context>) {
    message_box(HWND::default(), "感谢支持！", "RigelA", MB_OK);
}

/// 打开关于窗口
pub(crate) fn about_form_cmd(context: Arc<Context>) {
    context.gui_provider.show_about_form();
}

/// 访问开源官网
pub(crate) fn visit_host_website_cmd(_context: Arc<Context>) {
    const URL: &str = "https://github.com/mzdk100/rigela";

    Command::new("cmd")
        .args(&["/c", "start", URL])
        .spawn()
        .expect("Failed to start cmd");
}

/// 设置开机自动启动
pub(crate) fn set_auto_start_cmd(context: Arc<Context>, toggle: bool) {
    // 在注册表添加开机自动启动

    save_run_on_startup(context.clone(), toggle);

    let msg = if toggle {
        "已开启开机自动启动！"
    } else {
        "已关闭开机自动启动！"
    };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(msg.to_string()).await;
    });
}

/// 设置自动检测更新
pub(crate) fn set_auto_check_update_cmd(context: Arc<Context>, toggle: bool) {
    // Todo

    save_auto_check_update(context.clone(), toggle);

    let msg = if toggle {
        "已开启自动更新！"
    } else {
        "已关闭自动更新！"
    };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(msg.to_string()).await;
    });
}

/// 设置语言
pub(crate) fn set_lang_cmd(context: Arc<Context>, index: usize) {
    // Todo

    let lang = match index {
        1 => Lang::En,
        _ => Lang::Zh,
    };
    save_lang(context.clone(), &lang);

    let msg = if lang == Lang::Zh {
        "已切换为中文！"
    } else {
        "switch to English！"
    };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(msg.to_string()).await;
    });
}

/// 设置语音角色
pub(crate) fn set_voice_cmd(_context: Arc<Context>, index: usize) {
    // Todo

    let msg = format!("设置成功！当前语音角色：{}", index);
    message_box(HWND::default(), msg.as_str(), "提示", MB_OK);
}

/// 设置语音速度
pub(crate) fn set_speed_cmd(_context: Arc<Context>, index: usize) {
    // Todo

    let msg = format!("设置成功！当前语音速度为：{}", index);
    message_box(HWND::default(), msg.as_str(), "提示", MB_OK);
}

/// 设置语音音调
pub(crate) fn set_pitch_cmd(_context: Arc<Context>, index: usize) {
    // Todo

    let msg = format!("设置成功！当前语音音调为：{}", index);
    message_box(HWND::default(), msg.as_str(), "提示", MB_OK);
}

/// 设置语音音量
pub(crate) fn set_volume_cmd(_context: Arc<Context>, index: usize) {
    // Todo

    let msg = format!("设置成功！当前语音音量为：{}", index);
    message_box(HWND::default(), msg.as_str(), "提示", MB_OK);
}

/// 设置鼠标朗读
pub(crate) fn set_mouse_read_cmd(context: Arc<Context>, toggle: bool) {
    apply_mouse_config(context.clone(), toggle);
    let state = if toggle { "开启" } else { "关闭" };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(format!("{}鼠标朗读", state).to_string()).await;
    });
}

/// 导出配置
pub(crate) fn export_config_cmd(_context: Arc<Context>) {
    // Todo

    message_box(HWND::default(), "导出配置成功！", "提示", MB_OK);
}

/// 导入配置
pub(crate) fn import_config_cmd(_context: Arc<Context>) {
    // Todo

    message_box(HWND::default(), "导入配置成功！", "提示", MB_OK);
}

/// 还原默认配置
pub(crate) fn reset_config_cmd(_context: Arc<Context>) {
    // Todo

    message_box(HWND::default(), "恢复默认配置成功！", "提示", MB_OK);
}
