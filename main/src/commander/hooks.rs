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
use crate::commander::keys::Keys::*;
use crate::configs::config_operations::{get_hotkeys, get_mouse_read_state};
use crate::talent::mouse::mouse_read;
use crate::{
    commander::{CommandType, Talent},
    context::Context,
};
use std::ops::{Deref, DerefMut};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, RwLock},
};
use win_wrap::input::{get_key_state, send_key, VK_CAPITAL};
use win_wrap::{
    common::LRESULT,
    ext::LParamExt,
    hook::HOOK_TYPE_MOUSE_LL,
    hook::{KbdLlHookStruct, MsLlHookStruct, WindowsHook, HOOK_TYPE_KEYBOARD_LL, LLKHF_EXTENDED},
    input::{WM_KEYDOWN, WM_MOUSEMOVE, WM_SYSKEYDOWN},
};

/// 设置键盘钩子
pub(crate) fn set_keyboard_hook(context: Arc<Context>, talents: Arc<Vec<Talent>>) -> WindowsHook {
    let context = context.clone();
    // 跟踪每一个键的按下状态
    let key_track: RwLock<HashMap<Keys, bool>> = RwLock::new(HashMap::new());
    // 暂停键盘钩子
    let ignore_hook = Arc::new(Mutex::new(false));
    // 大小写锁定键状态
    let capital_key_state = Arc::new(Mutex::new(false));
    // 暂停大小写键转换功能
    let ignore_capital_key = Arc::new(Mutex::new(false));

    WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, next| {
        // 根据状态条件暂停钩子处理
        if *ignore_hook.lock().unwrap().deref() {
            return next();
        }

        let info: &KbdLlHookStruct = l_param.to();
        let is_extended = info.flags.contains(LLKHF_EXTENDED);
        let key = (info.vkCode, is_extended).into();
        let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;

        // 调用已在指挥器注册过的回调函数
        let fns = context.commander.get_key_callback_fns();
        for (keys, callback) in fns.iter() {
            if keys.contains(&key) {
                callback(key, pressed);
            }
        }

        // 转换RigelA键
        let key = match key {
            VkNumPad0 | VkCapital | VkInsert => VkRigelA,
            _ => key,
        };

        let mut map = key_track.write().unwrap();
        map.insert(key, pressed);

        match pressed {
            // 松开按键，需要排除大写锁定键，由后面的大写锁定键代码专门处理
            false if info.vkCode as u16 != VK_CAPITAL.0 => {
                drop(map); // 必须先释放锁再next()，否则可能会死锁
                return next();
            }
            true => {
                // 所有键按下都把大写锁定键的状态切换关闭
                *ignore_capital_key.lock().unwrap().deref_mut() = true;
            }
            _ => {}
        }

        for i in talents.iter() {
            match get_hotkeys(context.clone()).get(&i.get_id()) {
                // 如果用户自定义过热键优先使用他定义的。
                Some(keys) if match_keys(keys, &map) => {
                    return execute(context.clone(), Arc::clone(i));
                }
                // 如果用户没定义过这个能力的热键就使用默认的。
                None if match_cmd_list(i.clone(), &map) => {
                    return execute(context.clone(), Arc::clone(i));
                }
                _ => continue,
            };
        }

        let key_count = map.values().filter(|i| **i).count();
        // 大写锁定键处理
        if info.vkCode as u16 == VK_CAPITAL.0 {
            match pressed {
                true => {
                    // 如果按下大写锁定键，保存状态
                    let (_, state) = get_key_state(VK_CAPITAL);
                    *capital_key_state.lock().unwrap().deref_mut() = state;
                    // 如果单独按下大写锁定键，开启锁定键的状态改变
                    if key_count == 1 {
                        *ignore_capital_key.lock().unwrap().deref_mut() = false;
                    }
                }
                false => {
                    // 松开按键时，检测是否允许改变状态，如果允许，关闭钩子处理，模拟发送锁定键并播报状态
                    if *ignore_capital_key.lock().unwrap().deref() == false {
                        let state = *capital_key_state.lock().unwrap().deref();
                        capital_handle(context.clone(), state, &ignore_hook);
                    }
                }
            }

            // 所有的大写锁定键全部拦截住，满足状态改变条件时，关闭钩子处理，模拟发送锁定键
            return LRESULT(1);
        }

        drop(map); // 必须先释放锁再next()，否则可能会死锁
        next()
    })
}

// 匹配技能项的热键列表是否与当前Hook到的按键相同
fn match_keys(keys: &[Keys], map: &HashMap<Keys, bool>) -> bool {
    let pressed_key_count = map.values().filter(|i| **i).count();
    if keys.len() != pressed_key_count {
        // 如果当前按下的键数量和能力定义的键数量不一样就直接匹配失败
        return false;
    }

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
    talent.get_supported_cmd_list().iter().any(|x| match x {
        CommandType::Key(y) => match_keys(y, map),
        _ => false,
    })
}

// 执行能力项的操作
fn execute(context: Arc<Context>, talent: Talent) -> LRESULT {
    let ctx = context.clone();
    let id = talent.get_id();
    context.main_handler.spawn(async move {
        talent.perform(ctx.clone()).await;
    });
    if id == "stop_tts_output" {
        // 打断语音的能力不需要拦截键盘事件
        return LRESULT(0);
    }
    LRESULT(1)
}

// 处理大小写锁定键
fn capital_handle(context: Arc<Context>, state: bool, hook_toggle: &Mutex<bool>) {
    {
        *hook_toggle.lock().unwrap().deref_mut() = true;
    }
    send_key(VK_CAPITAL);
    {
        *hook_toggle.lock().unwrap().deref_mut() = false;
    }
    let pf = context.performer.clone();
    context.main_handler.spawn(async move {
        let info = if !state { "大写" } else { "小写" };
        pf.speak(info.to_string()).await;
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
        if !get_mouse_read_state(context.clone()) || w_param.0 != WM_MOUSEMOVE as usize {
            return next();
        }

        let info: &MsLlHookStruct = l_param.to();
        let (x, y) = (info.pt.x, info.pt.y);

        // 如果坐标差值小于10个像素，不处理直接返回
        let (old_x, old_y) = *get_old_point().lock().unwrap();
        if (x - old_x).pow(2) + (y - old_y).pow(2) < 100 {
            return next();
        }
        {
            *get_old_point().lock().unwrap() = (x, y);
        }

        mouse_read(context.clone(), x, y);
        next()
    })
}
