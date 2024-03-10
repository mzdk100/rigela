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

#[derive(Deserialize, Serialize)]
pub(crate) struct PeeperPacket {
    pub(crate) name: String,
    pub(crate) data: PeeperData,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CandidateList {
    pub selection: u32,
    pub page_start: u32,
    pub list: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum PeeperData {
    Log(String),
    Quit,
    InputChar(u16),
    ImeCandidateList(CandidateList),
    ImeConversionMode(u32),
}
