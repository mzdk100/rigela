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

use log::error;
use rigela_resources::clone_resource;
use rigela_utils::{
    fs::{get_file_modified_duration, get_program_directory},
    SERVER_HOME_URI,
};
use std::{io::Error, path::PathBuf};
use tokio::fs::File;

const PATH_NAME: &str = "resources";

/// 资源提供者
#[derive(Debug)]
pub(crate) struct ResourceProvider {
    root_dir: PathBuf,
}

impl ResourceProvider {
    /**
     * 创建一个资源读取器。
     * */
    pub(crate) fn new() -> Self {
        let root_dir = get_program_directory().join(PATH_NAME);
        if !root_dir.exists() {
            if std::fs::create_dir_all(&root_dir).is_err() {
                error!("创建资源目录失败");
            }
        }
        Self { root_dir }
    }

    /**
     * 打开一个资源文件。
     * `resource_name` 资源名称。
     * */
    pub(crate) async fn open(&self, resource_name: &str) -> Result<File, Error> {
        let path = self.root_dir.join(resource_name);

        if get_file_modified_duration(&path).await > 3600 * 6 {
            // 如果文件修改时间超出6个小时才重新克隆文件，加快启动速度
            let url = format!("{}/{}", SERVER_HOME_URI, resource_name);
            return clone_resource(url, &path).await;
        }

        File::open(&path).await
    }

    /**
     * 获取资源对应的文件路径。
     * */
    pub(crate) fn get_path(&self, resource_name: &str) -> String {
        self.root_dir
            .join(resource_name)
            .into_os_string()
            .into_string()
            .unwrap()
    }
}
