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

use scintilla_sys::{SC_EOL_CR, SC_EOL_CRLF, SC_EOL_LF};

#[derive(Debug, PartialEq)]
pub enum EolMode {
    /// SC_EOL_CRLF（0）
    CrLf,
    /// SC_EOL_CR（1）
    Cr,
    /// SC_EOL_LF（2）
    Lf,
}

impl From<u32> for EolMode {
    fn from(value: u32) -> Self {
        match value {
            SC_EOL_CRLF => Self::CrLf,
            SC_EOL_CR => Self::Cr,
            SC_EOL_LF => Self::Lf,
            _ => Self::CrLf,
        }
    }
}

impl Into<u32> for EolMode {
    fn into(self) -> u32 {
        match self {
            Self::CrLf => SC_EOL_CRLF,
            Self::Cr => SC_EOL_CR,
            Self::Lf => SC_EOL_LF,
        }
    }
}
