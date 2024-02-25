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

use crate::{
    configs::general::GeneralConfig, configs::hotkeys::HotKeysConfig, configs::mouse::MouseConfig,
    configs::tts::TtsConfig,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

/// 配置项目的根元素
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub(crate) struct ConfigRoot {
    pub(crate) tts_config: TtsConfig,
    pub(crate) mouse_config: MouseConfig,
    pub(crate) hotkeys_config: HotKeysConfig,
    pub(crate) general_config: GeneralConfig,
}

/// 配置管理器
#[derive(Debug)]
pub(crate) struct ConfigManager {
    // 配置文件的路径
    pub(crate) path: PathBuf,
    // 当前的配置
    config: Arc<Mutex<ConfigRoot>>,
    update_time: Arc<Mutex<Instant>>,
    write_finished: Arc<Mutex<bool>>,
}

impl ConfigManager {
    /**
     * 创建一个配置管理器
     * */
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            config: Arc::new(Mutex::new(ConfigRoot::default())),
            update_time: Arc::new(Mutex::new(Instant::now())),
            write_finished: Arc::new(Mutex::new(true)),
        }
    }

    /// 初始化当前配置，从配置文件获取配置信息
    pub(crate) fn init(&self) {
        *self.config.lock().unwrap().deref_mut() = self.read();
    }

    /// 获取当前的配置
    pub(crate) fn get_config(&self) -> ConfigRoot {
        self.config.lock().unwrap().clone()
    }

    /// 修改当前的配置，修改完写入配置文件
    pub(crate) fn set_config(&self, config: &ConfigRoot) {
        *self.config.lock().unwrap().deref_mut() = config.clone();
        *self.update_time.lock().unwrap().deref_mut() = Instant::now();
        self.write();
    }

    /*
     * 读取配置数据。如果不存在配置文件，写入默认配置
     * */
    pub(crate) fn read(&self) -> ConfigRoot {
        let config = match read_to_string(self.path.clone()) {
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
            *self.config.lock().unwrap().deref_mut() = Default::default();
            self.write();
            Default::default()
        }
    }

    // 写出配置数据。
    fn write(&self) {
        let path = self.path.clone();
        let config = self.config.clone();
        let update_time = self.update_time.clone();
        let write_finished = self.write_finished.clone();

        spawn(move || {
            if !*write_finished.lock().unwrap() {
                return;
            }
            {
                *write_finished.lock().unwrap().deref_mut() = false;
            }

            loop {
                if Instant::now() >= *update_time.lock().unwrap() + Duration::from_secs(10) {
                    break;
                }
                sleep(Duration::from_secs(10));
            }

            match toml::to_string(config.lock().unwrap().deref()) {
                Ok(content) => {
                    if let Ok(mut file) = File::create(&path) {
                        file.write_all(&content.into_bytes())
                            .expect("write config file failed");
                    } else {
                        error!("create config file failed");
                    }
                }
                Err(e) => error!("{}", e),
            }

            *write_finished.lock().unwrap().deref_mut() = true;
        });
    }

    /// 直接保存配置，不延时
    pub(crate) fn save_config(&self) {
        let path = self.path.clone();
        let config = self.config.clone();

        let _ = spawn(
            move || match toml::to_string(config.lock().unwrap().deref()) {
                Ok(content) => {
                    if let Ok(mut file) = File::create(&path) {
                        file.write_all(&content.into_bytes())
                            .expect("write config file failed");
                    } else {
                        error!("create config file failed");
                    }
                }
                Err(e) => error!("{}", e),
            },
        )
        .join();
    }
}
