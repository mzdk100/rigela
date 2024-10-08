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

//! 演示如何定义自己的组件和系统

use rigela_framework::basic::{AppRunner, Comp, Component};

#[derive(Debug)]
struct Com1(u8);

impl Component for Com1 {}

#[derive(Debug)]
struct Com2;

impl Component for Com2 {}

fn a(mut a: Comp<Com1>) {
    println!("{:?}", a);
    if a.0 == 2 {
        a.0 = 3;
    } else {
        *a = Com1(2);
    }
}

fn b(a: Comp<Com1>, b: Comp<Com2>) {
    println!("{:?},{:?}", a, b);
}

fn main() {
    AppRunner::new()
        .add_component(Com1(1))
        .add_component(Com2)
        .add_system(a)
        .add_system(b)
        .run();
}
