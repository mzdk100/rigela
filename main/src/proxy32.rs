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

#[cfg(target_arch = "x86_64")]
use rigela_proxy32::client::Proxy32Client;
#[cfg(target_arch = "x86_64")]
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
#[cfg(target_arch = "x86_64")]
use tokio::{
    process::Child,
    sync::{OnceCell, RwLock},
};

#[cfg(target_arch = "x86_64")]
const PIPE_NAME: &str = r"\\.\PIPE\RIGELA_PROXY32";

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
pub(crate) struct Proxy32 {
    process: RwLock<Option<Child>>,
    client: OnceCell<Arc<Proxy32Client>>,
}

#[cfg(target_arch = "x86_64")]
impl Proxy32 {
    /**
     * 创建一个proxy32模块实例。
     * */
    pub(crate) fn new() -> Self {
        Self {
            process: None.into(),
            client: OnceCell::new(),
        }
    }

    /**
     * 创建进程。
     * */
    #[cfg(target_arch = "x86_64")]
    pub(crate) async fn spawn(&self) -> &Self {
        use log::{error, info};
        use rigela_utils::{get_file_modified_duration, get_program_directory, write_file};
        use std::time::Duration;
        use tokio::{process::Command, time::sleep};

        // 获取proxy32.exe的二进制数据并写入到用户目录中，原理是在编译时把proxy32的数据使用include_bytes!内嵌到64位的主程序内部，在运行时释放到磁盘。
        // 注意：这里使用条件编译的方法，确保include_bytes!仅出现一次，不能使用if语句，那样会多次包含bytes，main.exe的大小会成倍增长。
        #[cfg(not(debug_assertions))]
            let (proxy32_bin, is_debug) =
            (include_bytes!("../../target/i686-pc-windows-msvc/release/rigela-proxy32.exe"), false);
        #[cfg(debug_assertions)]
            let (proxy32_bin, is_debug) =
            (include_bytes!("../../target/i686-pc-windows-msvc/debug/rigela-proxy32.exe"), true);
        let proxy32_path = get_program_directory().join("libs/proxy32.exe");
        if is_debug || get_file_modified_duration(&proxy32_path).await > 3600 * 6 {
            // 如果文件修改时间超出6个小时才重新写文件，加快启动速度
            if let Err(e) = write_file(&proxy32_path, proxy32_bin).await {
                error!("{}", e);
            }
        }

        // 启动32位的代理模块。
        let cmd = loop {
            if let Ok(x) = Command::new(&proxy32_path).args([PIPE_NAME]).spawn() {
                break x;
            }
            // 因为proxy32.exe刚刚释放到磁盘，很可能被微软杀毒锁定，这时候启动会失败（另一个程序正在使用此文件，进程无法访问。），1秒之后再尝试启动
            sleep(Duration::from_millis(1000)).await;
        };
        info!("The process is ready.");

        {
            let mut process = self.process.write().await;
            *process = Some(cmd).into();
        }
        if self
            .client
            .set(Proxy32Client::new(PIPE_NAME).await.into())
            .is_err()
        {
            error!("Can't set client field of the proxy32.");
        }
        self
    }

    /**
     * 创建进程。
     * */
    #[cfg(target_arch = "x86")]
    pub(crate) async fn spawn(&self) -> &Self {
        // 如果主程序本身就是32位，则无需执行此操作（proxy32模块没有用武之地）
        info!("Loaded proxy32.");
        self
    }

    /**
     * 杀死进程。
     * */
    pub(crate) async fn kill(&self) -> &Self {
        use log::{error, info};

        if let Some(x) = self.client.get() {
            x.quit().await;
        }
        let mut process = self.process.write().await;
        if let Some(p) = process.as_mut() {
            match p.wait().await {
                Ok(s) => {
                    info!(
                        "The process has exited successfully. Exit code is {}.",
                        s.code().unwrap()
                    );
                }
                Err(e) => {
                    error!("The process has exited with errors. {}", e);
                }
            };
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

#[cfg(target_arch = "x86_64")]
impl Future for &Proxy32 {
    type Output = Arc<Proxy32Client>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.client.get() {
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Some(c) => Poll::Ready(c.clone()),
        }
    }
}
