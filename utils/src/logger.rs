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

use crate::fs::get_rigela_program_directory;
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{
                roll::delete::DeleteRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    init_config,
};

const LOG_FILE_NAME: &str = "run.log";

/**
初始化日志收集器。
`path` 日志文件存放的文件夹路径，是相对于本项目用户目录的路径。
*/
pub fn init_logger(path: Option<&str>) {
    let level = LevelFilter::Info;
    let file_path = get_rigela_program_directory()
        .join("logs")
        .join(path.unwrap_or(LOG_FILE_NAME));

    // 创建一个标准错误日志器
    let stderr = ConsoleAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .target(Target::Stderr)
        .build();

    //输出到文件
    let logfile = RollingFileAppender::builder()
        .build(
            file_path,
            Box::new(CompoundPolicy::new(
                Box::new(SizeTrigger::new(1024 * 1024)), // 超过1MB后滚动
                Box::new(DeleteRoller::new()),
            )),
        )
        .unwrap();

    // 将跟踪级别输出记录到文件中，其中跟踪是默认级别，以编程方式指定的级别记录到stderr。
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // 使用此选项可以在运行时更改日志级别。这意味着您可以更改默认日志级别以进行跟踪，如果您正在尝试调试某个问题，并且需要打开更多日志，则在完成后将其关闭。
    init_config(config).expect("Can't initialize the logger.");
}
