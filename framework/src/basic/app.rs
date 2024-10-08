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

use crate::basic::{
    component::{Comp, Component},
    plugin::Plugin,
    system::{System, ToSystem},
};
use parking_lot::Mutex;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    panic::Location,
};

/// APP代理组件
pub struct AppProxy {
    /// 退出标志
    exiting: bool,
}

impl AppProxy {
    /// 通知程序结束运行
    pub(crate) fn exit(&mut self) {
        self.exiting = true;
    }
}

impl Component for AppProxy {}

/// 应用程序运行器，使用组件-系统(Component-System)架构模式。
pub struct AppRunner {
    /// 存储应用程序中的组件。
    components: HashMap<TypeId, Mutex<Box<dyn Any>>>,
    /// 存储应用程序的插件
    plugins: HashMap<TypeId, Box<dyn Plugin>>,
    /// 存储应用程序中的系统。
    systems: Vec<System>,
}

impl AppRunner {
    /// 创建一个新的应用程序运行器实例。
    pub fn new() -> Self {
        Self {
            components: Default::default(),
            plugins: Default::default(),
            systems: Default::default(),
        }
    }

    /// 向应用程序添加一个新的组件。
    ///
    /// # 参数
    /// * `component` - 要添加的组件。
    ///
    /// # 返回值
    /// 返回 `self` 的可变引用，以便可以链式调用。
    pub fn add_component<C: Component + 'static>(&mut self, component: C) -> &mut Self {
        self.components
            .insert(component.type_id(), Mutex::new(Box::new(component)));
        self
    }

    /// 获取指定类型的组件。
    ///
    /// # 参数
    /// * `C` - 组件的类型。
    ///
    /// # 返回值
    /// 返回一个封装了组件的 `Comp` 对象。
    ///
    /// # 错误
    /// 如果找不到指定类型的组件，会引发 panic。
    #[track_caller]
    pub(crate) fn get_component<C: Component + 'static>(&self) -> Comp<C> {
        let key = TypeId::of::<C>();
        if let Some(c) = self.components.get(&key) {
            return Comp::new(c.lock());
        }
        let location = Location::caller();
        panic!(
            "Component not found. File {} {}:{}. {:?}",
            location.file(),
            location.line(),
            location.column(),
            TypeId::of::<C>()
        );
    }

    /// 从应用程序中移除指定类型的组件。
    ///
    /// # 参数
    /// * `C` - 组件的类型。
    pub fn remove_component<C: Component + 'static>(&mut self) -> &mut Self {
        self.components.remove(&TypeId::of::<C>());
        self
    }

    /// 向应用程序添加一个新的 插件。
    ///
    /// # 参数
    /// * `plugin` - 要添加的插件。
    ///
    /// # 返回值
    /// 返回 `self` 的可变引用，以便可以链式调用。
    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        plugin.init(self);
        self.plugins.insert(plugin.type_id(), Box::new(plugin));
        self
    }

    /// 从应用程序中移除指定类型的插件。
    ///
    /// # 参数
    /// * `P` - 插件的类型。
    pub fn remove_plugin<P: Plugin + 'static>(&mut self) -> &mut Self {
        if let Some(p) = self.plugins.remove(&TypeId::of::<P>()) {
            p.uninit(self);
        }
        self
    }

    /// 向应用程序添加一个新的系统。
    ///
    /// # 参数
    /// * `system` - 要添加的系统。
    ///
    /// # 返回值
    /// 返回 `self` 的可变引用，以便可以链式调用。
    pub fn add_system<P>(&mut self, system: impl ToSystem<P> + 'static) -> &mut Self {
        self.systems.push(system.to_system());
        self
    }

    /// 从应用程序中移除指定系统。
    ///
    /// # 参数
    /// * `system` - 系统。
    pub fn remove_system<P>(&mut self, system: impl ToSystem<P> + 'static) -> &mut Self {
        let type_id = system.type_id();
        self.systems.retain(|s| s.type_id() != type_id);
        self
    }

    /// 启动应用程序的主循环。
    pub fn run(&mut self) {
        self.add_component(AppProxy { exiting: false });
        loop {
            for i in &self.systems {
                i.call(self);
            }

            if self.get_component::<AppProxy>().exiting {
                break;
            }

            #[cfg(debug_assertions)]
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
