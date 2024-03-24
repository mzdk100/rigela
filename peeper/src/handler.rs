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
    client::PeeperClient,
    model::{CandidateList, PeeperData},
};
use win_wrap::{
    common::{HWND, LPARAM, WPARAM},
    input::{
        imm_get_candidate_list, imm_get_context, imm_get_conversion_status, imm_release_context,
        IMN_CHANGECANDIDATE, IMN_SETCONVERSIONMODE,
    },
};

//noinspection SpellCheckingInspection
/**
 * 处理输入法通知。
 * `client` peeper客户端对象。
 * `h_wnd` 窗口句柄。
 * `command` 对应IMN_开头的输入法通知常亮值。
 * `data` 附加数据。
 * */
pub(crate) fn on_ime(
    client: &PeeperClient,
    h_wnd: HWND,
    command: WPARAM,
    #[allow(unused_variables)] data: LPARAM,
) {
    match command.0 as u32 {
        IMN_CHANGECANDIDATE => {
            // 当 IME 即将更改候选窗口的内容时，将发送此消息。然后，应用程序处理此消息以显示候选窗口本身。
            let h_imc = imm_get_context(h_wnd);
            if h_imc.is_invalid() {
                return;
            }
            let Some((list, text_list)) = imm_get_candidate_list(h_imc, 0) else {
                imm_release_context(h_wnd, h_imc);
                return;
            };
            let cand = CandidateList {
                selection: list.dwSelection,
                page_start: list.dwPageStart,
                list: text_list,
            };
            client.push(PeeperData::ImeCandidateList(cand));
            imm_release_context(h_wnd, h_imc);
        }
        IMN_SETCONVERSIONMODE => {
            // 更新输入上下文的转换模式时，将发送此消息。
            let h_imc = imm_get_context(h_wnd);
            if h_imc.is_invalid() {
                return;
            }
            if let Some((conversion_mode, _)) = imm_get_conversion_status(h_imc) {
                static mut OLD_VALUE: u32 = 0;
                if unsafe { OLD_VALUE != conversion_mode.0 } {
                    unsafe { OLD_VALUE = conversion_mode.0 };
                    client.push(PeeperData::ImeConversionMode(conversion_mode.0));
                }
            }
            imm_release_context(h_wnd, h_imc);
        }
        _ => {}
    }
}

/**
 * 处理输入字符。
 * `client` peeper客户端对象。
 * `character` 输入的unicode字符。
 * */
pub(crate) fn on_input_char(client: &PeeperClient, character: WPARAM) {
    client.push(PeeperData::InputChar(character.0 as u16))
}
