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

use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Clone, Debug)]
pub struct Terminator(Sender<()>);

pub struct TerminationWaiter(Receiver<()>);

impl Terminator {
    /**
     * 创建终结者对象，可以异步等待退出信号。
     * */
    pub(crate) fn new() -> (Self, TerminationWaiter) {
        let (tx, rx) = channel(1);
        (Self(tx), TerminationWaiter(rx))
    }

    /* 发送退出信号。 */
    pub(crate) async fn exit(&self) {
        self.0.send(()).await.unwrap();
    }
}

impl TerminationWaiter {
    /**
     * 等待退出信号。
     * */
    pub(crate) async fn wait(&mut self) {
        self.0.recv().await;
    }
}
