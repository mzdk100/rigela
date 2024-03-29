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

use log::error;
use serde::{Deserialize, Serialize};
use serde_json_bytes::serde_json::{from_slice, to_vec};
use std::fmt::{Display, Formatter};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::windows::named_pipe::{ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions},
    time::{sleep, Duration},
};

/**
 * 连接到一个管道。
 * `pipe_name` 管道名称。
 * */
pub async fn client_connect<T>(pipe_name: &str) -> PipeStream<T, NamedPipeClient>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    // 使用循环方法连接管道，因为可能在连接的时候管道还没创建完毕
    let client = loop {
        // 推迟一秒连接，尽量确保管道创建完毕
        sleep(Duration::from_millis(1000)).await;

        match ClientOptions::new().open(pipe_name) {
            Ok(x) => break x,
            Err(e) => {
                error!("Can't open the named pipe ({}). {}", pipe_name, e);
                continue;
            }
        }
    };
    PipeStream::new(client)
}

/**
 * 创建一个管道服务器，并等待一个客户端连接。
 * `pipe_name` 管道名称。
 * */
pub async fn server_run<T>(pipe_name: &str) -> PipeStream<T, NamedPipeServer>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let server = ServerOptions::new().create(pipe_name).unwrap();
    server.connect().await.unwrap();
    PipeStream::new(server)
}

#[derive(Debug)]
pub struct PipeStream<R, T>
where
    R: for<'de> Deserialize<'de> + Serialize,
    T: AsyncRead + AsyncWrite,
{
    _packet: Option<R>,
    reader: BufReader<T>,
}

impl<R, T> PipeStream<R, T>
where
    R: for<'de> Deserialize<'de> + Serialize,
    T: AsyncRead + AsyncWrite + Unpin,
{
    /**
     * 创建一个管道的流，用于发送和接收数据。
     * 其中传输的数据是实现了Deserialize 和 Serialize接口的struct。
     * */
    pub fn new(stream: T) -> Self {
        let reader = BufReader::new(stream);
        Self {
            _packet: None,
            reader,
        }
    }

    /**
     * 接收一个数据包。
     * */
    pub async fn recv(&mut self) -> Result<R, PipeStreamError> {
        let mut buf = Vec::new();
        if let Ok(x) = self.reader.read_until(b'\n', &mut buf).await {
            if x < 1 {
                return Err(PipeStreamError::ReadEof);
            }
        };
        let r = from_slice(&buf);
        if let Err(e) = r {
            return Err(PipeStreamError::DecodeError(e.to_string()));
        }
        Ok(r.unwrap())
    }

    /**
     * 发送一个数据包。
     * `packet` 实现了序列化接口的数据。
     * */
    pub async fn send(&mut self, packet: &R) -> Result<(), PipeStreamError> {
        let mut data = to_vec(&packet).unwrap();
        data.push(b'\n');
        if let Err(e) = self.reader.write_all(&data).await {
            Err(PipeStreamError::WriteError(format!(
                "Can't send the data. {}",
                e
            )))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum PipeStreamError {
    ReadEof,
    WriteError(String),
    DecodeError(String),
}

impl Display for PipeStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PipeStreamError::ReadEof => "Read eof.".to_string(),
            PipeStreamError::DecodeError(e) => e.to_string(),
            PipeStreamError::WriteError(e) => e.to_string(),
        };
        write!(f, "{}", msg)
    }
}
