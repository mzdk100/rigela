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
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, OnceLock};
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
}

/// 配置管理器
#[derive(Debug)]
pub(crate) struct ConfigManager {
    // 配置文件的路径
    pub(crate) path: PathBuf,
    // 当前的配置
    config: Mutex<ConfigRoot>,
}

impl ConfigManager {
    /**
     * 创建一个配置管理器
     * */
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            config: Mutex::new(ConfigRoot::default()),
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
    pub(crate) fn set_config(&self, config: ConfigRoot) {
        *self.config.lock().unwrap().deref_mut() = config.clone();

        *update_time().lock().unwrap().deref_mut() = Instant::now();
        *current_config().lock().unwrap().deref_mut() = config.clone();

        let path = self.path.clone();
        spawn(move || write_config(path));
    }

    /*
     * 读取配置数据。如果不存在配置文件，写入默认配置
     * */
    pub(crate) fn read(&self) -> ConfigRoot {
        let config = match read_to_string(&self.path.clone()) {
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
            *self.config.lock().unwrap().deref_mut() = cfg.clone();

            let path = self.path.clone();
            spawn(move || write_config(path));

            cfg
        }
    }
}

// 当前的配置值
fn current_config() -> &'static Mutex<ConfigRoot> {
    static INSTANCE: OnceLock<Mutex<ConfigRoot>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(Default::default()))
}

// 是否完成写入，防止重复调用写入
fn write_finished() -> &'static Mutex<bool> {
    static INSTANCE: OnceLock<Mutex<bool>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(true))
}

// 更新时间
fn update_time() -> &'static Mutex<Instant> {
    static INSTANCE: OnceLock<Mutex<Instant>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(Instant::now()))
}

// 写出配置数据。
fn write_config(path: PathBuf) {
    if !*write_finished().lock().unwrap() {
        return;
    }
    {
        *write_finished().lock().unwrap().deref_mut() = false;
    }

    loop {
        if Instant::now() >= *update_time().lock().unwrap() + Duration::from_secs(10) {
            break;
        }
        sleep(Duration::from_secs(10));
    }

    match toml::to_string(current_config().lock().unwrap().deref()) {
        Ok(content) => {
            if let Ok(mut file) = File::create(&path) {
                file.write(&content.into_bytes())
                    .expect("write config file failed");
            } else {
                error!("create config file failed");
            }
        }
        Err(e) => error!("{}", e),
    }

    *write_finished().lock().unwrap().deref_mut() = true;
}
