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

use crate::commander::Commander;
use crate::configs::ConfigManager;
use crate::performer::{Performer, Speakable};
use crate::resources::ResourceAccessor;
use crate::terminator::Terminator;
use crate::utils::get_program_directory;
use std::sync::Arc;
use tokio::runtime::Handle;
use win_wrap::uia::{UiAutomation, UiAutomationElement};

impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_localized_control_type())
    }
}

pub struct Context {
    pub(crate) commander: Arc<Commander>,
    pub(crate) config_manager: Arc<ConfigManager>,
    pub(crate) main_handler: Arc<Handle>,
    pub(crate) resource_accessor: Arc<ResourceAccessor>,
    pub(crate) performer: Arc<Performer>,
    pub(crate) terminator: Arc<Terminator>,
    pub(crate) ui_automation: Arc<UiAutomation>,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            commander: self.commander.clone(),
            config_manager: self.config_manager.clone(),
            main_handler: self.main_handler.clone(),
            performer: self.performer.clone(),
            resource_accessor: self.resource_accessor.clone(),
            terminator: self.terminator.clone(),
            ui_automation: self.ui_automation.clone(),
        }
    }
}

impl Context {
    /**
     * 创建一个框架上下文环境。
     * */
    pub(crate) fn new(terminator: Terminator) -> Self {
        // 创建一个指挥官，用于下发操作命令
        let commander = Commander::new();
        let config_manager = ConfigManager::new(get_program_directory().join("config.toml"));
        // 创建表演者对象（用于把各种信息转换成用户可以感知的形式，例如语音、音效等）
        let performer = Performer::new();
        // 获取一个主线程携程处理器，可以在子线程中调度任务到主线程
        let main_handler = Handle::current();
        // 资源访问器
        let resources = ResourceAccessor::new();
        // 创建UiAutomation
        let ui_automation = UiAutomation::new();
        Self {
            commander: commander.into(),
            config_manager: config_manager.into(),
            main_handler: main_handler.into(),
            performer: performer.into(),
            resource_accessor: resources.into(),
            terminator: terminator.into(),
            ui_automation: ui_automation.into(),
        }
    }

    /**
     * 把上下文对象应用于每一个组件。
     * */
    pub(crate) fn apply(&self) {
        self.commander.apply(self.clone().into());
        self.performer.apply_config(self.clone().into(), |_| {});
    }

    /**
     * 清理环境。
     * */
    pub(crate) fn dispose(&self) {
        self.commander.dispose();
    }
}
