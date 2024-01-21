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

use crate::{
    browser::form_browser::FormBrowser,
    commander::Commander,
    configs::ConfigManager,
    event_core,
    gui::GuiAccessor,
    helper::proxy32::Proxy32,
    performer::{Performer, Speakable},
    resources::ResourceAccessor,
    talent::TalentAccessor,
    terminator::Terminator,
};
use rigela_utils::get_program_directory;
use std::sync::Arc;
use tokio::runtime::Handle;
use win_wrap::uia::{
    automation::UiAutomation, element::UiAutomationElement,
    pattern::UiAutomationLegacyIAccessiblePattern,
};

const CONFIG_FILE_NAME: &str = "config.toml";

/// 核心上下文对象，通过此对象可以访问整个程序API
pub struct Context {
    pub(crate) commander: Arc<Commander>,
    pub(crate) config_manager: Arc<ConfigManager>,
    pub(crate) gui_accessor: Arc<GuiAccessor>,
    pub(crate) main_handler: Arc<Handle>,
    pub(crate) resource_accessor: Arc<ResourceAccessor>,
    pub(crate) performer: Arc<Performer>,
    pub(crate) proxy32: Arc<Proxy32>,
    pub(crate) talent_accessor: Arc<TalentAccessor>,
    pub(crate) terminator: Arc<Terminator>,
    pub(crate) ui_automation: Arc<UiAutomation>,
    pub(crate) event_core: Arc<event_core::EventCore>,
    pub(crate) form_browser: Arc<FormBrowser>,
}

impl Context {
    /**
     * 创建一个框架上下文环境。
     * */
    pub(crate) fn new(terminator: Terminator) -> Self {
        // 创建一个指挥官，用于下发操作命令
        let commander = Commander::new();

        // 配置管理器
        let path = get_program_directory().join(CONFIG_FILE_NAME);
        let config_manager = ConfigManager::new(path);

        // 创建GUI访问器
        let gui_accessor = GuiAccessor::new();

        // 创建表演者对象（用于把各种信息转换成用户可以感知的形式，例如语音、音效等）
        let performer = Performer::new();

        // 获取一个主线程携程处理器，可以在子线程中调度任务到主线程
        let main_handler = Handle::current();

        // 用于兼容32位进程访问
        let proxy32 = Proxy32::new();

        // 创建资源访问器
        let resources = ResourceAccessor::new();

        // 创建能力访问器
        let talent_accessor = TalentAccessor::new();

        // 创建UiAutomation
        let ui_automation = UiAutomation::new();

        // 事件处理中心
        let event_core = event_core::EventCore::new();

        // 窗口浏览器
        let form_browser = FormBrowser::new();

        Self {
            commander: commander.into(),
            config_manager: config_manager.into(),
            gui_accessor: gui_accessor.into(),
            main_handler: main_handler.into(),
            performer: performer.into(),
            proxy32: proxy32.into(),
            resource_accessor: resources.into(),
            talent_accessor: talent_accessor.into(),
            terminator: terminator.into(),
            ui_automation: ui_automation.into(),
            event_core: event_core.into(),
            form_browser: Arc::new(form_browser),
        }
    }

    /**
     * 把上下文对象应用于每一个组件。
     * */
    pub(crate) fn apply(&self) {
        self.commander.apply(self.clone().into());

        let ctx = Arc::new(self.clone());

        self.main_handler.spawn(async move {
            ctx.performer.apply(ctx.clone()).await;
        });
    }

    /**
     * 清理环境。
     * */
    pub(crate) fn dispose(&self) {
        self.commander.dispose();
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            commander: self.commander.clone(),
            config_manager: self.config_manager.clone(),
            gui_accessor: self.gui_accessor.clone(),
            main_handler: self.main_handler.clone(),
            performer: self.performer.clone(),
            proxy32: self.proxy32.clone(),
            resource_accessor: self.resource_accessor.clone(),
            talent_accessor: self.talent_accessor.clone(),
            terminator: self.terminator.clone(),
            ui_automation: self.ui_automation.clone(),
            event_core: self.event_core.clone(),
            form_browser: self.form_browser.clone(),
        }
    }
}

/// 给UIA元素实现朗读接口
impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        let mut name = self.get_name();

        if name.is_empty() {
            let accessible: UiAutomationLegacyIAccessiblePattern = self.into();
            name = accessible.get_name();

            if name.is_empty() {
                name = accessible.get_description();
            }
        }

        format!("{}: {}", name, self.get_localized_control_type())
    }
}
