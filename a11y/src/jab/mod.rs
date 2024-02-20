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

use rigela_utils::call_proc;
use std::{
    env::var,
    path::{Path, PathBuf},
};
use win_wrap::{
    common::{free_library, get_proc_address, load_library, Result, BOOL, FARPROC, HMODULE, HWND},
    message::pump_waiting_messages,
};
use windows::{
    core::{Error, HSTRING},
    Win32::Foundation::S_FALSE,
};

#[macro_export]
macro_rules! jab {
    ($module:expr,windows_run) => {
        call_proc!($module, Windows_run, extern "cdecl" fn() -> BOOL,)
    };
    ($module:expr,is_java_window,$h_wnd:expr) => {
        call_proc!(
            $module,
            isJavaWindow,
            extern "cdecl" fn(HWND) -> BOOL,
            $h_wnd
        )
    };
}

#[allow(unused)]
#[derive(Debug)]
pub struct JabLib {
    h_module: HMODULE,
}

impl JabLib {
    //noinspection SpellCheckingInspection
    #[allow(unused)]
    pub(crate) fn new(path: Option<PathBuf>) -> Result<Self> {
        #[cfg(target_arch = "x86_64")]
        const DLL_NAME: &str = "windowsaccessbridge-64.dll";
        #[cfg(target_arch = "x86")]
        const DLL_NAME: &str = "windowsaccessbridge-32.dll";
        let lib = match path {
            None => match var("JAVA_HOME") {
                Ok(s) => Path::new(&s).join("bin").join(DLL_NAME),
                Err(e) => {
                    return Err(Error::new(
                        S_FALSE,
                        HSTRING::from(format!("Can't find the jab library. ({})", e)),
                    ))
                }
            },
            Some(p) => p.to_path_buf(),
        };
        let h_module = match load_library(lib.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        let res = jab!(h_module, windows_run);
        if res.is_none() {
            return Err(Error::new(
                S_FALSE,
                HSTRING::from("Can't load the jab library."),
            ));
        }
        pump_waiting_messages();
        Ok(Self { h_module })
    }

    #[allow(unused)]
    /**
     * 判断窗口是否是java的窗口。
     * */
    pub fn is_java_window(&self, h_wnd: HWND) -> bool {
        jab!(self.h_module, is_java_window, h_wnd)
            .unwrap_or(BOOL::from(false))
            .as_bool()
    }
}

impl Drop for JabLib {
    fn drop(&mut self) {
        if self.h_module.is_invalid() {
            return;
        }
        free_library(self.h_module);
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
mod test_jab {
    use crate::jab::JabLib;
    use win_wrap::common::{find_window, get_desktop_window};

    #[test]
    fn main() {
        let jab = JabLib::new(None).unwrap();
        assert!(!jab.is_java_window(get_desktop_window()));
        let h_wnd = find_window(Some("SunAwtFrame"), None);
        assert!(jab.is_java_window(h_wnd));
        dbg!(jab);
    }
}
