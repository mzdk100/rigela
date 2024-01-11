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

use std::sync::Arc;
use crate::{
    commander::Commander,
    configs::ConfigManager,
    gui::GuiAccessor,
    performer::{Performer, Speakable},
    resources::ResourceAccessor,
    talent::TalentAccessor,
    terminator::Terminator,
    utils::get_program_directory,
};
use tokio::runtime::Handle;
use win_wrap::uia::{UiAutomation, UiAutomationElement};

impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_localized_control_type())
    }
}

#[derive(Clone)]
pub struct Context {
    pub(crate) commander: Commander,
    pub(crate) config_manager: ConfigManager,
    pub(crate) gui_accessor: GuiAccessor,
    pub(crate) main_handler: Handle,
    pub(crate) resource_accessor: ResourceAccessor,
    pub(crate) performer: Performer,
    pub(crate) talent_accessor: TalentAccessor,
    pub(crate) terminator: Terminator,
    pub(crate) ui_automation: UiAutomation,
}

impl Context {
    /**
     * 创建一个框架上下文环境。
     * */
    pub(crate) fn new(terminator: Terminator) -> Self {
        // 创建一个指挥官，用于下发操作命令
        let commander = Commander::new();
        let config_manager = ConfigManager::new(get_program_directory().join("config.toml"));
        // 创建GUI访问器
        let gui_accessor = GuiAccessor::new();
        // 创建表演者对象（用于把各种信息转换成用户可以感知的形式，例如语音、音效等）
        let performer = Performer::new();
        // 获取一个主线程携程处理器，可以在子线程中调度任务到主线程
        let main_handler = Handle::current();
        // 创建资源访问器
        let resources = ResourceAccessor::new();
        // 创建能力访问器
        let talent_accessor = TalentAccessor::new();
        // 创建UiAutomation
        let ui_automation = UiAutomation::new();
        Self {
            commander,
            config_manager,
            gui_accessor,
            main_handler,
            performer,
            resource_accessor: resources,
            talent_accessor,
            terminator,
            ui_automation,
        }
    }

    /**
     * 把上下文对象应用于每一个组件。
     * */
    pub(crate) fn apply(&self) {
        self.commander.apply(Arc::new(self.clone()));
        self.performer.apply_config(self.clone().into(), |_| {});
    }

    /**
     * 清理环境。
     * */
    pub(crate) fn dispose(&self) {
        self.commander.dispose();
    }
}
