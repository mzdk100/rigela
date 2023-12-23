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

use chrono::prelude::{DateTime, Local};
use std::sync::Arc;
use win_wrap::input::{VK_ESCAPE, VK_F12, VK_INSERT};
use crate::performer::Speakable;
use super::{super::{commander::CommandType::{self, Key}, launcher::Launcher}, Talented};

pub(crate) struct ExitTalent;
impl Talented for ExitTalent {
    fn get_supported_cmd_list(&self) -> Vec<CommandType> {
        vec![
            Key(vec![
                // 小键盘区域的insert加esc键
                (VK_INSERT, false),
                (VK_ESCAPE, false)
            ])
        ]
    }

    fn perform(&self, launcher: Arc<Launcher>) {
        let terminator = launcher.terminator.clone();
        let main_handler = launcher.main_handler.clone();
        main_handler.spawn(async move {
            terminator.exit().await;
        });
    }
}
pub struct CurrentTimeTalent;
impl Speakable for DateTime<Local> {
    fn get_sentence(&self) -> String {
        format!("{}", self)
    }
}
impl Talented for CurrentTimeTalent {
    fn get_supported_cmd_list(&self) -> Vec<CommandType> {
        vec![
            Key(vec![
                // 小键盘区域的insert加f12键
                (VK_INSERT, false),
                (VK_F12, false)
                    ])
        ]
    }

    fn perform(&self, launcher: Arc<Launcher>) {
        let main_handler = launcher.main_handler.clone();
        main_handler.spawn(async move {
            launcher.performer.speak(&Local::now()).await;
        });
    }
}