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
    configs::{
        general::Lang,
        config_manager::ConfigManager,
    },
    browser::form_browser::FormBrowser,
    commander::Commander,
    event_core,
    gui::GuiProvider,
    performer::Performer,
    resources::ResourceProvider,
    talent::TalentProvider,
    tasks::TaskManager,
    terminator::Terminator,
};
#[cfg(target_arch = "x86_64")]
use rigela_proxy32::process::Proxy32Process;
use a11y::{
    ia2::Ia2,
    jab::Jab,
};
use log::info;
use peeper::server::PeeperServer;
use rigela_utils::fs::get_program_directory;
use std::sync::Arc;
use tokio::runtime::{Builder, Handle, Runtime};
use win_wrap::{msaa::Msaa, uia::automation::UiAutomation};

const CONFIG_FILE_NAME: &str = "config.toml";

/// 核心上下文对象，通过此对象可以访问整个读屏框架的API
#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) commander: Arc<Commander>,
    pub(crate) config_manager: Arc<ConfigManager>,
    pub(crate) event_core: Arc<event_core::EventCore>,
    pub(crate) form_browser: Arc<FormBrowser>,
    pub(crate) gui_provider: Arc<GuiProvider>,
    pub(crate) jab: Arc<Jab>,
    pub(crate) main_handler: Arc<Handle>,
    pub(crate) msaa: Arc<Msaa>,
    pub(crate) ia2: Arc<Ia2>,
    pub(crate) work_runtime: Arc<Runtime>,
    pub(crate) resource_provider: Arc<ResourceProvider>,
    pub(crate) peeper_server: Arc<PeeperServer>,
    pub(crate) performer: Arc<Performer>,
    #[cfg(target_arch = "x86_64")]
    pub(crate) proxy32process: Arc<Proxy32Process>,
    pub(crate) talent_provider: Arc<TalentProvider>,
    pub(crate) task_manager: Arc<TaskManager>,
    pub(crate) terminator: Arc<Terminator>,
    pub(crate) ui_automation: Arc<UiAutomation>,
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

        // MSAA(Microsoft Active Accessibility，辅助功能）接口
        let msaa = Msaa::new();

        // IA2（扩展了MSAA)
        let ia2 = Ia2::new();

        // JAB (Java Access Bridge，无障碍访问桥)
        let jab = Jab::new();

        // 获取一个工作线程携程运行时，可以把任何耗时的操作任务调度到子线程中
        let work_runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        // 进程亏叹气服务
        let peeper_server = PeeperServer::new();

        // 用于兼容32位进程访问
        #[cfg(target_arch = "x86_64")]
            let proxy32process = Proxy32Process::new();

        // 创建资源提供者
        let resource_provider = ResourceProvider::new();

        // 创建能力提供者
        let talent_provider = TalentProvider::new();

        // 创建任务管理器
        let task_manager = TaskManager::new();

        // 创建UiAutomation
        let ui_automation = UiAutomation::new();

        // 事件处理中心
        let event_core = event_core::EventCore::new();

        // 窗口浏览器
        let form_browser = FormBrowser::new();

        // Gui 窗口界面管理器
        let window_manager = GuiProvider::new();

        Self {
            commander: commander.into(),
            config_manager: config_manager.into(),
            main_handler: main_handler.into(),
            msaa: msaa.into(),
            ia2: ia2.into(),
            jab: jab.into(),
            peeper_server: peeper_server.into(),
            performer: performer.into(),
            #[cfg(target_arch = "x86_64")]
            proxy32process: proxy32process.into(),
            resource_provider: resource_provider.into(),
            talent_provider: talent_provider.into(),
            task_manager: task_manager.into(),
            terminator: terminator.into(),
            ui_automation: ui_automation.into(),
            work_runtime: work_runtime.into(),
            event_core: event_core.into(),
            form_browser: Arc::new(form_browser),
            gui_provider: window_manager.into(),
        }
    }

    /**
     * 把上下文对象应用于每一个组件。
     * */
    pub(crate) fn apply(&self) {
        let ctx = Arc::new(self.clone());

        // 初始化命令管理器
        self.commander.apply(ctx.clone());

        // 加载配置文件
        self.config_manager.init();
        // 设置语言
        let lang = match self.config_manager.get_config().general_config.lang.clone() {
            Lang::Zh => "zh-CN",
            Lang::En => "en",
        };
        rust_i18n::set_locale(lang);

        // 启动表演者
        let performer = self.performer.clone();
        self.work_runtime.spawn(async move {
            performer.apply(ctx).await;
            info!("The performer is ready.");
        });
    }

    /**
     * 清理环境。
     * */
    pub(crate) fn dispose(&self) {
        self.commander.dispose();
        self.event_core.shutdown();
        self.jab.remove_all_listeners();
        self.msaa.remove_all_listeners();
        self.ia2.remove_all_listeners();
        self.ui_automation.remove_all_event_listeners();
        self.config_manager.save_config();
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            commander: self.commander.clone(),
            config_manager: self.config_manager.clone(),
            main_handler: self.main_handler.clone(),
            msaa: self.msaa.clone(),
            ia2: self.ia2.clone(),
            jab: self.jab.clone(),
            peeper_server: self.peeper_server.clone(),
            performer: self.performer.clone(),
            #[cfg(target_arch = "x86_64")]
            proxy32process: self.proxy32process.clone(),
            resource_provider: self.resource_provider.clone(),
            talent_provider: self.talent_provider.clone(),
            task_manager: self.task_manager.clone(),
            terminator: self.terminator.clone(),
            ui_automation: self.ui_automation.clone(),
            work_runtime: self.work_runtime.clone(),
            event_core: self.event_core.clone(),
            form_browser: self.form_browser.clone(),
            gui_provider: self.gui_provider.clone(),
        }
    }
}
