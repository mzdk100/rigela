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

use super::keys::Keys;
use crate::{
    commander::{CommandType, Talent},
    context::Context,
};
use log::debug;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, RwLock},
};
use win_wrap::hook::HOOK_TYPE_MOUSE_LL;
use win_wrap::{
    common::LRESULT,
    ext::LParamExt,
    hook::{KbdLlHookStruct, MsLlHookStruct, WindowsHook, HOOK_TYPE_KEYBOARD_LL, LLKHF_EXTENDED},
    input::{WM_KEYDOWN, WM_MOUSEMOVE, WM_SYSKEYDOWN},
};

/// 设置键盘钩子
pub(crate) fn set_keyboard_hook(context: Arc<Context>, talents: Arc<Vec<Talent>>) -> WindowsHook {
    // 跟踪每一个键的按下状态
    let key_track: RwLock<HashMap<Keys, bool>> = RwLock::new(HashMap::new());
    let talent_keys = context
        .config_manager
        .get_config()
        .hotkeys_config
        .talent_keys;

    WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, next| {
        let info: &KbdLlHookStruct = l_param.to();
        let is_extended = info.flags.contains(LLKHF_EXTENDED);
        let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;

        let mut map = key_track.write().unwrap();
        map.insert((info.vkCode, is_extended).into(), pressed);

        if !pressed {
            drop(map); // 必须先释放锁再next()，否则可能会死锁
            return next();
        }

        for i in talents.iter() {
            if let Some(keys) = talent_keys.get(&i.get_id()) {
                // 如果用户自定义过热键优先使用他定义的。
                if match_keys(keys, &map) {
                    execute(context.clone(), Arc::clone(i));
                    return LRESULT(1);
                }
            }
            if match_cmd_list(Arc::clone(i), &map) {
                // 如果用户没定义过这个能力的热键就使用默认的。
                execute(context.clone(), Arc::clone(i));
                return LRESULT(1);
            }
        }

        drop(map); // 必须先释放锁再next()，否则可能会死锁
        next()
    })
}

// 匹配技能项的热键列表是否与当前Hook到的按键相同
fn match_keys(keys: &Vec<Keys>, map: &HashMap<Keys, bool>) -> bool {
    for key in keys {
        match map.get(key) {
            // 能匹配到按键，并且按键状态为按下，进入下一轮循环
            Some(x) if *x => continue,
            _ => return false,
        }
    }
    // 所有按键都匹配成功
    true
}
fn match_cmd_list(talent: Talent, map: &HashMap<Keys, bool>) -> bool {
    !talent
        .get_supported_cmd_list()
        .iter()
        .find(|x| match *x {
            CommandType::Key(y) => match_keys(y, map),
            _ => false,
        })
        .is_none()
}

// 执行技能项的操作
fn execute(context: Arc<Context>, talent: Talent) {
    let ctx = context.clone();
    context.main_handler.spawn(async move {
        talent.perform(ctx.clone()).await;
    });
}

// 保存鼠标坐标，由于hook闭包函数是Fn类型，无法修改闭包外部值，所以坐标无法保存在set_mouse函数当中
fn get_old_point() -> &'static Mutex<(i32, i32)> {
    static INSTANCE: OnceLock<Mutex<(i32, i32)>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new((0, 0)))
}

/// 设置鼠标钩子
pub(crate) fn set_mouse_hook(context: Arc<Context>) -> WindowsHook {
    let context = context.clone();

    WindowsHook::new(HOOK_TYPE_MOUSE_LL, move |w_param, l_param, next| {
        if w_param.0 != WM_MOUSEMOVE as usize {
            return next();
        }

        let info: &MsLlHookStruct = l_param.to();
        let (x, y) = (info.pt.x, info.pt.y);

        // 如果坐标差值小于10个像素，不处理直接返回
        let (old_x, old_y) = get_old_point().lock().unwrap().clone();
        if (x - old_x) * (x - old_x) + (y - old_y) * (y - old_y) < 100 {
            return next();
        }
        {
            *get_old_point().lock().unwrap() = (x, y);
        }

        let ctx = context.clone();
        context.main_handler.spawn(async move {
            if ctx.config_manager.get_config().mouse_config.is_read {
                debug!("x: {}, y: {}", x, y);
                mouse_read(ctx.clone(), x, y).await;
            }
        });

        next()
    })
}

// 朗读鼠标元素
async fn mouse_read(context: Arc<Context>, x: i32, y: i32) {
    let uia = context.ui_automation.clone();
    let ele = uia.element_from_point(x, y).unwrap();
    context.performer.speak_with_sapi5(ele).await
}
