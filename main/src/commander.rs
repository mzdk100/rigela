/*
 * Copyright (c) 2023. The RigelA open source project team and
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

use std::sync::Arc;
use win_wrap::common::LRESULT;
use win_wrap::hook::{ConvertLParam, HOOK_TYPE_KEYBOARD_LL, KbdLlHookStruct, LLKHF_EXTENDED, WindowsHook};
use crate::launcher::{Launcher};

#[derive(Clone)]
pub struct Commander {
    launcher: Option<Arc<Launcher>>,
    keyboard_hook: Option<WindowsHook>,
}

impl Commander {
    /**
     * 创建一个指挥官对象。
     * 负责收集用户的操作请求，例如快捷键、触摸动作、语音命令等，之后把这些命令调度给具体的任务。
     * */
    pub(crate) fn new() -> Self {
        Self {
            launcher: None,
            keyboard_hook: None
        }
    }

    /**
     * 让指挥官开始工作。
     * `launcher` 对对发射台的引用，可以通过此对象访问整个框架的所有API。
     * */
    pub(crate) fn apply(&mut self, launcher: Arc<Launcher>) {
        self.launcher = Some(launcher.clone());
        let terminator = launcher.terminator.clone();
        let main_handler = launcher.main_handler.clone();
        // 准备安装键盘钩子
        let keyboard_hook = WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, next| {
            let terminator = terminator.clone();
            let info: &KbdLlHookStruct = l_param.to();
            if info.vkCode == 163 && info.flags.contains(LLKHF_EXTENDED) {
                main_handler.spawn(async move {
                    terminator.exit().await;
                });
                return LRESULT::default()
            }
            next()
        });
        self.keyboard_hook = Some(keyboard_hook);
    }

    /**
     * 清理环境，后续不可以重复使用。
     * */
    pub(crate) fn dispose(&self) {
        // 解除键盘钩子
        if let Some(x) = &self.keyboard_hook {
            x.unhook()
        }
    }
}