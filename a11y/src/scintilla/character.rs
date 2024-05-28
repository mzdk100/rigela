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

use scintilla_sys::{
    SC_CHARSET_8859_15, SC_CHARSET_ANSI, SC_CHARSET_ARABIC, SC_CHARSET_BALTIC,
    SC_CHARSET_CHINESEBIG5, SC_CHARSET_CYRILLIC, SC_CHARSET_DEFAULT, SC_CHARSET_EASTEUROPE,
    SC_CHARSET_GB2312, SC_CHARSET_GREEK, SC_CHARSET_HANGUL, SC_CHARSET_HEBREW, SC_CHARSET_JOHAB,
    SC_CHARSET_MAC, SC_CHARSET_OEM, SC_CHARSET_OEM866, SC_CHARSET_RUSSIAN, SC_CHARSET_SHIFTJIS,
    SC_CHARSET_SYMBOL, SC_CHARSET_THAI, SC_CHARSET_TURKISH, SC_CHARSET_VIETNAMESE,
};

#[derive(Debug, PartialEq)]
pub enum CharacterSet {
    /// Windows✓;GTK✓;Cocoa✓ (8859-1)
    Ansi,
    /// Windows✓;Cocoa✓
    Arabic,
    /// Windows✓;Cocoa✓
    Baltic,
    /// Windows✓;Cocoa✓
    ChineseBig5,
    /// Windows✓;GTK✓ (8859-1);Cocoa✓ (8859-1)
    Default,
    /// Windows✓;GTK✓;Cocoa✓
    EastEurope,
    /// Windows✓;GTK✓;Cocoa✓
    GB2312,
    /// Windows✓;Cocoa✓
    Greek,
    /// Windows✓;GTK✓;Cocoa✓
    Hangul,
    /// Windows✓;Cocoa✓
    Hebrew,
    /// Windows✓;Cocoa✓
    Johab,
    /// Windows✓;Cocoa✓
    Mac,
    /// Windows✓;Cocoa✓
    Oem,
    /// Windows✓ (cp1251);GTK✓ (koi8-r);Cocoa✓ (cp1251)
    Russian,
    /// Windows✓;GTK✓;Cocoa✓
    ShiftJis,
    /// Windows✓;Cocoa✓
    Symbol,
    /// Windows✓;Cocoa✓
    Thai,
    /// Windows✓;Cocoa✓
    Turkish,
    /// Windows✓;Cocoa✓
    Vietnamese,
    /// GTK✓ (cp866)
    Oem866,
    /// GTK✓ (cp1251);Cocoa✓ (cp1251)
    Cyrillic,
    /// GTK✓;Cocoa✓
    Eight859_15,
}

impl From<u32> for CharacterSet {
    fn from(value: u32) -> Self {
        match value {
            SC_CHARSET_ANSI => Self::Ansi,
            SC_CHARSET_ARABIC => Self::Arabic,
            SC_CHARSET_BALTIC => Self::Baltic,
            SC_CHARSET_CHINESEBIG5 => Self::ChineseBig5,
            SC_CHARSET_DEFAULT => Self::Default,
            SC_CHARSET_EASTEUROPE => Self::EastEurope,
            SC_CHARSET_GB2312 => Self::GB2312,
            SC_CHARSET_GREEK => Self::Greek,
            SC_CHARSET_HANGUL => Self::Hangul,
            SC_CHARSET_HEBREW => Self::Hebrew,
            SC_CHARSET_JOHAB => Self::Johab,
            SC_CHARSET_MAC => Self::Mac,
            SC_CHARSET_OEM => Self::Oem,
            SC_CHARSET_RUSSIAN => Self::Russian,
            SC_CHARSET_SHIFTJIS => Self::ShiftJis,
            SC_CHARSET_SYMBOL => Self::Symbol,
            SC_CHARSET_THAI => Self::Thai,
            SC_CHARSET_TURKISH => Self::Turkish,
            SC_CHARSET_VIETNAMESE => Self::Vietnamese,
            SC_CHARSET_OEM866 => Self::Oem866,
            SC_CHARSET_CYRILLIC => Self::Cyrillic,
            SC_CHARSET_8859_15 => Self::Eight859_15,
            _ => Self::Default,
        }
    }
}

impl Into<u32> for CharacterSet {
    fn into(self) -> u32 {
        match self {
            Self::Ansi => SC_CHARSET_ANSI,
            Self::Arabic => SC_CHARSET_ARABIC,
            Self::Baltic => SC_CHARSET_BALTIC,
            Self::ChineseBig5 => SC_CHARSET_CHINESEBIG5,
            Self::Default => SC_CHARSET_DEFAULT,
            Self::EastEurope => SC_CHARSET_EASTEUROPE,
            Self::GB2312 => SC_CHARSET_GB2312,
            Self::Greek => SC_CHARSET_GREEK,
            Self::Hangul => SC_CHARSET_HANGUL,
            Self::Hebrew => SC_CHARSET_HEBREW,
            Self::Johab => SC_CHARSET_JOHAB,
            Self::Mac => SC_CHARSET_MAC,
            Self::Oem => SC_CHARSET_OEM,
            Self::Russian => SC_CHARSET_RUSSIAN,
            Self::ShiftJis => SC_CHARSET_SHIFTJIS,
            Self::Symbol => SC_CHARSET_SYMBOL,
            Self::Thai => SC_CHARSET_THAI,
            Self::Turkish => SC_CHARSET_TURKISH,
            Self::Vietnamese => SC_CHARSET_VIETNAMESE,
            Self::Oem866 => SC_CHARSET_OEM866,
            Self::Cyrillic => SC_CHARSET_CYRILLIC,
            Self::Eight859_15 => SC_CHARSET_8859_15,
        }
    }
}
