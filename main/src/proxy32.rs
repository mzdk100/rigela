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

use rigela_proxy32::client::Proxy32Client;
use tokio::process::Child;
use tokio::sync::RwLock;

#[cfg(target_arch = "x86_64")]
const PIPE_NAME: &str = r"\\.\PIPE\PROXY32";

pub(crate) struct Proxy32 {
    #[allow(dead_code)]
    process: RwLock<Option<Child>>,
    client: RwLock<Option<Proxy32Client>>,
}

impl Proxy32 {
    /**
     * 创建一个proxy32模块实例。
     * */
    pub(crate) fn new() -> Self {
        Self {
            process: None.into(),
            client: None.into(),
        }
    }

    /**
     * 创建进程。
     * */
    #[cfg(target_arch = "x86_64")]
    pub(crate) async fn spawn(&self) -> &Self {
        use log::error;
        use rigela_utils::{get_program_directory, write_file};
        use std::time::Duration;
        use tokio::{process::Command, time::sleep};

        // 获取proxy32.exe的二进制数据并写入到用户目录中，原理是在编译时把proxy32的数据使用include_bytes!内嵌到64位的主程序内部，在运行时释放到磁盘。
        // 注意：这里使用条件编译的方法，确保include_bytes!仅出现一次，不能使用if语句，那样会多次包含bytes，main.exe的大小会成倍增长。
        #[cfg(not(debug_assertions))]
        let proxy32_bin =
            include_bytes!("../../target/i686-pc-windows-msvc/release/rigela-proxy32.exe");
        #[cfg(debug_assertions)]
        let proxy32_bin =
            include_bytes!("../../target/i686-pc-windows-msvc/debug/rigela-proxy32.exe");
        let proxy32_path = get_program_directory().join("proxy32.exe");
        if let Err(e) = write_file(&proxy32_path, proxy32_bin).await {
            error!("{}", e);
        }

        // 启动32位的代理模块。
        let cmd = loop {
            // 因为proxy32.exe刚刚释放到磁盘，很可能被微软杀毒锁定，这时候启动会失败（另一个程序正在使用此文件，进程无法访问。）
            sleep(Duration::from_millis(1000)).await;
            // 1秒之后再尝试启动
            if let Ok(x) = Command::new(&proxy32_path).args([PIPE_NAME]).spawn() {
                break x;
            }
        };

        {
            let mut process = self.process.write().await;
            *process = Some(cmd).into();
        }
        let proxy32_client = Proxy32Client::new(PIPE_NAME).await;
        let mut client = self.client.write().await;
        *client = Some(proxy32_client);
        self
    }

    /**
     * 创建进程。
     * */
    #[cfg(target_arch = "x86")]
    pub(crate) async fn spawn(&self) -> &Self {
        // 如果主程序本身就是32位，则无需执行此操作（proxy32模块没有用武之地）
        use log::info;
        info!("Loaded proxy32.");
        self
    }

    /**
     * 杀死进程。
     * */
    pub(crate) async fn kill(&self) -> &Self {
        let mut client = self.client.write().await;
        if let Some(c) = client.as_mut() {
            c.quit().await;
        }
        let mut process = self.process.write().await;
        if let Some(p) = process.as_mut() {
            p.kill().await.unwrap_or(());
        }
        self
    }

    /**
     * 等待进程结束。
     * */
    pub(crate) async fn wait(&self) {
        let mut process = self.process.write().await;
        if let Some(x) = process.as_mut() {
            x.wait().await.unwrap();
        }
    }
}
