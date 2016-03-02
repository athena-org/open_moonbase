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

use cgmath::{Vector2, EuclideanVector, Vector};
use jamkit;
use jamkit::{Key};
use jamkit::utils::{InputState};

pub struct Player {
    position: Vector2<f32>,
    texture: jamkit::Texture
}

impl Player {
    pub fn new(graphics: &jamkit::Graphics, texture: &str) -> Player {
        Player {
            position: Vector2::new(500.0, 500.0),
            texture: jamkit::Texture::load(&graphics, texture)
        }
    }

    pub fn update(&mut self, input: &InputState) {
        let mut direction = Vector2::new(0.0, 0.0);
        let speed = Vector2::new(2.0, 2.0);

        if input[Key::A].is_pressed() { direction.x -= 1.0; }
        if input[Key::D].is_pressed() { direction.x += 1.0; }
        if input[Key::W].is_pressed() { direction.y -= 1.0; }
        if input[Key::S].is_pressed() { direction.y += 1.0; }

        if direction != Vector2::zero() {
            direction = direction.normalize();
        }

        self.position = self.position + (direction * speed);
    }

    pub fn draw(&self, frame: &mut jamkit::Frame) {
        let pos = Vector2::new(self.position.x as i32, self.position.y as i32);
        frame.draw(&self.texture, None, [pos.x - 32, pos.y - 32, pos.x + 32, pos.y + 32]);
    }
}
