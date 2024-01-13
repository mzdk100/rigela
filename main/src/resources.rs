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

use home::home_dir;
use rigela_resources::clone;
use std::fs::create_dir;
use std::path::PathBuf;
use tokio::fs::File;

//noinspection HttpUrlsUsage
const SERVER_HOME_URI: &str = "http://api.zhumang.vip:8080/rigela";
const DIR_NAME: &str = ".rigela";

pub struct ResourceAccessor {
    root_dir: PathBuf,
}

impl ResourceAccessor {
    /**
     * 创建一个资源读取器。
     * */
    pub(crate) fn new() -> Self {
        let root_dir = home_dir()
            .expect("Can't get the current user directory.")
            .join(DIR_NAME);
        if !root_dir.exists() {
            create_dir(&root_dir).expect("Can't create the root directory.");
        }

        Self { root_dir }
    }

    /**
     * 打开一个资源文件。
     * `resource_name` 资源名称。
     * */
    pub(crate) async fn open(&self, resource_name: &str) -> Result<File, String> {
        let url = format!("{}/{}", SERVER_HOME_URI, resource_name);
        clone(url, self.root_dir.join(resource_name)).await
    }
}
