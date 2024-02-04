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

use serde::{Deserialize, Serialize};

//noinspection SpellCheckingInspection
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IbmeciVoiceParams {
    pub gender: i32,
    pub head_size: i32,
    pub pitch_baseline: i32,
    pub pitch_fluctuation: i32,
    pub roughness: i32,
    pub breathiness: i32,
    pub speed: i32,
    pub volume: i32,
}
impl Default for IbmeciVoiceParams {
    fn default() -> Self {
        Self {
            gender: 0,
            head_size: 50,
            pitch_baseline: 69,
            pitch_fluctuation: 69,
            roughness: 0,
            breathiness: 0,
            speed: 50,
            volume: 92,
        }
    }
}

//noinspection SpellCheckingInspection
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Proxy32Data {
    Quit,                                     // 退出
    EciSetParamsRequest(IbmeciVoiceParams),   // vvtts设置参数请求
    EciSetParamsResponse(()),                 // vvtts设置参数响应
    EciGetParamsRequest,                      // vvtts获取参数请求
    EciGetParamsResponse(IbmeciVoiceParams),  // vvtts获取参数响应
    EciSetVoiceRequest(u32),                  // vvtts设置发音人请求
    EciSetVoiceResponse(()),                  // vvtts设置发音人响应
    EciGetVoicesRequest,                      // vvtts获取发音人列表请求
    EciGetVoicesResponse(Vec<(u32, String)>), // vvtts获取发音人列表响应
    EciSynthRequest(String),                  // vvtts合成请求
    EciSynthResponse(Vec<u8>),                // vvtts合成响应
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Proxy32Packet {
    pub(crate) id: u32,
    pub data: Proxy32Data,
}
