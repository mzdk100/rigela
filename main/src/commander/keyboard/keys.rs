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
use std::fmt;
use win_wrap::input::{
    VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_ABNT_C1,
    VK_ABNT_C2, VK_ACCEPT, VK_ADD, VK_APPS, VK_ATTN, VK_B, VK_BACK, VK_BROWSER_BACK,
    VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME, VK_BROWSER_REFRESH,
    VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_C, VK_CANCEL, VK_CAPITAL, VK_CLEAR, VK_CONTROL,
    VK_CONVERT, VK_CRSEL, VK_D, VK_DBE_ALPHANUMERIC, VK_DBE_CODEINPUT, VK_DBE_DBCSCHAR,
    VK_DBE_DETERMINESTRING, VK_DBE_ENTERDLGCONVERSIONMODE, VK_DBE_ENTERIMECONFIGMODE,
    VK_DBE_FLUSHSTRING, VK_DBE_HIRAGANA, VK_DBE_KATAKANA, VK_DBE_NOCODEINPUT, VK_DBE_ROMAN,
    VK_DBE_SBCSCHAR, VK_DECIMAL, VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END, VK_ESCAPE,
    VK_EXECUTE, VK_F, VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17,
    VK_F18, VK_F19, VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6,
    VK_F7, VK_F8, VK_F9, VK_FINAL, VK_G, VK_GAMEPAD_A, VK_GAMEPAD_B, VK_GAMEPAD_DPAD_DOWN,
    VK_GAMEPAD_DPAD_LEFT, VK_GAMEPAD_DPAD_RIGHT, VK_GAMEPAD_DPAD_UP, VK_GAMEPAD_LEFT_SHOULDER,
    VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON, VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
    VK_GAMEPAD_LEFT_THUMBSTICK_LEFT, VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
    VK_GAMEPAD_LEFT_THUMBSTICK_UP, VK_GAMEPAD_LEFT_TRIGGER, VK_GAMEPAD_MENU,
    VK_GAMEPAD_RIGHT_SHOULDER, VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
    VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN, VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
    VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT, VK_GAMEPAD_RIGHT_THUMBSTICK_UP, VK_GAMEPAD_RIGHT_TRIGGER,
    VK_GAMEPAD_VIEW, VK_GAMEPAD_X, VK_GAMEPAD_Y, VK_H, VK_HANGEUL, VK_HANJA, VK_HELP, VK_HOME,
    VK_I, VK_ICO_00, VK_ICO_CLEAR, VK_ICO_HELP, VK_IME_OFF, VK_IME_ON, VK_INSERT, VK_J, VK_JUNJA,
    VK_K, VK_L, VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT, VK_LBUTTON,
    VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON, VK_MEDIA_NEXT_TRACK,
    VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_MENU, VK_MODECHANGE, VK_MULTIPLY,
    VK_N, VK_NAVIGATION_ACCEPT, VK_NAVIGATION_CANCEL, VK_NAVIGATION_DOWN, VK_NAVIGATION_LEFT,
    VK_NAVIGATION_MENU, VK_NAVIGATION_RIGHT, VK_NAVIGATION_UP, VK_NAVIGATION_VIEW, VK_NEXT,
    VK_NONCONVERT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4,
    VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1, VK_OEM_102,
    VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_AX, VK_OEM_CLEAR,
    VK_OEM_COMMA, VK_OEM_CUSEL, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA, VK_OEM_FJ_MASSHOU, VK_OEM_FJ_ROYA,
    VK_OEM_FJ_TOUROKU, VK_OEM_JUMP, VK_OEM_MINUS, VK_OEM_PA1, VK_OEM_PA2, VK_OEM_PA3,
    VK_OEM_PERIOD, VK_OEM_PLUS, VK_OEM_RESET, VK_OEM_WSCTRL, VK_P, VK_PACKET, VK_PAUSE, VK_PRINT,
    VK_PRIOR, VK_PROCESSKEY, VK_Q, VK_R, VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU,
    VK_RSHIFT, VK_RWIN, VK_S, VK_SCROLL, VK_SELECT, VK_SEPARATOR, VK_SHIFT, VK_SLEEP, VK_SNAPSHOT,
    VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V, VK_VOLUME_DOWN, VK_VOLUME_MUTE,
    VK_VOLUME_UP, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2, VK_Y, VK_Z,
};

//noinspection SpellCheckingInspection
/**
 * 特别注意： 命名没有完善， 小键盘 VkNumPad 开头(大写P)， 不要与 VkNumpad (小写p)混淆
 * 键盘枚举
 * */
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum Keys {
    VkRigelA,
    VkCtrl,
    VkShift,
    VkAlt,
    VkWin,
    VkNumPad0,
    VkNumPad1,
    VkNumPad2,
    VkNumPad3,
    VkNumPad4,
    VkNumPad5,
    VkNumPad6,
    VkNumPad7,
    VkNumPad8,
    VkNumPad9,
    VkNumPadDiv,
    VkNumPadMul,
    VkNumPadDot,
    VkNumPadReturn,
    Vk0,
    Vk1,
    Vk2,
    Vk3,
    Vk4,
    Vk5,
    Vk6,
    Vk7,
    Vk8,
    Vk9,
    VkA,
    VkAbntC1,
    VkAbntC2,
    VkAccept,
    VkAdd,
    VkApps,
    VkAttn,
    VkB,
    VkBack,
    VkBrowserBack,
    VkBrowserFavorites,
    VkBrowserForward,
    VkBrowserHome,
    VkBrowserRefresh,
    VkBrowserSearch,
    VkBrowserStop,
    VkC,
    VkCancel,
    VkCapital,
    VkClear,
    VkConvert,
    VkCrsel,
    VkD,
    VkDbeAlphanumeric,
    VkDbeCodeinput,
    VkDbeDbcschar,
    VkDbeDeterminestring,
    VkDbeEnterdlgconversionmode,
    VkDbeEnterimeconfigmode,
    VkDbeFlushstring,
    VkDbeHiragana,
    VkDbeKatakana,
    VkDbeNocodeinput,
    VkDbeRoman,
    VkDbeSbcschar,
    VkDecimal,
    VkDelete,
    VkDivide,
    VkDown,
    VkE,
    VkEnd,
    VkEscape,
    VkExecute,
    VkF,
    VkF1,
    VkF10,
    VkF11,
    VkF12,
    VkF13,
    VkF14,
    VkF15,
    VkF16,
    VkF17,
    VkF18,
    VkF19,
    VkF2,
    VkF20,
    VkF21,
    VkF22,
    VkF23,
    VkF24,
    VkF3,
    VkF4,
    VkF5,
    VkF6,
    VkF7,
    VkF8,
    VkF9,
    VkFinal,
    VkG,
    VkGamepadA,
    VkGamepadB,
    VkGamepadDpadDown,
    VkGamepadDpadLeft,
    VkGamepadDpadRight,
    VkGamepadDpadUp,
    VkGamepadLeftShoulder,
    VkGamepadLeftThumbstickButton,
    VkGamepadLeftThumbstickDown,
    VkGamepadLeftThumbstickLeft,
    VkGamepadLeftThumbstickRight,
    VkGamepadLeftThumbstickUp,
    VkGamepadLeftTrigger,
    VkGamepadMenu,
    VkGamepadRightShoulder,
    VkGamepadRightThumbstickButton,
    VkGamepadRightThumbstickDown,
    VkGamepadRightThumbstickLeft,
    VkGamepadRightThumbstickRight,
    VkGamepadRightThumbstickUp,
    VkGamepadRightTrigger,
    VkGamepadView,
    VkGamepadX,
    VkGamepadY,
    VkH,
    VkHangeul,
    VkHanja,
    VkHelp,
    VkHome,
    VkI,
    VkIco00,
    VkIcoClear,
    VkIcoHelp,
    VkImeOff,
    VkImeOn,
    VkInsert,
    VkJ,
    VkJunja,
    VkK,
    VkL,
    VkLaunchApp1,
    VkLaunchApp2,
    VkLaunchMail,
    VkLaunchMediaSelect,
    VkLbutton,
    VkLcontrol,
    VkLeft,
    VkLmenu,
    VkLshift,
    VkLwin,
    VkM,
    VkMbutton,
    VkMediaNextTrack,
    VkMediaPlayPause,
    VkMediaPrevTrack,
    VkMediaStop,
    VkMenu,
    VkModechange,
    VkMultiply,
    VkN,
    VkNavigationAccept,
    VkNavigationCancel,
    VkNavigationDown,
    VkNavigationLeft,
    VkNavigationMenu,
    VkNavigationRight,
    VkNavigationUp,
    VkNavigationView,
    VkNext,
    VkNonconvert,
    VkNumlock,
    VkNumpad0,
    VkNumpad1,
    VkNumpad2,
    VkNumpad3,
    VkNumpad4,
    VkNumpad5,
    VkNumpad6,
    VkNumpad7,
    VkNumpad8,
    VkNumpad9,
    VkO,
    VkOem1,
    VkOem102,
    VkOem2,
    VkOem3,
    VkOem4,
    VkOem5,
    VkOem6,
    VkOem7,
    VkOem8,
    VkOemAx,
    VkOemClear,
    VkOemComma,
    VkOemCusel,
    VkOemFjJisho,
    VkOemFjLoya,
    VkOemFjMasshou,
    VkOemFjRoya,
    VkOemFjTouroku,
    VkOemJump,
    VkOemMinus,
    VkOemPa1,
    VkOemPa2,
    VkOemPa3,
    VkOemPeriod,
    VkOemPlus,
    VkOemReset,
    VkOemWsctrl,
    VkP,
    VkPacket,
    VkPause,
    VkPrint,
    VkPrior,
    VkProcesskey,
    VkQ,
    VkR,
    VkRbutton,
    VkRcontrol,
    VkReturn,
    VkRight,
    VkRmenu,
    VkRshift,
    VkRwin,
    VkS,
    VkScroll,
    VkSelect,
    VkSeparator,
    VkSleep,
    VkSnapshot,
    VkSpace,
    VkSubtract,
    VkT,
    VkTab,
    VkU,
    VkUp,
    VkV,
    VkVolumeDown,
    VkVolumeMute,
    VkVolumeUp,
    VkW,
    VkX,
    VkXbutton1,
    VkXbutton2,
    VkY,
    VkZ,

    VkNone,
}

impl Keys {
    pub(crate) fn get_code(&self) -> Option<VIRTUAL_KEY> {
        match self {
            Self::VkRigelA => None,
            Self::VkCtrl => Some(VK_CONTROL),
            Self::VkShift => Some(VK_SHIFT),
            Self::VkAlt => Some(VK_MENU),
            Self::VkWin => Some(VK_LWIN),
            Self::VkNumPad0 => Some(VK_NUMPAD0),
            Self::VkNumPad1 => Some(VK_NUMPAD1),
            Self::VkNumPad2 => Some(VK_NUMPAD2),
            Self::VkNumPad3 => Some(VK_NUMPAD3),
            Self::VkNumPad4 => Some(VK_NUMPAD4),
            Self::VkNumPad5 => Some(VK_NUMPAD5),
            Self::VkNumPad6 => Some(VK_NUMPAD6),
            Self::VkNumPad7 => Some(VK_NUMPAD7),
            Self::VkNumPad8 => Some(VK_NUMPAD8),
            Self::VkNumPad9 => Some(VK_NUMPAD9),
            Self::VkNumPadDiv => Some(VK_DIVIDE),
            Self::VkNumPadMul => Some(VK_MULTIPLY),
            Self::VkNumPadDot => Some(VK_DELETE),
            Self::VkNumPadReturn => Some(VK_RETURN),
            Self::Vk0 => Some(VK_0),
            Self::Vk1 => Some(VK_1),
            Self::Vk2 => Some(VK_2),
            Self::Vk3 => Some(VK_3),
            Self::Vk4 => Some(VK_4),
            Self::Vk5 => Some(VK_5),
            Self::Vk6 => Some(VK_6),
            Self::Vk7 => Some(VK_7),
            Self::Vk8 => Some(VK_8),
            Self::Vk9 => Some(VK_9),
            Self::VkA => Some(VK_A),
            Self::VkAbntC1 => Some(VK_ABNT_C1),
            Self::VkAbntC2 => Some(VK_ABNT_C2),
            Self::VkAccept => Some(VK_ACCEPT),
            Self::VkAdd => Some(VK_ADD),
            Self::VkApps => Some(VK_APPS),
            Self::VkAttn => Some(VK_ATTN),
            Self::VkB => Some(VK_B),
            Self::VkBack => Some(VK_BACK),
            Self::VkBrowserBack => Some(VK_BROWSER_BACK),
            Self::VkBrowserFavorites => Some(VK_BROWSER_FAVORITES),
            Self::VkBrowserForward => Some(VK_BROWSER_FORWARD),
            Self::VkBrowserHome => Some(VK_BROWSER_HOME),
            Self::VkBrowserRefresh => Some(VK_BROWSER_REFRESH),
            Self::VkBrowserSearch => Some(VK_BROWSER_SEARCH),
            Self::VkBrowserStop => Some(VK_BROWSER_STOP),
            Self::VkC => Some(VK_C),
            Self::VkCancel => Some(VK_CANCEL),
            Self::VkCapital => Some(VK_CAPITAL),
            Self::VkClear => Some(VK_CLEAR),
            Self::VkConvert => Some(VK_CONVERT),
            Self::VkCrsel => Some(VK_CRSEL),
            Self::VkD => Some(VK_D),
            Self::VkDbeAlphanumeric => Some(VK_DBE_ALPHANUMERIC),
            Self::VkDbeCodeinput => Some(VK_DBE_CODEINPUT),
            Self::VkDbeDbcschar => Some(VK_DBE_DBCSCHAR),
            Self::VkDbeDeterminestring => Some(VK_DBE_DETERMINESTRING),
            Self::VkDbeEnterdlgconversionmode => Some(VK_DBE_ENTERDLGCONVERSIONMODE),
            Self::VkDbeEnterimeconfigmode => Some(VK_DBE_ENTERIMECONFIGMODE),
            Self::VkDbeFlushstring => Some(VK_DBE_FLUSHSTRING),
            Self::VkDbeHiragana => Some(VK_DBE_HIRAGANA),
            Self::VkDbeKatakana => Some(VK_DBE_KATAKANA),
            Self::VkDbeNocodeinput => Some(VK_DBE_NOCODEINPUT),
            Self::VkDbeRoman => Some(VK_DBE_ROMAN),
            Self::VkDbeSbcschar => Some(VK_DBE_SBCSCHAR),
            Self::VkDecimal => Some(VK_DECIMAL),
            Self::VkDelete => Some(VK_DELETE),
            Self::VkDivide => Some(VK_DIVIDE),
            Self::VkDown => Some(VK_DOWN),
            Self::VkE => Some(VK_E),
            Self::VkEnd => Some(VK_END),
            Self::VkEscape => Some(VK_ESCAPE),
            Self::VkExecute => Some(VK_EXECUTE),
            Self::VkF => Some(VK_F),
            Self::VkF1 => Some(VK_F1),
            Self::VkF10 => Some(VK_F10),
            Self::VkF11 => Some(VK_F11),
            Self::VkF12 => Some(VK_F12),
            Self::VkF13 => Some(VK_F13),
            Self::VkF14 => Some(VK_F14),
            Self::VkF15 => Some(VK_F15),
            Self::VkF16 => Some(VK_F16),
            Self::VkF17 => Some(VK_F17),
            Self::VkF18 => Some(VK_F18),
            Self::VkF19 => Some(VK_F19),
            Self::VkF2 => Some(VK_2),
            Self::VkF20 => Some(VK_F20),
            Self::VkF21 => Some(VK_F21),
            Self::VkF22 => Some(VK_F22),
            Self::VkF23 => Some(VK_F23),
            Self::VkF24 => Some(VK_F24),
            Self::VkF3 => Some(VK_F3),
            Self::VkF4 => Some(VK_F4),
            Self::VkF5 => Some(VK_F5),
            Self::VkF6 => Some(VK_F6),
            Self::VkF7 => Some(VK_F7),
            Self::VkF8 => Some(VK_F8),
            Self::VkF9 => Some(VK_F9),
            Self::VkFinal => Some(VK_FINAL),
            Self::VkG => Some(VK_G),
            Self::VkGamepadA => Some(VK_GAMEPAD_A),
            Self::VkGamepadB => Some(VK_GAMEPAD_B),
            Self::VkGamepadDpadDown => Some(VK_GAMEPAD_DPAD_DOWN),
            Self::VkGamepadDpadLeft => Some(VK_GAMEPAD_DPAD_LEFT),
            Self::VkGamepadDpadRight => Some(VK_GAMEPAD_DPAD_RIGHT),
            Self::VkGamepadDpadUp => Some(VK_GAMEPAD_DPAD_UP),
            Self::VkGamepadLeftShoulder => Some(VK_GAMEPAD_LEFT_SHOULDER),
            Self::VkGamepadLeftThumbstickButton => Some(VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON),
            Self::VkGamepadLeftThumbstickDown => Some(VK_GAMEPAD_LEFT_THUMBSTICK_DOWN),
            Self::VkGamepadLeftThumbstickLeft => Some(VK_GAMEPAD_LEFT_THUMBSTICK_LEFT),
            Self::VkGamepadLeftThumbstickRight => Some(VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT),
            Self::VkGamepadLeftThumbstickUp => Some(VK_GAMEPAD_LEFT_THUMBSTICK_UP),
            Self::VkGamepadLeftTrigger => Some(VK_GAMEPAD_LEFT_TRIGGER),
            Self::VkGamepadMenu => Some(VK_GAMEPAD_MENU),
            Self::VkGamepadRightShoulder => Some(VK_GAMEPAD_RIGHT_SHOULDER),
            Self::VkGamepadRightThumbstickButton => Some(VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON),
            Self::VkGamepadRightThumbstickDown => Some(VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN),
            Self::VkGamepadRightThumbstickLeft => Some(VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT),
            Self::VkGamepadRightThumbstickRight => Some(VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT),
            Self::VkGamepadRightThumbstickUp => Some(VK_GAMEPAD_RIGHT_THUMBSTICK_UP),
            Self::VkGamepadRightTrigger => Some(VK_GAMEPAD_RIGHT_TRIGGER),
            Self::VkGamepadView => Some(VK_GAMEPAD_VIEW),
            Self::VkGamepadX => Some(VK_GAMEPAD_X),
            Self::VkGamepadY => Some(VK_GAMEPAD_Y),
            Self::VkH => Some(VK_H),
            Self::VkHangeul => Some(VK_HANGEUL),
            Self::VkHanja => Some(VK_HANJA),
            Self::VkHelp => Some(VK_HELP),
            Self::VkHome => Some(VK_HOME),
            Self::VkI => Some(VK_I),
            Self::VkIco00 => Some(VK_ICO_00),
            Self::VkIcoClear => Some(VK_ICO_CLEAR),
            Self::VkIcoHelp => Some(VK_ICO_HELP),
            Self::VkImeOff => Some(VK_IME_OFF),
            Self::VkImeOn => Some(VK_IME_ON),
            Self::VkInsert => Some(VK_INSERT),
            Self::VkJ => Some(VK_J),
            Self::VkJunja => Some(VK_JUNJA),
            Self::VkK => Some(VK_K),
            Self::VkL => Some(VK_L),
            Self::VkLaunchApp1 => Some(VK_LAUNCH_APP1),
            Self::VkLaunchApp2 => Some(VK_LAUNCH_APP2),
            Self::VkLaunchMail => Some(VK_LAUNCH_MAIL),
            Self::VkLaunchMediaSelect => Some(VK_LAUNCH_MEDIA_SELECT),
            Self::VkLbutton => Some(VK_LBUTTON),
            Self::VkLcontrol => Some(VK_LCONTROL),
            Self::VkLeft => Some(VK_LEFT),
            Self::VkLmenu => Some(VK_LMENU),
            Self::VkLshift => Some(VK_LSHIFT),
            Self::VkLwin => Some(VK_LWIN),
            Self::VkM => Some(VK_M),
            Self::VkMbutton => Some(VK_MBUTTON),
            Self::VkMediaNextTrack => Some(VK_MEDIA_NEXT_TRACK),
            Self::VkMediaPlayPause => Some(VK_MEDIA_PLAY_PAUSE),
            Self::VkMediaPrevTrack => Some(VK_MEDIA_PREV_TRACK),
            Self::VkMediaStop => Some(VK_MEDIA_STOP),
            Self::VkMenu => Some(VK_MENU),
            Self::VkModechange => Some(VK_MODECHANGE),
            Self::VkMultiply => Some(VK_MULTIPLY),
            Self::VkN => Some(VK_N),
            Self::VkNavigationAccept => Some(VK_NAVIGATION_ACCEPT),
            Self::VkNavigationCancel => Some(VK_NAVIGATION_CANCEL),
            Self::VkNavigationDown => Some(VK_NAVIGATION_DOWN),
            Self::VkNavigationLeft => Some(VK_NAVIGATION_LEFT),
            Self::VkNavigationMenu => Some(VK_NAVIGATION_MENU),
            Self::VkNavigationRight => Some(VK_NAVIGATION_RIGHT),
            Self::VkNavigationUp => Some(VK_NAVIGATION_UP),
            Self::VkNavigationView => Some(VK_NAVIGATION_VIEW),
            Self::VkNext => Some(VK_NEXT),
            Self::VkNonconvert => Some(VK_NONCONVERT),
            Self::VkNumlock => Some(VK_NUMLOCK),
            Self::VkNumpad0 => Some(VK_NUMPAD0),
            Self::VkNumpad1 => Some(VK_NUMPAD1),
            Self::VkNumpad2 => Some(VK_NUMPAD2),
            Self::VkNumpad3 => Some(VK_NUMPAD3),
            Self::VkNumpad4 => Some(VK_NUMPAD4),
            Self::VkNumpad5 => Some(VK_NUMPAD5),
            Self::VkNumpad6 => Some(VK_NUMPAD6),
            Self::VkNumpad7 => Some(VK_NUMPAD7),
            Self::VkNumpad8 => Some(VK_NUMPAD8),
            Self::VkNumpad9 => Some(VK_NUMPAD9),
            Self::VkO => Some(VK_O),
            Self::VkOem1 => Some(VK_OEM_1),
            Self::VkOem102 => Some(VK_OEM_102),
            Self::VkOem2 => Some(VK_OEM_2),
            Self::VkOem3 => Some(VK_OEM_3),
            Self::VkOem4 => Some(VK_OEM_4),
            Self::VkOem5 => Some(VK_OEM_5),
            Self::VkOem6 => Some(VK_OEM_6),
            Self::VkOem7 => Some(VK_OEM_7),
            Self::VkOem8 => Some(VK_OEM_8),
            Self::VkOemAx => Some(VK_OEM_AX),
            Self::VkOemClear => Some(VK_OEM_CLEAR),
            Self::VkOemComma => Some(VK_OEM_COMMA),
            Self::VkOemCusel => Some(VK_OEM_CUSEL),
            Self::VkOemFjJisho => Some(VK_OEM_FJ_JISHO),
            Self::VkOemFjLoya => Some(VK_OEM_FJ_LOYA),
            Self::VkOemFjMasshou => Some(VK_OEM_FJ_MASSHOU),
            Self::VkOemFjRoya => Some(VK_OEM_FJ_ROYA),
            Self::VkOemFjTouroku => Some(VK_OEM_FJ_TOUROKU),
            Self::VkOemJump => Some(VK_OEM_JUMP),
            Self::VkOemMinus => Some(VK_OEM_MINUS),
            Self::VkOemPa1 => Some(VK_OEM_PA1),
            Self::VkOemPa2 => Some(VK_OEM_PA2),
            Self::VkOemPa3 => Some(VK_OEM_PA3),
            Self::VkOemPeriod => Some(VK_OEM_PERIOD),
            Self::VkOemPlus => Some(VK_OEM_PLUS),
            Self::VkOemReset => Some(VK_OEM_RESET),
            Self::VkOemWsctrl => Some(VK_OEM_WSCTRL),
            Self::VkP => Some(VK_P),
            Self::VkPacket => Some(VK_PACKET),
            Self::VkPause => Some(VK_PAUSE),
            Self::VkPrint => Some(VK_PRINT),
            Self::VkPrior => Some(VK_PRIOR),
            Self::VkProcesskey => Some(VK_PROCESSKEY),
            Self::VkQ => Some(VK_Q),
            Self::VkR => Some(VK_R),
            Self::VkRbutton => Some(VK_RBUTTON),
            Self::VkRcontrol => Some(VK_RCONTROL),
            Self::VkReturn => Some(VK_RETURN),
            Self::VkRight => Some(VK_RIGHT),
            Self::VkRmenu => Some(VK_RMENU),
            Self::VkRshift => Some(VK_RSHIFT),
            Self::VkRwin => Some(VK_RWIN),
            Self::VkS => Some(VK_S),
            Self::VkScroll => Some(VK_SCROLL),
            Self::VkSelect => Some(VK_SELECT),
            Self::VkSeparator => Some(VK_SEPARATOR),
            Self::VkSleep => Some(VK_SLEEP),
            Self::VkSnapshot => Some(VK_SNAPSHOT),
            Self::VkSpace => Some(VK_SPACE),
            Self::VkSubtract => Some(VK_SUBTRACT),
            Self::VkT => Some(VK_T),
            Self::VkTab => Some(VK_TAB),
            Self::VkU => Some(VK_U),
            Self::VkUp => Some(VK_UP),
            Self::VkV => Some(VK_V),
            Self::VkVolumeDown => Some(VK_VOLUME_DOWN),
            Self::VkVolumeMute => Some(VK_VOLUME_MUTE),
            Self::VkVolumeUp => Some(VK_VOLUME_UP),
            Self::VkW => Some(VK_W),
            Self::VkX => Some(VK_X),
            Self::VkXbutton1 => Some(VK_XBUTTON1),
            Self::VkXbutton2 => Some(VK_XBUTTON2),
            Self::VkY => Some(VK_Y),
            Self::VkZ => Some(VK_Z),
            Self::VkNone => None,
        }
    }

    /// 转换RigelA键, 非Rigela键原键返回
    pub(crate) fn trans_rigela(self) -> Self {
        const RIGELA_KEY: [Keys; 3] = [Keys::VkNumPad0, Keys::VkInsert, Keys::VkCapital];

        if RIGELA_KEY.contains(&self) {
            Keys::VkRigelA
        } else {
            self
        }
    }

    /// 判断是否为组合键的辅助案件，一共5个：[RigelA, Ctrl, Alt, Shift, Win]
    pub(crate) fn is_modifierkey(&self) -> bool {
        const KEYS: [Keys; 5] = [
            Keys::VkShift,
            Keys::VkCtrl,
            Keys::VkAlt,
            Keys::VkWin,
            Keys::VkRigelA,
        ];
        KEYS.contains(self)
    }
}

impl From<(u32, bool)> for Keys {
    //noinspection SpellCheckingInspection
    /// 从原始虚拟键码和扩展码转换为按键枚举
    fn from(info: (u32, bool)) -> Self {
        let (vk, ext) = (VIRTUAL_KEY { 0: info.0 as u16 }, info.1);

        match (vk, ext) {
            // 读屏主热键
            (VK_INSERT, false) => Self::VkNumPad0,
            (VK_INSERT, true) => Self::VkInsert,
            (VK_CAPITAL, false) => Self::VkCapital,

            // 小键盘数字键
            (VK_END, false) => Self::VkNumPad1,
            (VK_DOWN, false) => Self::VkNumPad2,
            (VK_NEXT, false) => Self::VkNumPad3,
            (VK_LEFT, false) => Self::VkNumPad4,
            (VK_CLEAR, false) => Self::VkNumPad5,
            (VK_RIGHT, false) => Self::VkNumPad6,
            (VK_HOME, false) => Self::VkNumPad7,
            (VK_UP, false) => Self::VkNumPad8,
            (VK_PRIOR, false) => Self::VkNumPad9,
            (VK_DIVIDE, true) => Self::VkNumPadDiv,
            (VK_MULTIPLY, false) => Self::VkNumPadMul,
            (VK_DELETE, false) => Self::VkNumPadDot,
            (VK_RETURN, true) => Self::VkNumPadReturn,

            // Ctrl键
            (VK_LCONTROL, false) => Self::VkCtrl,
            (VK_RCONTROL, true) => Self::VkCtrl,
            (VK_CONTROL, false) => Self::VkCtrl,

            //Shift
            (VK_LSHIFT, false) => Self::VkShift,
            (VK_RSHIFT, true) => Self::VkShift,

            //Alt
            (VK_LMENU, false) => Self::VkAlt,
            (VK_RMENU, true) => Self::VkAlt,

            //Win
            (VK_LWIN, true) => Self::VkWin,
            (VK_RWIN, true) => Self::VkWin,

            (VK_0, false) => Self::Vk0,
            (VK_1, false) => Self::Vk1,
            (VK_2, false) => Self::Vk2,
            (VK_3, false) => Self::Vk3,
            (VK_4, false) => Self::Vk4,
            (VK_5, false) => Self::Vk5,
            (VK_6, false) => Self::Vk6,
            (VK_7, false) => Self::Vk7,
            (VK_8, false) => Self::Vk8,
            (VK_9, false) => Self::Vk9,
            (VK_A, false) => Self::VkA,
            (VK_ABNT_C1, false) => Self::VkAbntC1,
            (VK_ABNT_C2, false) => Self::VkAbntC2,
            (VK_ACCEPT, false) => Self::VkAccept,
            (VK_ADD, false) => Self::VkAdd,
            (VK_APPS, true) => Self::VkApps,
            (VK_ATTN, false) => Self::VkAttn,
            (VK_B, false) => Self::VkB,
            (VK_BACK, false) => Self::VkBack,
            (VK_BROWSER_BACK, false) => Self::VkBrowserBack,
            (VK_BROWSER_FAVORITES, false) => Self::VkBrowserFavorites,
            (VK_BROWSER_FORWARD, false) => Self::VkBrowserForward,
            (VK_BROWSER_HOME, false) => Self::VkBrowserHome,
            (VK_BROWSER_REFRESH, false) => Self::VkBrowserRefresh,
            (VK_BROWSER_SEARCH, false) => Self::VkBrowserSearch,
            (VK_BROWSER_STOP, false) => Self::VkBrowserStop,
            (VK_C, false) => Self::VkC,
            (VK_CANCEL, false) => Self::VkCancel,
            (VK_CLEAR, true) => Self::VkClear,
            (VK_CONVERT, false) => Self::VkConvert,
            (VK_CRSEL, false) => Self::VkCrsel,
            (VK_D, false) => Self::VkD,
            (VK_DBE_ALPHANUMERIC, false) => Self::VkDbeAlphanumeric,
            (VK_DBE_CODEINPUT, false) => Self::VkDbeCodeinput,
            (VK_DBE_DBCSCHAR, false) => Self::VkDbeDbcschar,
            (VK_DBE_DETERMINESTRING, false) => Self::VkDbeDeterminestring,
            (VK_DBE_ENTERDLGCONVERSIONMODE, false) => Self::VkDbeEnterdlgconversionmode,
            (VK_DBE_ENTERIMECONFIGMODE, false) => Self::VkDbeEnterimeconfigmode,
            // (VK_DBE_ENTERWORDREGISTERMODE, false) => Self::VkDbeEnterwordregistermode,
            (VK_DBE_FLUSHSTRING, false) => Self::VkDbeFlushstring,
            (VK_DBE_HIRAGANA, false) => Self::VkDbeHiragana,
            (VK_DBE_KATAKANA, false) => Self::VkDbeKatakana,
            (VK_DBE_NOCODEINPUT, false) => Self::VkDbeNocodeinput,
            // (VK_DBE_NOROMAN, false) => Self::VkDbeNoroman,
            (VK_DBE_ROMAN, false) => Self::VkDbeRoman,
            (VK_DBE_SBCSCHAR, false) => Self::VkDbeSbcschar,
            (VK_DECIMAL, false) => Self::VkDecimal,
            (VK_DELETE, true) => Self::VkDelete,
            (VK_DIVIDE, false) => Self::VkDivide,
            (VK_DOWN, true) => Self::VkDown,
            (VK_E, false) => Self::VkE,
            (VK_END, true) => Self::VkEnd,
            // (VK_EREOF, false) => Self::VkEreof,
            (VK_ESCAPE, false) => Self::VkEscape,
            (VK_EXECUTE, false) => Self::VkExecute,
            // (VK_EXSEL, false) => Self::VkExsel,
            (VK_F, false) => Self::VkF,
            (VK_F1, false) => Self::VkF1,
            (VK_F10, false) => Self::VkF10,
            (VK_F11, false) => Self::VkF11,
            (VK_F12, false) => Self::VkF12,
            (VK_F13, false) => Self::VkF13,
            (VK_F14, false) => Self::VkF14,
            (VK_F15, false) => Self::VkF15,
            (VK_F16, false) => Self::VkF16,
            (VK_F17, false) => Self::VkF17,
            (VK_F18, false) => Self::VkF18,
            (VK_F19, false) => Self::VkF19,
            (VK_F2, false) => Self::VkF2,
            (VK_F20, false) => Self::VkF20,
            (VK_F21, false) => Self::VkF21,
            (VK_F22, false) => Self::VkF22,
            (VK_F23, false) => Self::VkF23,
            (VK_F24, false) => Self::VkF24,
            (VK_F3, false) => Self::VkF3,
            (VK_F4, false) => Self::VkF4,
            (VK_F5, false) => Self::VkF5,
            (VK_F6, false) => Self::VkF6,
            (VK_F7, false) => Self::VkF7,
            (VK_F8, false) => Self::VkF8,
            (VK_F9, false) => Self::VkF9,
            (VK_FINAL, false) => Self::VkFinal,
            (VK_G, false) => Self::VkG,
            (VK_GAMEPAD_A, false) => Self::VkGamepadA,
            (VK_GAMEPAD_B, false) => Self::VkGamepadB,
            (VK_GAMEPAD_DPAD_DOWN, false) => Self::VkGamepadDpadDown,
            (VK_GAMEPAD_DPAD_LEFT, false) => Self::VkGamepadDpadLeft,
            (VK_GAMEPAD_DPAD_RIGHT, false) => Self::VkGamepadDpadRight,
            (VK_GAMEPAD_DPAD_UP, false) => Self::VkGamepadDpadUp,
            (VK_GAMEPAD_LEFT_SHOULDER, false) => Self::VkGamepadLeftShoulder,
            (VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON, false) => Self::VkGamepadLeftThumbstickButton,
            (VK_GAMEPAD_LEFT_THUMBSTICK_DOWN, false) => Self::VkGamepadLeftThumbstickDown,
            (VK_GAMEPAD_LEFT_THUMBSTICK_LEFT, false) => Self::VkGamepadLeftThumbstickLeft,
            (VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT, false) => Self::VkGamepadLeftThumbstickRight,
            (VK_GAMEPAD_LEFT_THUMBSTICK_UP, false) => Self::VkGamepadLeftThumbstickUp,
            (VK_GAMEPAD_LEFT_TRIGGER, false) => Self::VkGamepadLeftTrigger,
            (VK_GAMEPAD_MENU, false) => Self::VkGamepadMenu,
            (VK_GAMEPAD_RIGHT_SHOULDER, false) => Self::VkGamepadRightShoulder,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON, false) => Self::VkGamepadRightThumbstickButton,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN, false) => Self::VkGamepadRightThumbstickDown,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT, false) => Self::VkGamepadRightThumbstickLeft,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT, false) => Self::VkGamepadRightThumbstickRight,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_UP, false) => Self::VkGamepadRightThumbstickUp,
            (VK_GAMEPAD_RIGHT_TRIGGER, false) => Self::VkGamepadRightTrigger,
            (VK_GAMEPAD_VIEW, false) => Self::VkGamepadView,
            (VK_GAMEPAD_X, false) => Self::VkGamepadX,
            (VK_GAMEPAD_Y, false) => Self::VkGamepadY,
            (VK_H, false) => Self::VkH,
            (VK_HANGEUL, false) => Self::VkHangeul,
            // (VK_HANGUL, false) => Self::VkHangul,
            (VK_HANJA, false) => Self::VkHanja,
            (VK_HELP, false) => Self::VkHelp,
            (VK_HOME, true) => Self::VkHome,
            (VK_I, false) => Self::VkI,
            (VK_ICO_00, false) => Self::VkIco00,
            (VK_ICO_CLEAR, false) => Self::VkIcoClear,
            (VK_ICO_HELP, false) => Self::VkIcoHelp,
            (VK_IME_OFF, false) => Self::VkImeOff,
            (VK_IME_ON, false) => Self::VkImeOn,
            (VK_J, false) => Self::VkJ,
            (VK_JUNJA, false) => Self::VkJunja,
            (VK_K, false) => Self::VkK,
            // (VK_KANA, false) => Self::VkKana,
            // (VK_KANJI, false) => Self::VkKanji,
            (VK_L, false) => Self::VkL,
            (VK_LAUNCH_APP1, false) => Self::VkLaunchApp1,
            (VK_LAUNCH_APP2, false) => Self::VkLaunchApp2,
            (VK_LAUNCH_MAIL, false) => Self::VkLaunchMail,
            (VK_LAUNCH_MEDIA_SELECT, false) => Self::VkLaunchMediaSelect,
            (VK_LBUTTON, false) => Self::VkLbutton,
            (VK_LEFT, true) => Self::VkLeft,
            (VK_M, false) => Self::VkM,
            (VK_MBUTTON, false) => Self::VkMbutton,
            (VK_MEDIA_NEXT_TRACK, false) => Self::VkMediaNextTrack,
            (VK_MEDIA_PLAY_PAUSE, false) => Self::VkMediaPlayPause,
            (VK_MEDIA_PREV_TRACK, false) => Self::VkMediaPrevTrack,
            (VK_MEDIA_STOP, false) => Self::VkMediaStop,
            (VK_MENU, false) => Self::VkMenu,
            (VK_MODECHANGE, false) => Self::VkModechange,
            (VK_MULTIPLY, true) => Self::VkMultiply,
            (VK_N, false) => Self::VkN,
            (VK_NAVIGATION_ACCEPT, false) => Self::VkNavigationAccept,
            (VK_NAVIGATION_CANCEL, false) => Self::VkNavigationCancel,
            (VK_NAVIGATION_DOWN, false) => Self::VkNavigationDown,
            (VK_NAVIGATION_LEFT, false) => Self::VkNavigationLeft,
            (VK_NAVIGATION_MENU, false) => Self::VkNavigationMenu,
            (VK_NAVIGATION_RIGHT, false) => Self::VkNavigationRight,
            (VK_NAVIGATION_UP, false) => Self::VkNavigationUp,
            (VK_NAVIGATION_VIEW, false) => Self::VkNavigationView,
            (VK_NEXT, true) => Self::VkNext,
            // (VK_NONAME, false) => Self::VkNoname,
            (VK_NONCONVERT, false) => Self::VkNonconvert,
            (VK_NUMLOCK, true) => Self::VkNumlock,
            (VK_NUMPAD0, false) => Self::VkNumpad0,
            (VK_NUMPAD1, false) => Self::VkNumpad1,
            (VK_NUMPAD2, false) => Self::VkNumpad2,
            (VK_NUMPAD3, false) => Self::VkNumpad3,
            (VK_NUMPAD4, false) => Self::VkNumpad4,
            (VK_NUMPAD5, false) => Self::VkNumpad5,
            (VK_NUMPAD6, false) => Self::VkNumpad6,
            (VK_NUMPAD7, false) => Self::VkNumpad7,
            (VK_NUMPAD8, false) => Self::VkNumpad8,
            (VK_NUMPAD9, false) => Self::VkNumpad9,
            (VK_O, false) => Self::VkO,
            (VK_OEM_1, false) => Self::VkOem1,
            (VK_OEM_102, false) => Self::VkOem102,
            (VK_OEM_2, false) => Self::VkOem2,
            (VK_OEM_3, false) => Self::VkOem3,
            (VK_OEM_4, false) => Self::VkOem4,
            (VK_OEM_5, false) => Self::VkOem5,
            (VK_OEM_6, false) => Self::VkOem6,
            (VK_OEM_7, false) => Self::VkOem7,
            (VK_OEM_8, false) => Self::VkOem8,
            // (VK_OEM_ATTN, false) => Self::VkOemAttn,
            // (VK_OEM_AUTO, false) => Self::VkOemAuto,
            (VK_OEM_AX, false) => Self::VkOemAx,
            // (VK_OEM_BACKTAB, false) => Self::VkOemBacktab,
            (VK_OEM_CLEAR, false) => Self::VkOemClear,
            (VK_OEM_COMMA, false) => Self::VkOemComma,
            // (VK_OEM_COPY, false) => Self::VkOemCopy,
            (VK_OEM_CUSEL, false) => Self::VkOemCusel,
            // (VK_OEM_ENLW, false) => Self::VkOemEnlw,
            // (VK_OEM_FINISH, false) => Self::VkOemFinish,
            (VK_OEM_FJ_JISHO, false) => Self::VkOemFjJisho,
            (VK_OEM_FJ_LOYA, false) => Self::VkOemFjLoya,
            (VK_OEM_FJ_MASSHOU, false) => Self::VkOemFjMasshou,
            (VK_OEM_FJ_ROYA, false) => Self::VkOemFjRoya,
            (VK_OEM_FJ_TOUROKU, false) => Self::VkOemFjTouroku,
            (VK_OEM_JUMP, false) => Self::VkOemJump,
            (VK_OEM_MINUS, false) => Self::VkOemMinus,
            // (VK_OEM_NEC_EQUAL, false) => Self::VkOemNecEqual,
            (VK_OEM_PA1, false) => Self::VkOemPa1,
            (VK_OEM_PA2, false) => Self::VkOemPa2,
            (VK_OEM_PA3, false) => Self::VkOemPa3,
            (VK_OEM_PERIOD, false) => Self::VkOemPeriod,
            (VK_OEM_PLUS, false) => Self::VkOemPlus,
            (VK_OEM_RESET, false) => Self::VkOemReset,
            (VK_OEM_WSCTRL, false) => Self::VkOemWsctrl,
            (VK_P, false) => Self::VkP,
            // (VK_PA1, false) => Self::VkPa1,
            (VK_PACKET, false) => Self::VkPacket,
            (VK_PAUSE, false) => Self::VkPause,
            // (VK_PLAY, false) => Self::VkPlay,
            (VK_PRINT, false) => Self::VkPrint,
            (VK_PRIOR, true) => Self::VkPrior,
            (VK_PROCESSKEY, false) => Self::VkProcesskey,
            (VK_Q, false) => Self::VkQ,
            (VK_R, false) => Self::VkR,
            (VK_RBUTTON, false) => Self::VkRbutton,

            (VK_RETURN, false) => Self::VkReturn,
            (VK_RIGHT, true) => Self::VkRight,

            (VK_S, false) => Self::VkS,
            (VK_SCROLL, false) => Self::VkScroll,
            (VK_SELECT, false) => Self::VkSelect,
            (VK_SEPARATOR, false) => Self::VkSeparator,
            (VK_SHIFT, false) => Self::VkShift,
            (VK_SLEEP, false) => Self::VkSleep,
            (VK_SNAPSHOT, true) => Self::VkSnapshot,
            (VK_SPACE, false) => Self::VkSpace,
            (VK_SUBTRACT, false) => Self::VkSubtract,
            (VK_T, false) => Self::VkT,
            (VK_TAB, false) => Self::VkTab,
            (VK_U, false) => Self::VkU,
            (VK_UP, true) => Self::VkUp,
            (VK_V, false) => Self::VkV,
            (VK_VOLUME_DOWN, false) => Self::VkVolumeDown,
            (VK_VOLUME_MUTE, false) => Self::VkVolumeMute,
            (VK_VOLUME_UP, false) => Self::VkVolumeUp,
            (VK_W, false) => Self::VkW,
            (VK_X, false) => Self::VkX,
            (VK_XBUTTON1, false) => Self::VkXbutton1,
            (VK_XBUTTON2, false) => Self::VkXbutton2,
            (VK_Y, false) => Self::VkY,
            (VK_Z, false) => Self::VkZ,
            // (VK_ZOOM, false) => Self::VkZoom,
            _ => Self::VkNone,
        }
    }
}

/// 键盘枚举和字符的相互转换
macro_rules! impl_keys_str_into {
    ($($k:tt => $v:path), *) => {
        impl From<&str> for Keys {
            fn from(value: &str) -> Self {
                match value {
                    $($k => $v,) *
                    _ => unreachable!()
                }
            }
        }

        impl From<Keys> for &str {
            fn from(value: Keys) -> Self {
                match value {
                    $($v => $k,) *
                    _ => unreachable!()
                }
            }
        }
    };
}

impl_keys_str_into!(
    "RigelA" => Keys::VkRigelA,

    "大写锁定" => Keys::VkCapital,
    "小键盘0" => Keys::VkNumPad0,

    "小键盘1" => Keys::VkNumPad1,
    "小键盘2" => Keys::VkNumPad2,
    "小键盘3" => Keys::VkNumPad3,
    "小键盘4" => Keys::VkNumPad4,
    "小键盘5" => Keys::VkNumPad5,
    "小键盘6" => Keys::VkNumPad6,
    "小键盘7" => Keys::VkNumPad7,
    "小键盘8" => Keys::VkNumPad8,
    "小键盘9" => Keys::VkNumPad9,
    "小键盘点" => Keys::VkNumPadDot,
    "小回车" => Keys::VkNumPadReturn,

    "小键盘/" => Keys::VkNumPadDiv,
    "小键盘*" => Keys::VkNumPadMul,
    "小键盘+" => Keys::VkAdd,
    "小键盘-" => Keys::VkSubtract,

    "Ctrl" => Keys::VkCtrl,
    "Shift" => Keys::VkShift,
    "Alt"=> Keys::VkAlt,
    "Win" =>  Keys::VkWin,

    "空格"=> Keys::VkSpace,
    "Esc" => Keys::VkEscape,
    "Tab" => Keys::VkTab,
    "回车" => Keys::VkReturn,
    "菜单" => Keys::VkApps,
    "退格" => Keys::VkBack,

    "左光标" => Keys::VkLeft,
    "右光标" => Keys::VkRight,
    "上光标" => Keys::VkUp,
    "下光标" => Keys::VkDown,
    "删除" => Keys::VkDelete,
    "插入"=> Keys::VkInsert,
    "Home"=> Keys::VkHome,
    "End"=> Keys::VkEnd,
    "PageUp"=> Keys::VkPrior,
    "PageDown"=> Keys::VkNext,

    "~" => Keys::VkOem3,
    "-" => Keys::VkOemMinus,
    "=" => Keys::VkOemPlus,
    "[" => Keys::VkOem4,
    "]" => Keys::VkOem6,
    "\\" => Keys::VkOem5,
    ";" => Keys::VkOem1,
    "'" => Keys::VkOem7,
    "," => Keys::VkOemComma,
    "." => Keys::VkOemPeriod,
    "/" => Keys::VkOem2,

    "截图" => Keys::VkSnapshot,
    "Scroll"=> Keys::VkScroll,
    "Pause"=> Keys::VkPause,
    "NumLock"=> Keys::VkNumlock,

    "F1" => Keys::VkF1,
    "F2" => Keys::VkF2,
    "F3" => Keys::VkF3,
    "F4" => Keys::VkF4,
    "F5" => Keys::VkF5,
    "F6" => Keys::VkF6,
    "F7" => Keys::VkF7,
    "F8" => Keys::VkF8,
    "F9" => Keys::VkF9,
    "F10" => Keys::VkF10,
    "F11" => Keys::VkF11,
    "F12" => Keys::VkF12,
    "0" => Keys::Vk0,
    "1" => Keys::Vk1,
    "2" => Keys::Vk2,
    "3" => Keys::Vk3,
    "4" => Keys::Vk4,
    "5" => Keys::Vk5,
    "6" => Keys::Vk6,
    "7" => Keys::Vk7,
    "8" => Keys::Vk8,
    "9" => Keys::Vk9,
    "A" => Keys::VkA,
    "B" => Keys::VkB,
    "C" => Keys::VkC,
    "D" => Keys::VkD,
    "E" => Keys::VkE,
    "F" => Keys::VkF,
    "G" => Keys::VkG,
    "H" => Keys::VkH,
    "I" => Keys::VkI,
    "J" => Keys::VkJ,
    "K" => Keys::VkK,
    "L" => Keys::VkL,
    "M" => Keys::VkM,
    "N" => Keys::VkN,
    "O" => Keys::VkO,
    "P" => Keys::VkP,
    "Q" => Keys::VkQ,
    "R" => Keys::VkR,
    "S" => Keys::VkS,
    "T" => Keys::VkT,
    "U" => Keys::VkU,
    "V" => Keys::VkV,
    "W" => Keys::VkW,
    "X" => Keys::VkX,
    "Y" => Keys::VkY,
    "Z" => Keys::VkZ,

    "Num0" => Keys::VkNumpad0,
    "Num1" => Keys::VkNumpad1,
    "Num2" => Keys::VkNumpad2,
    "Num3" => Keys::VkNumpad3,
    "Num4" => Keys::VkNumpad4,
    "Num5" => Keys::VkNumpad5,
    "Num6" => Keys::VkNumpad6,
    "Num7" => Keys::VkNumpad7,
    "Num8" => Keys::VkNumpad8,
    "Num9" => Keys::VkNumpad9,
    "Num." => Keys::VkDecimal,

    "" => Keys::VkNone
);

impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text: &str = self.clone().into();
        writeln!(f, "{text}")
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self::VkNone
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_vk_from_num() {
        let key: Keys = (27u32, false).into();
        assert_eq!(key, Keys::VkEscape);
    }

    #[test]
    fn test_vk_from_str() {
        let key: Keys = "A".into();
        assert_eq!(key, Keys::VkA);
    }

    #[test]
    fn test_vk_to_str() {
        let key_str: &str = Keys::VkA.into();
        assert_eq!(key_str, "A");
    }
}
