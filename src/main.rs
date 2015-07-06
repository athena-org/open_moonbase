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

extern crate jamkit;
extern crate rand;

use rand::{Rng};

fn main() {
    let mut graphics = jamkit::Graphics::init();

    let map = Map::new(&graphics, "assets/tiles.png");

    'main: loop {
        for event in graphics.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                _ => {}
            }
        }

        let mut frame = jamkit::Frame::start(&graphics);

        map.draw(&mut frame);

        frame.finish();
    }
}

struct Map {
    tileset: jamkit::Texture,
    size: (u32, u32),
    data: Vec<u32>
}

impl Map {
    fn new(graphics: &jamkit::Graphics, tileset: &str) -> Map {
        let width = 50; //256
        let height = 50;
        let mut rng = rand::thread_rng();

        Map {
            tileset: jamkit::Texture::load(&graphics, tileset),
            size: (width, height),
            data: (0u32..width*height).map(|_| rng.gen::<u32>() % 2).collect()
        }
    }

    fn draw(&self, frame: &mut jamkit::Frame) {
        for tile in 0..self.data.len() {
            let (x, y) = (
                tile as i32 / self.size.0 as i32 * 64,
                tile as i32 % self.size.1 as i32 * 64);

            let dest = [x, y, x + 64, y + 64];
            let src = if self.data[tile] == 0 {
                [0, 0, 32, 32]
            } else {
                [32, 0, 64, 32]
            };

            frame.draw(&self.tileset, Some(src), dest);
        }
    }
}
