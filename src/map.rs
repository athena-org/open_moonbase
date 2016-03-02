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

use cgmath::Vector2;
use jamkit;
use rand;
use rand::{Rng};

pub struct Map {
    tileset: jamkit::Texture,
    size: Vector2<i32>,
    data: Vec<u32>
}

impl Map {
    pub fn new(graphics: &jamkit::Graphics, tileset: &str) -> Map {
        let size = Vector2::new(256, 256);
        let mut rng = rand::thread_rng();

        Map {
            tileset: jamkit::Texture::load(&graphics, tileset),
            size: size,
            data: (0i32..size.x*size.y).map(|_| rng.gen::<u32>() % 2).collect()
        }
    }

    pub fn draw(&self, frame: &mut jamkit::Frame) {
        let frame_dimensions = u32_to_i32(frame.get_dimensions());
        let mut data = Vec::<jamkit::DrawData>::new();

        for tile in 0..self.data.len() {
            // Convert the tile number to an actual position
            let (x, y) = (
                tile as i32 / self.size.x * 64,
                tile as i32 % self.size.y * 64);
            let dest = [x, y, x + 64, y + 64];

            // Make sure the destination is within the screen before we continue
            if dest[2] < 0 || dest[3] < 0 ||
               dest[0] > frame_dimensions.0 || dest[1] > frame_dimensions.1 {
                continue;
            }

            // Get the location in the tilemap the tiles are in
            let src = if self.data[tile] == 0 {
                [32, 0, 64, 32]
            } else {
                [64, 0, 96, 32]
            };

            data.push(jamkit::DrawData{source: Some(src), destination: dest});
        }

        frame.draw_many(&self.tileset, &data);
    }
}

fn u32_to_i32(val: (u32, u32)) -> (i32, i32) {
    (val.0 as i32, val.1 as i32)
}
