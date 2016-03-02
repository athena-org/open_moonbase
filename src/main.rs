// Copyright 2015 The Athena Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cgmath;
extern crate jamkit;
extern crate num;
extern crate rand;

mod map;
mod player;
mod world;

use jamkit::Graphics;
use jamkit::utils::{InputState, TickTimer};

fn main() {
    let mut graphics = Graphics::init("Open Moonbase", 1280, 800);
    let mut input = InputState::new();

    let mut world = world::World::new(&graphics);

    let mut timer = TickTimer::at_interval(10);
    'main: loop {
        for event in graphics.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                jamkit::Event::KeyboardInput(state, key) =>
                    input.process_keyboard(state, key),
                _ => ()
            }
        }

        // Update everything
        timer.update(&mut |_| {
            world.update(&input);
        });

        // Render everything
        let mut frame = jamkit::Frame::start(&graphics);
        world.draw(&mut frame);
        frame.finish();
    }
}
