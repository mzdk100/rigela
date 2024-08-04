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
    commander::Commander, configs::ConfigManager, event_core::EventCore, gui::GuiProvider,
    navigator::UiNavigator, performer::Performer, resources::ResourceProvider,
    talent::TalentProvider, tasks::TaskManager, terminator::Terminator,
};
use a11y::{ia2::Ia2, jab::Jab};
use log::info;
use peeper::server::PeeperServer;
#[cfg(target_arch = "x86_64")]
use rigela_proxy32::process::Proxy32Process;
use rigela_utils::fs::get_rigela_program_directory;
use std::sync::{Arc, Weak};
use tokio::runtime::Runtime;
use win_wrap::{msaa::Msaa, uia::automation::UiAutomation};

const CONFIG_FILE_NAME: &str = "config.toml";

/// 核心上下文对象，通过此对象可以访问整个读屏框架的API
#[derive(Debug)]
pub(crate) struct Context {
    commander: Arc<Commander>,
    config_manager: Arc<ConfigManager>,
    event_core: Arc<EventCore>,
    gui_provider: Arc<GuiProvider>,
    jab: Arc<Jab>,
    msaa: Arc<Msaa>,
    ia2: Arc<Ia2>,
    work_runtime: &'static Runtime,
    resource_provider: Arc<ResourceProvider>,
    peeper_server: Arc<PeeperServer>,
    performer: Arc<Performer>,
    #[cfg(target_arch = "x86_64")]
    proxy32process: Arc<Proxy32Process>,
    talent_provider: Arc<TalentProvider>,
    task_manager: Arc<TaskManager>,
    terminator: Arc<Terminator>,
    ui_automation: Arc<UiAutomation>,
    ui_navigator: Arc<UiNavigator>,
}

impl Context {
    /**
     * 创建一个框架上下文环境。
     * */
    pub(crate) fn new(work_runtime: &'static Runtime, terminator: Arc<Terminator>) -> Self {
        // 创建一个指挥官，用于下发操作命令
        let commander = Commander::new();

        // 配置管理器
        let path = get_rigela_program_directory().join(CONFIG_FILE_NAME);
        let config_manager = ConfigManager::new(path);

        // 创建表演者对象（用于把各种信息转换成用户可以感知的形式，例如语音、音效等）
        let performer = Performer::new();

        // MSAA(Microsoft Active Accessibility，辅助功能）接口
        let msaa = Msaa::new();

        // IA2（扩展了MSAA)
        let ia2 = Ia2::new();

        // JAB (Java Access Bridge，无障碍访问桥)
        let jab = Jab::new();

        // 进程亏叹气服务
        let peeper_server = PeeperServer::new(work_runtime);

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
        let event_core = EventCore::new();

        // 窗口浏览器
        let form_browser = UiNavigator::new();

        // Gui 窗口界面管理器
        let window_manager = GuiProvider::new();

        Self {
            commander: commander.into(),
            config_manager: config_manager.into(),
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
            terminator,
            ui_automation: ui_automation.into(),
            work_runtime,
            event_core: event_core.into(),
            ui_navigator: Arc::new(form_browser),
            gui_provider: window_manager.into(),
        }
    }

    /**
     * 把上下文对象应用于每一个组件。
     * */
    pub(crate) fn apply(self: &Arc<Self>) {
        let ctx = Arc::downgrade(self);
        // 初始化命令管理器
        self.commander.apply(ctx.clone());

        // 加载配置文件
        self.config_manager.apply();

        // 启动表演者
        let performer = self.performer.clone();
        self.work_runtime.spawn(async move {
            performer.apply(ctx).await;
            info!("The performer is ready.");
        });

        // 初始化GUI窗口界面
        self.gui_provider.apply(Arc::downgrade(self));
    }

    /**
     * 清理环境。
     * */
    pub(crate) fn dispose(&self) {
        self.event_core.shutdown();
        self.jab.remove_all_listeners();
        self.msaa.remove_all_listeners();
        self.ia2.remove_all_listeners();
        self.ui_automation.remove_all_event_listeners();
        self.config_manager.save_config();

        // 退出Gui界面
        self.gui_provider.dispose();
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            commander: self.commander.clone(),
            config_manager: self.config_manager.clone(),
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
            work_runtime: self.work_runtime,
            event_core: self.event_core.clone(),
            ui_navigator: self.ui_navigator.clone(),
            gui_provider: self.gui_provider.clone(),
        }
    }
}

/// 用于访问Context中的私有字段
pub(crate) trait ContextAccessor {
    /// 获取指挥官对象
    fn get_commander(&self) -> &Commander;
    /// 获取配置管理器对象
    fn get_config_manager(&self) -> &ConfigManager;
    /// 获取事件中心对象
    fn get_event_core(&self) -> &EventCore;
    /// 获取GUI提供者对象
    fn get_gui_provider(&self) -> &GuiProvider;
    /// 获取Java Access Bridge对象
    fn get_jab(&self) -> &Jab;
    /// 获取Microsoft Active Accessibility对象
    fn get_msaa(&self) -> &Msaa;
    /// 获取IAccessible2管理器对象
    fn get_ia2(&self) -> &Ia2;
    /// 获取亏叹气服务端对象
    fn get_peeper_server(&self) -> &PeeperServer;
    /// 获取表演者对象
    fn get_performer(&self) -> &Performer;
    /// 获取32位的代理进程对象
    #[cfg(target_arch = "x86_64")]
    fn get_proxy32process(&self) -> &Proxy32Process;
    /// 获取资源提供者对象
    fn get_resource_provider(&self) -> &ResourceProvider;
    /// 获取能力提供者对象
    fn get_talent_provider(&self) -> &TalentProvider;
    /// 获取任务管理器对象
    fn get_task_manager(&self) -> &TaskManager;
    /// 获取终结者对象
    fn get_terminator(&self) -> &Terminator;
    /// 获取Ui自动化对象
    fn get_ui_automation(&self) -> &UiAutomation;
    /// 获取UI导航器对象
    fn get_ui_navigator(&self) -> &UiNavigator;
    /// 获取工作线程运行时对象
    fn get_work_runtime(&self) -> &Runtime;
}

impl ContextAccessor for Weak<Context> {
    fn get_commander(&self) -> &Commander {
        unsafe { &*self.as_ptr() }.commander.as_ref()
    }

    fn get_config_manager(&self) -> &ConfigManager {
        unsafe { &*self.as_ptr() }.config_manager.as_ref()
    }

    fn get_event_core(&self) -> &EventCore {
        unsafe { &*self.as_ptr() }.event_core.as_ref()
    }

    fn get_gui_provider(&self) -> &GuiProvider {
        unsafe { &*self.as_ptr() }.gui_provider.as_ref()
    }

    fn get_jab(&self) -> &Jab {
        unsafe { &*self.as_ptr() }.jab.as_ref()
    }

    fn get_msaa(&self) -> &Msaa {
        unsafe { &*self.as_ptr() }.msaa.as_ref()
    }

    fn get_ia2(&self) -> &Ia2 {
        unsafe { &*self.as_ptr() }.ia2.as_ref()
    }

    fn get_peeper_server(&self) -> &PeeperServer {
        unsafe { &*self.as_ptr() }.peeper_server.as_ref()
    }

    fn get_performer(&self) -> &Performer {
        unsafe { &*self.as_ptr() }.performer.as_ref()
    }

    #[cfg(target_arch = "x86_64")]
    fn get_proxy32process(&self) -> &Proxy32Process {
        unsafe { &*self.as_ptr() }.proxy32process.as_ref()
    }

    fn get_resource_provider(&self) -> &ResourceProvider {
        unsafe { &*self.as_ptr() }.resource_provider.as_ref()
    }

    fn get_talent_provider(&self) -> &TalentProvider {
        unsafe { &*self.as_ptr() }.talent_provider.as_ref()
    }

    fn get_task_manager(&self) -> &TaskManager {
        unsafe { &*self.as_ptr() }.task_manager.as_ref()
    }

    fn get_terminator(&self) -> &Terminator {
        unsafe { &*self.as_ptr() }.terminator.as_ref()
    }

    fn get_ui_automation(&self) -> &UiAutomation {
        unsafe { &*self.as_ptr() }.ui_automation.as_ref()
    }

    fn get_ui_navigator(&self) -> &UiNavigator {
        unsafe { &*self.as_ptr() }.ui_navigator.as_ref()
    }

    fn get_work_runtime(&self) -> &Runtime {
        unsafe { &*self.as_ptr() }.work_runtime
    }
}

impl ContextAccessor for Arc<Context> {
    fn get_commander(&self) -> &Commander {
        self.commander.as_ref()
    }

    fn get_config_manager(&self) -> &ConfigManager {
        self.config_manager.as_ref()
    }

    fn get_event_core(&self) -> &EventCore {
        self.event_core.as_ref()
    }

    fn get_gui_provider(&self) -> &GuiProvider {
        self.gui_provider.as_ref()
    }

    fn get_jab(&self) -> &Jab {
        self.jab.as_ref()
    }

    fn get_msaa(&self) -> &Msaa {
        self.msaa.as_ref()
    }

    fn get_ia2(&self) -> &Ia2 {
        self.ia2.as_ref()
    }

    fn get_peeper_server(&self) -> &PeeperServer {
        self.peeper_server.as_ref()
    }

    fn get_performer(&self) -> &Performer {
        self.performer.as_ref()
    }

    #[cfg(target_arch = "x86_64")]
    fn get_proxy32process(&self) -> &Proxy32Process {
        self.proxy32process.as_ref()
    }

    fn get_resource_provider(&self) -> &ResourceProvider {
        self.resource_provider.as_ref()
    }

    fn get_talent_provider(&self) -> &TalentProvider {
        self.talent_provider.as_ref()
    }

    fn get_task_manager(&self) -> &TaskManager {
        self.task_manager.as_ref()
    }

    fn get_terminator(&self) -> &Terminator {
        self.terminator.as_ref()
    }

    fn get_ui_automation(&self) -> &UiAutomation {
        self.ui_automation.as_ref()
    }

    fn get_ui_navigator(&self) -> &UiNavigator {
        self.ui_navigator.as_ref()
    }

    fn get_work_runtime(&self) -> &Runtime {
        self.work_runtime
    }
}
