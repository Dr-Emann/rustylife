// Copyright (C) 2016 Morgan Kalvin Nrykkxyyyn
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
use std::mem;

use rand::Rng;

pub struct GameState {
    width: u32,
    new_board: Vec<bool>,
    board: Vec<bool>,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> GameState {
        let board = vec![false; width as usize * height as usize];
        GameState {
            width: width,
            board: board.clone(),
            new_board: board,
        }
    }

    pub fn randomize<R>(&mut self, rng: &mut R)
        where R: Rng
    {
        for item in self.board.iter_mut() {
            *item = rng.gen_weighted_bool(7);
        }
    }

    pub fn update(&mut self) {
        for y in 0..self.height() as i32 {
            for x in 0..self.width() as i32 {
                let idx = self.idx(x, y);
                self.new_board[idx] = self.step_cell(x, y);
            }
        }

        mem::swap(&mut self.board, &mut self.new_board);
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        (self.board.len() / self.width() as usize) as u32
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> bool {
        let idx = self.idx(x, y);
        self.board[idx]
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, value: bool) {
        let idx = self.idx(x, y);
        self.board[idx] = value;
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        // TODO: Keep some info
        let new_size = new_width as usize * new_height as usize;
        self.board.resize(new_size, false);
        self.new_board.resize(new_size, false);
    }

    #[inline]
    fn idx(&self, mut x: i32, mut y: i32) -> usize {
        x = x % self.width() as i32;
        if x < 0 {
            x += self.width() as i32;
        }

        y = y % self.height() as i32;
        if y < 0 {
            y += self.height() as i32;
        }

        y as usize * self.width() as usize + x as usize
    }

    fn living_neighbors(&self, x: i32, y: i32) -> u32 {
        let offsets = [(-1, -1), (0, -1), (1, -1),
                       (-1,  0),          (1,  0),
                       (-1,  1), (0,  1), (1,  1)];
        offsets.iter()
               .cloned()
               .filter(|&(dx, dy)| self.get(x + dx, y + dy))
               .count() as u32
    }

    fn step_cell(&self, x: i32, y: i32) -> bool {
        match self.living_neighbors(x, y) {
            0|1 => false,
            3 => true,
            n if n > 3 => false,
            _ => self.get(x, y),
        }
    }
}
