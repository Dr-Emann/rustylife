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

use std::collections::HashMap;

use pancurses;
use rand::Rng;

const LIVE: char = '#';
const DEAD: char = '.';
const XLEN: i32 = 100;
const YLEN: i32 = 50;

// update_map and draw_screen need to be able to catch errors
// in such a way that curses will exit gracefully.

pub fn update_map(map: HashMap<(i32, i32), bool>) -> HashMap<(i32, i32), bool> {
    // ...
}

pub fn create_map() -> HashMap<(i32, i32), bool> {
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    let mut rng = rand::thread_rng();

    for x in 0..XLEN {
        for y in 0..YLEN {
            map.insert((x, y), rng.gen());
        }
    }

    map
}

// Not sure if this is going to work.
pub fn draw_screen(screen: &pancurses::Window, map: &HashMap<(i32, i32), bool>) {
    // ...
}
