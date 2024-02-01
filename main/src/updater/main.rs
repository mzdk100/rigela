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

use select::{document::Document, predicate::Class};

pub mod form;

const LOG_URL: &str = "https://gitcode.net/mzdk100/rigela/-/commits/master/";
//noinspection HttpUrlsUsage
#[allow(unused)]
const DOWNLOAD_URL: &str = "http://api.zhumang.vip:8080/rigela/rigela_x64/main.exe";

#[tokio::main]
async fn main() {
    form::show(get_log().await.as_mut_str());
}

async fn get_log() -> String {
    match reqwest::get(LOG_URL).await {
        Ok(response) => {
            let html = response.text().await.unwrap();
            let document = Document::from(html.as_str());
            let mut text = String::new();
            for i in document.find(Class("commit-detail")) {
                text += (i.text().trim().to_owned() + "\r\n").as_str();
            }
            text
        }
        Err(_) => "网络异常!".to_string(),
    }
}
