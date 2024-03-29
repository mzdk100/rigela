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

use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{
        Mutex,
        RwLock,
    },
    time::sleep,
};
use win_wrap::message::{change_window_message_filter, MSGFLT_ADD, pump_waiting_messages, WM_COPYDATA, WM_USER};

#[derive(Clone)]
pub(crate) struct Terminator(
    Arc<Mutex<Vec<Box<dyn Fn() + Sync + Send + 'static>>>>,
    Arc<RwLock<bool>>,
);

impl Terminator {
    /**
     * 创建终结者对象，可以异步等待退出信号。
     * */
    pub(crate) fn new() -> Self {
        let listeners = Arc::new(Mutex::new(vec![]));
        Self(listeners, RwLock::new(false).into())
    }

    /**
     * * 添加一个监听器，当正在退出主程序时，发出通知。
     * `func` 一个监听函数。
     * */
    pub(crate) async fn add_exiting_listener(&self, func: impl Fn() + Sync + Send + 'static) {
        self.0.lock().await.push(Box::new(func))
    }

    /**
     * 发送退出信号。
     * */
    pub(crate) async fn exit(&self) {
        *self.1.write().await = true;
    }

    pub(crate) async fn wait(&self) {
        // 某些进程的WM_COPYDATA和大于WM_USER的消息可能会因为权限无法接收和处理，我们使用change_window_message_filter函数来改变这种行为
        change_window_message_filter(WM_COPYDATA, MSGFLT_ADD);
        for i in (WM_USER + 1)..0xffff {
            change_window_message_filter(i, MSGFLT_ADD);
        }

        // 异步的线程循环
        loop {
            pump_waiting_messages();
            let lock = self.1.read().await;
            if *lock {
                break;
            }
            drop(lock);
            sleep(Duration::from_millis(10)).await;
        }
        let lock = self.0.lock().await;
        lock.iter().for_each(|f| f());
    }
}

impl Debug for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Terminator")
    }
}
