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

#![windows_subsystem = "windows"]

mod form;
mod utils;

use nwg::NativeUi;
use rigela_utils::logger::init_logger;
use std::env::args;

#[tokio::main]
async fn main() {
    if args().len() < 2 {
        // 调用更新器需要传递主程序的路径作为参数
        return;
    }

    init_logger(Some(format!("{}.log", module_path!()).as_str()));

    nwg::init().expect("Failed to init Native Windows GUI");

    let app = form::App::build_ui(Default::default()).expect("Failed to build UI");
    app.handler.set(tokio::runtime::Handle::current()).unwrap();

    nwg::dispatch_thread_events();
}
