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

use crate::{configs::mouse::MouseConfig, configs::tts::TtsConfig};
use log::{debug, error, info};
use rigela_utils::{read_file, write_file};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use tokio::{sync::RwLock, time::sleep};

/// 配置项目的根元素
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub(crate) struct ConfigRoot {
    pub(crate) tts_config: TtsConfig,
    pub(crate) mouse_config: MouseConfig,
}

/// 配置管理器
#[derive(Debug)]
pub(crate) struct ConfigManager {
    // 配置文件的路径
    path: PathBuf,
    // 当前的配置
    config: RwLock<ConfigRoot>,

    update_time: RwLock<Instant>,
    write_finished: RwLock<bool>,
}

impl ConfigManager {
    /**
     * 创建一个配置管理器
     * */
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            config: RwLock::new(ConfigRoot::default()),
            update_time: RwLock::new(Instant::now()),
            write_finished: RwLock::new(true),
        }
    }

    /// 初始化当前配置，从配置文件获取配置信息
    pub(crate) async fn init(&self) {
        let mut config = self.config.write().await;
        *config = self.read().await;
    }

    /// 获取当前的配置
    pub(crate) async fn get_config(&self) -> ConfigRoot {
        (*self.config.read().await).clone()
    }

    /// 修改当前的配置，修改完写入配置文件
    pub(crate) async fn set_config(&self, config: ConfigRoot) {
        let mut cur_config = self.config.write().await;
        *cur_config = config.clone();

        debug!("config changed");

        *self.update_time.write().await = Instant::now();
        self.write().await;
    }

    /*
     * 读取配置数据。如果不存在配置文件，写入默认配置
     * */
    pub(crate) async fn read(&self) -> ConfigRoot {
        let config = match read_file(&self.path.clone()).await {
            Ok(mut content) => match toml::from_str::<ConfigRoot>(content.as_mut_str()) {
                Ok(c) => Some(c),
                Err(_) => {
                    info!("配置文件格式错误，将使用默认配置");
                    None
                }
            },
            _ => {
                info!("配置文件不存在，将使用默认配置");
                None
            }
        };

        if let Some(cfg) = config {
            cfg
        } else {
            let cfg: ConfigRoot = Default::default();
            *self.config.write().await = cfg.clone();
            self.write().await;

            cfg
        }
    }

    /**
     * 写出配置数据。
     * `config_root` 完整的配置数据。
     * */
    pub(crate) async fn write(&self) {
        if !*self.write_finished.read().await {
            return;
        }
        {
            *self.write_finished.write().await = false;
        }

        loop {
            debug!("prepare writing config");

            if Instant::now() >= *self.update_time.read().await + Duration::from_secs(10) {
                break;
            }
            sleep(Duration::from_secs(10)).await;
        }

        debug!("start writing config");

        let path = self.path.clone();
        let config_root = &self.get_config().await;

        match toml::to_string(config_root) {
            Ok(content) => {
                if let Err(e) = write_file(&path, content.as_bytes()).await {
                    error!("{}", e);
                }
            }
            Err(e) => error!("{}", e),
        }

        *self.write_finished.write().await = true;
    }
}
