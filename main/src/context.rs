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

use crate::configs::config_manager::ConfigManager;
use crate::{
    browser::form_browser::FormBrowser, commander::Commander, event_core, performer::Performer,
    proxy32::Proxy32, resources::ResourceAccessor, talent::TalentAccessor, terminator::Terminator,
};
use peeper::server::PeeperServer;
use rigela_utils::get_program_directory;
use std::sync::Arc;
use tokio::runtime::{Builder, Handle, Runtime};
use win_wrap::uia::automation::UiAutomation;

const CONFIG_FILE_NAME: &str = "config.toml";

/// 核心上下文对象，通过此对象可以访问整个程序API
#[derive(Debug)]
pub struct Context {
    pub(crate) commander: Arc<Commander>,
    pub(crate) config_manager: Arc<ConfigManager>,
    pub(crate) main_handler: Arc<Handle>,
    pub(crate) work_runtime: Arc<Runtime>,
    pub(crate) resource_accessor: Arc<ResourceAccessor>,
    pub(crate) peeper_server: Arc<PeeperServer>,
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

        // 创建表演者对象（用于把各种信息转换成用户可以感知的形式，例如语音、音效等）
        let performer = Performer::new();

        // 获取一个主线程携程处理器，可以在子线程中调度任务到主线程
        let main_handler = Handle::current();

        // 获取一个工作线程携程运行时，可以把任何耗时的操作任务调度到子线程中
        let work_runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        // 进程亏叹气服务
        let peeper_server = PeeperServer::new();

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
            main_handler: main_handler.into(),
            peeper_server: peeper_server.into(),
            performer: performer.into(),
            proxy32: proxy32.into(),
            resource_accessor: resources.into(),
            talent_accessor: talent_accessor.into(),
            terminator: terminator.into(),
            ui_automation: ui_automation.into(),
            work_runtime: work_runtime.into(),
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
            ctx.config_manager.init().await;
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
            main_handler: self.main_handler.clone(),
            peeper_server: self.peeper_server.clone(),
            performer: self.performer.clone(),
            proxy32: self.proxy32.clone(),
            resource_accessor: self.resource_accessor.clone(),
            talent_accessor: self.talent_accessor.clone(),
            terminator: self.terminator.clone(),
            ui_automation: self.ui_automation.clone(),
            work_runtime: self.work_runtime.clone(),
            event_core: self.event_core.clone(),
            form_browser: self.form_browser.clone(),
        }
    }
}
