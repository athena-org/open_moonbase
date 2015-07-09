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

use cgmath::{Vector2};
use jamkit;

pub struct Player {
    position: Vector2<i32>,
    texture: jamkit::Texture
}

impl Player {
    pub fn new(graphics: &jamkit::Graphics, texture: &str) -> Player {
        Player {
            position: Vector2::new(500, 500),
            texture: jamkit::Texture::load(&graphics, texture)
        }
    }

    pub fn draw(&self, frame: &mut jamkit::Frame) {
        let pos = self.position;
        frame.draw(&self.texture, None, [pos.x - 32, pos.y - 32, pos.x + 32, pos.y + 32]);
    }
}
