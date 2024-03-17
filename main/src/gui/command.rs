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

use crate::configs::config_operations::save_is_display_shortcut;
use crate::{
    configs::{
        config_manager::ConfigRoot,
        config_operations::{
            apply_mouse_config, save_auto_check_update, save_lang, save_run_on_startup,
        },
        general::Lang,
        tts::TtsConfig,
    },
    context::Context,
    gui::utils::{
        backup_data, check_update, confirm_update_exists, restore_data, set_startup_registry,
        UpdateState, HELP_DIR,
    },
    talent::Talented,
};
use log::error;
use nwg::{message, MessageParams};
use rigela_utils::fs::get_program_directory;
use std::{env::args, path::PathBuf, process::Command, sync::Arc};
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
    // {
    //     #[cfg(debug_assertions)]
    //     return;
    // }

    context.work_runtime.spawn(async move {
        match (auto, check_update().await) {
            (true, UpdateState::None) => {
                // 如果是自动检查更新，且检查失败，则不做任何操作
                return;
            }
            (false, UpdateState::None) => {
                // 手动检查, 未检测到更新需要弹窗提示
                message_box(
                    HWND::default(),
                    &t!("cmd.msg_newest_version"),
                    &t!("cmd.msg_mind_title"),
                    MB_OK,
                );
                return;
            }
            (_, UpdateState::Updated) => {
                message_box(
                    HWND::default(),
                    &t!("cmd.msg_newest_version"),
                    &t!("cmd.msg_mind_title"),
                    MB_OK,
                );
                return;
            }
            _ => {}
        };

        // 启动更新器
        let child = match confirm_update_exists().await {
            Ok(_) => Command::new("cmd.exe")
                // 需要使用cmd.exe辅助启动，使用start参数不等待，否则当更新器尝试kill主进程时，更新器自己也会被kill
                .arg("/c")
                .arg("start")
                .arg(get_program_directory().join("libs/update.exe"))
                .arg(args().nth(0).unwrap())
                .spawn(),
            Err(_) => {
                message_box(
                    HWND::default(),
                    &t!("cmd.msg_no_updater"),
                    &t!("cmd.msg_mind_title"),
                    MB_OK,
                );
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
    message_box(HWND::default(), &t!("cmd.msg_thanks"), "RigelA", MB_OK);
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
    let path = args().nth(0).unwrap();
    if set_startup_registry("RigelA", &PathBuf::from(path), toggle).is_err() {
        error!("registry operation failed! ");
    }
    save_run_on_startup(context.clone(), toggle);

    let msg = if toggle {
        t!("cmd.msg_auto_start_on").to_string()
    } else {
        t!("cmd.msg_auto_start_off").to_string()
    };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(&msg).await;
    });
}

/// 设置自动检测更新
pub(crate) fn set_auto_check_update_cmd(context: Arc<Context>, toggle: bool) {
    save_auto_check_update(context.clone(), toggle);

    let msg = if toggle {
        t!("cmd.msg_auto_check_on").to_string()
    } else {
        t!("cmd.msg_auto_check_off").to_string()
    };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(&msg).await;
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
        t!("cmd.msg_switch_to_zh").to_string()
    } else {
        t!("cmd.msg_switch_to_en").to_string()
    };
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        pf.speak(&msg).await;
    });
}

/// 设置语音角色
pub(crate) fn set_voice_cmd(context: Arc<Context>, engine: String, name: String) {
    let ctx = context.clone();
    let tts = context.performer.get_tts().clone();
    context.main_handler.spawn(async move {
        let info = tts
            .get_all_voiceinfo()
            .await
            .iter()
            .find(|v| v.name == name && v.engine == engine)
            .unwrap()
            .clone();

        let mut root = ctx.config_manager.get_config();
        let cfg = TtsConfig {
            voice: (info.engine, info.id),
            ..root.tts_config
        };
        root.tts_config = cfg.clone();
        ctx.config_manager.set_config(&root);
        tts.apply_config(&cfg.clone()).await;
        ctx.performer
            .speak(&t!("cmd.tts_role", value = info.name))
            .await;
    });
}

/// 设置语音速度
pub(crate) fn set_speed_cmd(context: Arc<Context>, index: usize) {
    let speed = 100 - index as i32;

    let mut root = context.config_manager.get_config();
    let cfg = TtsConfig {
        speed,
        ..root.tts_config
    };
    root.tts_config = cfg.clone();
    context.config_manager.set_config(&root);

    let tts = context.performer.get_tts();
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        tts.apply_config(&cfg.clone()).await;
        pf.speak(&t!("cmd.tts_speed", value = speed)).await;
    });
}

/// 设置语音音调
pub(crate) fn set_pitch_cmd(context: Arc<Context>, index: usize) {
    let pitch = 100 - index as i32;

    let mut root = context.config_manager.get_config();
    let cfg = TtsConfig {
        pitch,
        ..root.tts_config
    };
    root.tts_config = cfg.clone();
    context.config_manager.set_config(&root);

    let tts = context.performer.get_tts();
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        tts.apply_config(&cfg.clone()).await;
        pf.speak(&t!("cmd.tts_pitch", value = pitch)).await;
    });
}

/// 设置语音音量
pub(crate) fn set_volume_cmd(context: Arc<Context>, index: usize) {
    let volume = 100 - index as i32;

    let mut root = context.config_manager.get_config();
    let cfg = TtsConfig {
        volume,
        ..root.tts_config
    };
    root.tts_config = cfg.clone();
    context.config_manager.set_config(&root);

    let tts = context.performer.get_tts();
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        tts.apply_config(&cfg.clone()).await;
        pf.speak(&t!("cmd.tts_volume", value = volume)).await;
    });
}

/// 设置鼠标朗读
pub(crate) fn set_mouse_read_cmd(context: Arc<Context>, toggle: bool) {
    apply_mouse_config(context.clone(), toggle);

    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        let state = match toggle {
            true => t!("cmd.msg_mouse_read_on"),
            false => t!("cmd.msg_mouse_read_off"),
        };
        pf.speak(&state).await;
    });
}

/// 导出配置
pub(crate) fn export_config_cmd(_context: Arc<Context>, path: PathBuf) {
    if backup_data(&path).is_err() {
        error!("备份数据失败");
    }

    message_box(
        HWND::default(),
        &t!("cmd.msg_export_success"),
        &t!("cmd.msg_mind_title"),
        MB_OK,
    );
}

/// 导入配置
pub(crate) fn import_config_cmd(context: Arc<Context>, path: PathBuf) {
    if restore_data(&path).is_err() {
        error!("恢复数据失败");
    }

    context.config_manager.init();
    reapply_config(context.clone());

    message_box(
        HWND::default(),
        &t!("cmd.msg_import_success"),
        &t!("cmd.msg_mind_title"),
        MB_OK,
    );
}

/// 还原默认配置
pub(crate) fn reset_config_cmd(context: Arc<Context>) {
    let msg_params = MessageParams {
        title: &t!("cmd.msg_confirm_title"),
        content: &t!("cmd.msg_reset_confirm"),
        buttons: nwg::MessageButtons::OkCancel,
        icons: nwg::MessageIcons::Question,
    };
    if message(&msg_params) == nwg::MessageChoice::Cancel {
        return;
    }

    context.config_manager.set_config(&ConfigRoot::default());
    reapply_config(context.clone());
}

// 重新应用配置
fn reapply_config(context: Arc<Context>) {
    let config = context.config_manager.get_config();

    let path = args().nth(0).unwrap();
    let enable = config.general_config.run_on_startup.clone();
    if set_startup_registry("RigelA", &PathBuf::from(path), enable).is_err() {
        error!("关闭开机自动启动失败");
    }

    let pf = context.performer.clone();
    let ctx = context.clone();
    let tts = context.performer.get_tts();
    let tts_cfg = config.tts_config.clone();
    context.main_handler.spawn(async move {
        // 应用配置到TTS
        tts.apply_config(&tts_cfg.clone()).await;

        // 重新显示设置界面，更新界面上的状态值
        ctx.gui_provider.show_settings_form();

        pf.speak(&t!("cmd.msg_reset_success")).await;
    });
}

/// 添加桌面快捷方式
pub(crate) fn add_desktop_shortcut_cmd(context: Arc<Context>, toggle: bool) {
    match toggle {
        true => {
            // todo
        }
        false => {
            // todo
        }
    }

    save_is_display_shortcut(context.clone(), toggle);

    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        let state = if toggle { "添加" } else { "删除" };
        let info = format!("已{}桌面快捷方式", state);
        pf.speak(&info).await;
    });
}
