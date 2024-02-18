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

pub(crate) mod hotkeys;
pub(crate) mod popup_menu;
pub(crate) mod system_tray;
pub(crate) mod utils;
pub(crate) mod welcome;

#[macro_export]
macro_rules! bring_window_front {
    ($window:expr) => {
        if let nwg::ControlHandle::Hwnd(h) = $window.handle {
            let current_thread_id = win_wrap::threading::get_current_thread_id();
            let h_foreground = win_wrap::common::get_foreground_window();
            let (remote_thread_id, _) =
                win_wrap::threading::get_window_thread_process_id(h_foreground);

            win_wrap::common::attach_thread_input(
                current_thread_id,
                remote_thread_id,
                win_wrap::common::TRUE,
            );

            win_wrap::common::show_window(
                win_wrap::common::HWND(h as isize),
                win_wrap::common::SW_HIDE,
            );
            win_wrap::common::show_window(
                win_wrap::common::HWND(h as isize),
                win_wrap::common::SW_SHOW,
            );
            win_wrap::common::set_foreground_window(win_wrap::common::HWND(h as isize));
            win_wrap::common::set_active_window(win_wrap::common::HWND(h as isize));

            win_wrap::common::attach_thread_input(
                current_thread_id,
                remote_thread_id,
                win_wrap::common::FALSE,
            );
        }
    };
}
