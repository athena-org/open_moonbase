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
extern crate rand;

mod map;
mod player;

fn main() {
    let mut graphics = jamkit::Graphics::init("Open Moonbase", 1280, 720);

    let map = map::Map::new(&graphics, "assets/tiles.png");
    let player = player::Player::new(&graphics, "assets/player.png");

    let mut timer = jamkit::utils::DeterminismTimer::at_interval(1000);
    'main: loop {
        for event in graphics.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                _ => {}
            }
        }

        // Update everything
        timer.update(&mut |_| {
            println!("Tick!");
        });

        // Render everything
        let mut frame = jamkit::Frame::start(&graphics);
        map.draw(&mut frame);
        player.draw(&mut frame);
        frame.finish();
    }
}
